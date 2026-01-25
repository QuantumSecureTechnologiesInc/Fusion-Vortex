# Fusion GitHub

**Version:** Workspace
**Type:** Service Integration
**License:** MIT / Apache 2.0 Dual License

## Overview

Fusion GitHub (`fusion-github`) provides a strongly-typed client for the GitHub API. It is used by Fusion's PR review agents, release automation tools, and CI/CD integrations.

## Features

- **API Coverage**: Repositories, Issues, Pull Requests, Actions, Packages
- **Auth**: Supports Personal Access Tokens and GitHub Apps
- **Webhooks**: Utilities for verifying and parsing webhook payloads
- **Diff Parsing**: Utilities for parsing PR diffs for code review

## Usage

```rust
use fusion_github::{GitHubClient, PullRequest};

let client = GitHubClient::new(token)?;
let pr = client.get_pr("owner/repo", 123).await?;

println!("PR Title: {}", pr.title);

// Post a comment
client.post_comment(pr.number, "Reviewed by Fusion AI").await?;
```text

## Dependencies

- `fusion_runtime_core`
- `reqwest`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)