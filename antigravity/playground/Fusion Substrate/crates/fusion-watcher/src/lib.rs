use notify::{
    Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::time::Instant;

/// Watcher errors
#[derive(Debug, Error)]
pub enum WatcherError {
    #[error("Notify error: {0}")]
    NotifyError(#[from] notify::Error),

    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// File change event
#[derive(Debug, Clone)]
pub struct FileChange {
    /// Path that changed
    pub path: PathBuf,
    /// Type of change
    pub kind: ChangeKind,
    /// Timestamp of the event
    pub timestamp: Instant,
}

/// Type of file system change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// File was created
    Created,
    /// File was modified
    Modified,
    /// File was deleted
    Removed,
    /// Other change type
    Other,
}

impl From<EventKind> for ChangeKind {
    fn from(kind: EventKind) -> Self {
        match kind {
            EventKind::Create(_) => ChangeKind::Created,
            EventKind::Modify(_) => ChangeKind::Modified,
            EventKind::Remove(_) => ChangeKind::Removed,
            _ => ChangeKind::Other,
        }
    }
}

/// Debounced file system watcher
///
/// Monitors files and directories for changes, debouncing rapid successive events
/// to avoid triggering rebuild storms.
pub struct FileWatcher {
    debounce_duration: Duration,
}

impl FileWatcher {
    /// Create a new file watcher with default debounce (100ms)
    pub fn new() -> Self {
        Self {
            debounce_duration: Duration::from_millis(100),
        }
    }

    /// Create a watcher with custom debounce duration
    pub fn with_debounce(debounce_ms: u64) -> Self {
        Self {
            debounce_duration: Duration::from_millis(debounce_ms),
        }
    }

    /// Watch a path and return a receiver for debounced file changes
    ///
    /// # Arguments
    /// * `path` - Path to watch (file or directory)
    /// * `recursive` - Whether to watch subdirectories
    ///
    /// # Returns
    /// A receiver that will get debounced file change events
    pub async fn watch<P: AsRef<Path>>(
        &self,
        path: P,
        recursive: bool,
    ) -> Result<mpsc::UnboundedReceiver<FileChange>, WatcherError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(WatcherError::PathNotFound(path.to_path_buf()));
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Event>();

        // Create the notify watcher
        let mut watcher = RecommendedWatcher::new(
            move |result: NotifyResult<Event>| {
                if let Ok(event) = result {
                    let _ = event_tx.send(event);
                }
            },
            notify::Config::default(),
        )?;

        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher.watch(path, mode)?;

        // Spawn debouncer task
        let debounce_duration = self.debounce_duration;
        tokio::spawn(async move {
            // Keep watcher alive
            let _watcher = watcher;

            let mut last_events: std::collections::HashMap<PathBuf, (ChangeKind, Instant)> =
                std::collections::HashMap::new();
            let mut check_interval = tokio::time::interval(Duration::from_millis(50));

            loop {
                tokio::select! {
                    Some(event) = event_rx.recv() => {
                        // Process new events
                        for path in event.paths {
                            let kind: ChangeKind = event.kind.into();
                            last_events.insert(path, (kind, Instant::now()));
                        }
                    }
                    _ = check_interval.tick() => {
                        // Check for events ready to emit
                        let now = Instant::now();
                        let mut to_emit = vec![];

                        last_events.retain(|path, (kind, timestamp)| {
                            if now.duration_since(*timestamp) >= debounce_duration {
                                to_emit.push(FileChange {
                                    path: path.clone(),
                                    kind: *kind,
                                    timestamp: *timestamp,
                                });
                                false // Remove from map
                            } else {
                                true // Keep in map
                            }
                        });

                        // Emit debounced events
                        for change in to_emit {
                            if tx.send(change).is_err() {
                                // Receiver dropped, exit
                                return;
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    /// Watch multiple paths simultaneously
    pub async fn watch_multiple<P: AsRef<Path>>(
        &self,
        paths: Vec<P>,
        recursive: bool,
    ) -> Result<mpsc::UnboundedReceiver<FileChange>, WatcherError> {
        let (combined_tx, combined_rx) = mpsc::unbounded_channel();

        for path in paths {
            let mut rx = self.watch(path, recursive).await?;
            let tx = combined_tx.clone();

            tokio::spawn(async move {
                while let Some(change) = rx.recv().await {
                    if tx.send(change).is_err() {
                        break;
                    }
                }
            });
        }

        Ok(combined_rx)
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_file_creation_detection() {
        let temp_dir = TempDir::new().unwrap();
        let watcher = FileWatcher::with_debounce(50);

        let mut rx = watcher.watch(temp_dir.path(), false).await.unwrap();

        // Create a file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").unwrap();

        // Wait for debounced event
        tokio::time::timeout(Duration::from_secs(1), async {
            while let Some(change) = rx.recv().await {
                if change.path == test_file && change.kind == ChangeKind::Created {
                    return;
                }
            }
        })
        .await
        .expect("Should detect file creation");
    }

    #[tokio::test]
    async fn test_file_modification_detection() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "initial").unwrap();

        let watcher = FileWatcher::with_debounce(50);
        let mut rx: mpsc::UnboundedReceiver<FileChange> =
            watcher.watch(temp_dir.path(), false).await.unwrap();

        // Small delay to ensure watcher is ready
        sleep(Duration::from_millis(100)).await;

        // Modify the file
        fs::write(&test_file, "modified").unwrap();

        // Wait for event
        tokio::time::timeout(Duration::from_secs(1), async {
            while let Some(change) = rx.recv().await {
                if change.path == test_file && change.kind == ChangeKind::Modified {
                    return;
                }
            }
        })
        .await
        .expect("Should detect file modification");
    }

    #[tokio::test]
    async fn test_debouncing() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "initial").unwrap();

        let watcher = FileWatcher::with_debounce(200); // Longer debounce for this test
        let mut rx: mpsc::UnboundedReceiver<FileChange> =
            watcher.watch(temp_dir.path(), false).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        // Rapid successive writes
        for i in 0..5 {
            fs::write(&test_file, format!("content{}", i)).unwrap();
            sleep(Duration::from_millis(20)).await;
        }

        // Should receive only one debounced event
        let mut event_count = 0;
        let timeout_result = tokio::time::timeout(Duration::from_millis(500), async {
            while let Some(_change) = rx.recv().await {
                event_count += 1;
            }
        })
        .await;

        // Should timeout (receiver still open) and have received 1-2 events max
        assert!(timeout_result.is_err());
        assert!(
            event_count <= 2,
            "Should debounce multiple rapid changes, got {}",
            event_count
        );
    }

    #[tokio::test]
    async fn test_nonexistent_path() {
        let watcher = FileWatcher::new();
        let result = watcher.watch("/nonexistent/path/12345", false).await;
        assert!(matches!(result, Err(WatcherError::PathNotFound(_))));
    }
}
