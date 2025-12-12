use std::path::PathBuf;

/// Filesystem MCP server configuration
#[derive(Debug, Clone)]
pub struct FilesystemServer {
    pub root_path: PathBuf,
}

impl FilesystemServer {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    pub fn command(&self) -> String {
        "npx".to_string()
    }

    pub fn args(&self) -> Vec<String> {
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-filesystem".to_string(),
            self.root_path.to_string_lossy().to_string(),
        ]
    }
}

/// GitHub MCP server configuration
#[derive(Debug, Clone)]
pub struct GitHubServer {
    pub token: String,
}

impl GitHubServer {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn command(&self) -> String {
        "npx".to_string()
    }

    pub fn args(&self) -> Vec<String> {
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-github".to_string(),
        ]
    }

    pub fn env_vars(&self) -> Vec<(String, String)> {
        vec![("GITHUB_TOKEN".to_string(), self.token.clone())]
    }
}

/// Web/HTTP MCP server configuration
#[derive(Debug, Clone)]
pub struct WebServer {
    pub allowed_domains: Vec<String>,
}

impl WebServer {
    pub fn new(allowed_domains: Vec<String>) -> Self {
        Self { allowed_domains }
    }

    pub fn command(&self) -> String {
        "npx".to_string()
    }

    pub fn args(&self) -> Vec<String> {
        let mut args = vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-fetch".to_string(),
        ];

        for domain in &self.allowed_domains {
            args.push(domain.clone());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filesystem_server() {
        let server = FilesystemServer::new(PathBuf::from("/tmp"));
        assert_eq!(server.command(), "npx");
        assert!(server.args().contains(&"-y".to_string()));
    }

    #[test]
    fn test_github_server() {
        let server = GitHubServer::new("token123".to_string());
        assert!(server.env_vars().iter().any(|(k, _)| k == "GITHUB_TOKEN"));
    }

    #[test]
    fn test_web_server() {
        let server = WebServer::new(vec!["example.com".to_string()]);
        assert!(server.args().contains(&"example.com".to_string()));
    }
}
