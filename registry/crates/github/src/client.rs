use anyhow::{Context, Result};
use reqwest::{header, Client, Method};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const GITHUB_API_BASE: &str = "https://api.github.com";
const USER_AGENT: &str = "Fusion-CLI/0.1.0";

/// GitHub API client configuration
#[derive(Debug, Clone)]
pub struct GitHubConfig {
    pub token: String,
    pub timeout: Duration,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            token: String::new(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// GitHub API client
pub struct GitHubClient {
    client: Client,
    #[allow(dead_code)]
    token: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(config: GitHubConfig) -> Result<Self> {
        if config.token.is_empty() {
            anyhow::bail!("GitHub token is required");
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", config.token)
                .parse()
                .context("Invalid token")?,
        );
        headers.insert(
            header::ACCEPT,
            "application/vnd.github+json".parse().unwrap(),
        );
        headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .user_agent(USER_AGENT)
            .timeout(config.timeout)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            token: config.token,
        })
    }

    /// Make an authenticated request
    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        let url = format!("{}{}", GITHUB_API_BASE, path);

        let mut request = self.client.request(method, &url);

        if let Some(b) = body {
            request = request.json(b);
        }

        let response = request.send().await.context("Request failed")?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, error_body);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Get authenticated user
    pub async fn get_user(&self) -> Result<User> {
        self.request(Method::GET, "/user", None::<&()>).await
    }

    /// List user repositories
    pub async fn list_repos(&self, username: Option<&str>) -> Result<Vec<Repository>> {
        let path = if let Some(user) = username {
            format!("/users/{}/repos", user)
        } else {
            "/user/repos".to_string()
        };

        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Create a repository
    pub async fn create_repo(&self, request: CreateRepoRequest) -> Result<Repository> {
        self.request(Method::POST, "/user/repos", Some(&request))
            .await
    }

    /// Get a repository
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let path = format!("/repos/{}/{}", owner, repo);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Fork a repository
    pub async fn fork_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let path = format!("/repos/{}/{}/forks", owner, repo);
        self.request(Method::POST, &path, None::<&()>).await
    }

    /// Delete a repository
    pub async fn delete_repo(&self, owner: &str, repo: &str) -> Result<()> {
        let path = format!("/repos/{}/{}", owner, repo);
        let _: serde_json::Value = self.request(Method::DELETE, &path, None::<&()>).await?;
        Ok(())
    }

    /// List issues
    pub async fn list_issues(&self, owner: &str, repo: &str, state: &str) -> Result<Vec<Issue>> {
        let path = format!("/repos/{}/{}/issues?state={}", owner, repo, state);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Create an issue
    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        request: CreateIssueRequest,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues", owner, repo);
        self.request(Method::POST, &path, Some(&request)).await
    }

    /// Update an issue
    pub async fn update_issue(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        request: UpdateIssueRequest,
    ) -> Result<Issue> {
        let path = format!("/repos/{}/{}/issues/{}", owner, repo, number);
        self.request(Method::PATCH, &path, Some(&request)).await
    }

    /// List pull requests
    pub async fn list_pulls(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
    ) -> Result<Vec<PullRequest>> {
        let path = format!("/repos/{}/{}/pulls?state={}", owner, repo, state);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Create a pull request
    pub async fn create_pull(
        &self,
        owner: &str,
        repo: &str,
        request: CreatePullRequest,
    ) -> Result<PullRequest> {
        let path = format!("/repos/{}/{}/pulls", owner, repo);
        self.request(Method::POST, &path, Some(&request)).await
    }

    /// Merge a pull request
    pub async fn merge_pull(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        request: MergePullRequest,
    ) -> Result<MergeResult> {
        let path = format!("/repos/{}/{}/pulls/{}/merge", owner, repo, number);
        self.request(Method::PUT, &path, Some(&request)).await
    }

    /// List gists
    pub async fn list_gists(&self) -> Result<Vec<Gist>> {
        self.request(Method::GET, "/gists", None::<&()>).await
    }

    /// Create a gist
    pub async fn create_gist(&self, request: CreateGistRequest) -> Result<Gist> {
        self.request(Method::POST, "/gists", Some(&request)).await
    }

    /// Delete a gist
    pub async fn delete_gist(&self, gist_id: &str) -> Result<()> {
        let path = format!("/gists/{}", gist_id);
        let _: serde_json::Value = self.request(Method::DELETE, &path, None::<&()>).await?;
        Ok(())
    }
}

// API Types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub default_branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepoRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIssueRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub merged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullRequest {
    pub title: String,
    pub head: String,
    pub base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergePullRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gist {
    pub id: String,
    pub description: Option<String>,
    pub public: bool,
    pub html_url: String,
    pub files: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGistRequest {
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, GistFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GistFile {
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = GitHubConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_client_requires_token() {
        let config = GitHubConfig::default();
        let result = GitHubClient::new(config);
        assert!(result.is_err());
    }
}
