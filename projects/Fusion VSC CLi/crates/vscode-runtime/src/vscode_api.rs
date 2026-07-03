//! VS Code API bridge implementation

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// VS Code API namespace implementations
pub struct VscodeApi {
    pub workspace: WorkspaceNamespace,
    pub window: WindowNamespace,
    pub commands: CommandsNamespace,
    pub languages: LanguagesNamespace,
}

impl VscodeApi {
    pub fn new() -> Self {
        Self {
            workspace: WorkspaceNamespace::new(),
            window: WindowNamespace::new(),
            commands: CommandsNamespace::new(),
            languages: LanguagesNamespace::new(),
        }
    }
}

/// workspace namespace
pub struct WorkspaceNamespace {
    workspace_folders: Arc<RwLock<Vec<WorkspaceFolder>>>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
    pub index: usize,
}

impl WorkspaceNamespace {
    pub fn new() -> Self {
        Self {
            workspace_folders: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn get_workspace_folders(&self) -> Vec<WorkspaceFolder> {
        self.workspace_folders.read().await.clone()
    }

    pub async fn add_workspace_folder(&self, folder: WorkspaceFolder) {
        self.workspace_folders.write().await.push(folder);
    }

    pub async fn find_files(&self, pattern: &str) -> Result<Vec<String>> {
        // Implement file search using glob patterns
        tracing::debug!("Finding files matching pattern: {}", pattern);
        Ok(Vec::new())
    }
}

/// window namespace
pub struct WindowNamespace {
    active_text_editor: Arc<RwLock<Option<String>>>,
}

impl WindowNamespace {
    pub fn new() -> Self {
        Self {
            active_text_editor: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn show_information_message(&self, message: &str) -> Result<()> {
        tracing::info!("[VS Code] {}", message);
        Ok(())
    }

    pub async fn show_warning_message(&self, message: &str) -> Result<()> {
        tracing::warn!("[VS Code] {}", message);
        Ok(())
    }

    pub async fn show_error_message(&self, message: &str) -> Result<()> {
        tracing::error!("[VS Code] {}", message);
        Ok(())
    }

    pub async fn show_quick_pick(&self, items: Vec<String>) -> Result<Option<String>> {
        // In CLI mode, this would use interactive prompts
        tracing::debug!("Quick pick items: {:?}", items);
        Ok(items.first().cloned())
    }
}

/// commands namespace
pub struct CommandsNamespace {
    registered_commands: Arc<RwLock<HashMap<String, CommandHandler>>>,
}

type CommandHandler = Arc<dyn Fn(Vec<Value>) -> Result<Value> + Send + Sync>;

impl CommandsNamespace {
    pub fn new() -> Self {
        Self {
            registered_commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_command<F>(&self, command: String, handler: F) -> Result<()>
    where
        F: Fn(Vec<Value>) -> Result<Value> + Send + Sync + 'static,
    {
        let mut commands = self.registered_commands.write().await;
        commands.insert(command.clone(), Arc::new(handler));
        tracing::debug!("Registered command: {}", command);
        Ok(())
    }

    pub async fn execute_command(&self, command: &str, args: Vec<Value>) -> Result<Value> {
        let commands = self.registered_commands.read().await;

        if let Some(handler) = commands.get(command) {
            handler(args)
        } else {
            Err(anyhow::anyhow!("Command not found: {}", command))
        }
    }

    pub async fn get_commands(&self) -> Vec<String> {
        self.registered_commands
            .read()
            .await
            .keys()
            .cloned()
            .collect()
    }
}

/// languages namespace
pub struct LanguagesNamespace {
    language_servers: Arc<RwLock<HashMap<String, LanguageServerInfo>>>,
}

#[derive(Debug, Clone)]
pub struct LanguageServerInfo {
    pub language_id: String,
    pub server_name: String,
}

impl LanguagesNamespace {
    pub fn new() -> Self {
        Self {
            language_servers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_language_server(&self, info: LanguageServerInfo) {
        let mut servers = self.language_servers.write().await;
        servers.insert(info.language_id.clone(), info);
    }

    pub async fn get_language_server(&self, language_id: &str) -> Option<LanguageServerInfo> {
        self.language_servers.read().await.get(language_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_workspace_namespace() {
        let workspace = WorkspaceNamespace::new();
        workspace
            .add_workspace_folder(WorkspaceFolder {
                uri: "file:///workspace".to_string(),
                name: "workspace".to_string(),
                index: 0,
            })
            .await;

        let folders = workspace.get_workspace_folders().await;
        assert_eq!(folders.len(), 1);
    }

    #[tokio::test]
    async fn test_commands_namespace() {
        let commands = CommandsNamespace::new();

        commands
            .register_command("test.command".to_string(), |_args| Ok(Value::Null))
            .await
            .unwrap();

        let result = commands.execute_command("test.command", vec![]).await;
        assert!(result.is_ok());
    }
}
