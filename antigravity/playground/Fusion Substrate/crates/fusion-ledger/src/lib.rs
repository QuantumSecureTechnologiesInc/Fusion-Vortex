use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// A single entry in the execution ledger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerEntry {
    /// Sequential entry number starting from 0
    pub sequence: u64,
    /// Entry payload (typically an MCP request or similar)
    pub payload: serde_json::Value,
}

/// Ledger errors
#[derive(Debug)]
pub enum LedgerError {
    /// I/O error during ledger operations
    IoError(std::io::Error),
    /// Serialization/deserialization error
    SerdeError(serde_json::Error),
    /// Sequence number mismatch
    SequenceMismatch { expected: u64, actual: u64 },
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerError::IoError(e) => write!(f, "Ledger I/O error: {}", e),
            LedgerError::SerdeError(e) => write!(f, "Ledger serialisation error: {}", e),
            LedgerError::SequenceMismatch { expected, actual } => {
                write!(
                    f,
                    "Sequence mismatch: expected {}, got {}",
                    expected, actual
                )
            }
        }
    }
}

impl std::error::Error for LedgerError {}

impl From<std::io::Error> for LedgerError {
    fn from(e: std::io::Error) -> Self {
        LedgerError::IoError(e)
    }
}

impl From<serde_json::Error> for LedgerError {
    fn from(e: serde_json::Error) -> Self {
        LedgerError::SerdeError(e)
    }
}

/// Append-only ledger for deterministic execution replay
///
/// The ledger is crash-safe: you can kill the process at any point,
/// and on restart, replay() will provide the exact sequence of entries.
///
/// **Key Insight**: Replay is not "debugging" - it IS the execution model.
pub struct Ledger {
    path: String,
}

impl Ledger {
    /// Create a new ledger at the specified path
    ///
    /// # Example
    /// ```
    /// use fusion_ledger::Ledger;
    /// let ledger = Ledger::new("execution.log");
    /// ```
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    /// Append an entry to the ledger
    ///
    /// This operation is crash-safe: the entry is written to disk immediately.
    ///
    /// # Errors
    /// Returns `LedgerError` if the file cannot be opened or written to
    pub fn append(&self, entry: &LedgerEntry) -> Result<(), LedgerError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let mut writer = std::io::BufWriter::new(file);
        let line = serde_json::to_string(entry)?;
        writeln!(writer, "{}", line)?;
        writer.flush()?;

        Ok(())
    }

    /// Replay all entries from the ledger
    ///
    /// Returns an empty vector if the ledger file doesn't exist.
    ///
    /// # Errors
    /// Returns `LedgerError` if any entry cannot be read or deserialized
    pub fn replay(&self) -> Result<Vec<LedgerEntry>, LedgerError> {
        if !Path::new(&self.path).exists() {
            return Ok(vec![]);
        }

        let file = OpenOptions::new().read(true).open(&self.path)?;

        let entries: Result<Vec<LedgerEntry>, LedgerError> = BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(idx, line_result)| {
                let line = line_result?;
                let entry: LedgerEntry = serde_json::from_str(&line)?;

                // Verify sequence integrity
                if entry.sequence != idx as u64 {
                    return Err(LedgerError::SequenceMismatch {
                        expected: idx as u64,
                        actual: entry.sequence,
                    });
                }

                Ok(entry)
            })
            .collect();

        entries
    }

    /// Get the current ledger length (number of entries)
    pub fn len(&self) -> Result<u64, LedgerError> {
        Ok(self.replay()?.len() as u64)
    }

    /// Check if the ledger is empty
    pub fn is_empty(&self) -> Result<bool, LedgerError> {
        Ok(self.len()? == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_append_and_replay() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());

        let entry1 = LedgerEntry {
            sequence: 0,
            payload: serde_json::json!({"tool": "test1"}),
        };

        let entry2 = LedgerEntry {
            sequence: 1,
            payload: serde_json::json!({"tool": "test2"}),
        };

        ledger.append(&entry1).unwrap();
        ledger.append(&entry2).unwrap();

        let replayed = ledger.replay().unwrap();
        assert_eq!(replayed.len(), 2);
        assert_eq!(replayed[0], entry1);
        assert_eq!(replayed[1], entry2);
    }

    #[test]
    fn test_empty_ledger() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());

        let replayed = ledger.replay().unwrap();
        assert_eq!(replayed.len(), 0);
        assert!(ledger.is_empty().unwrap());
    }

    #[test]
    fn test_sequence_mismatch_detection() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Manually write an entry with wrong sequence
        let entry = LedgerEntry {
            sequence: 5, // Wrong! Should be 0
            payload: serde_json::json!({"tool": "test"}),
        };

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        writeln!(file, "{}", serde_json::to_string(&entry).unwrap()).unwrap();
        drop(file);

        let ledger = Ledger::new(path);
        let result = ledger.replay();

        assert!(result.is_err());
    }
}
