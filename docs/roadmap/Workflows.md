# Fusion Workflows

## Development Workflow

1. **Issue Tracking**: All work must be tracked in issue tickets.
2. **Branching**: Use feature branches (`feature/name`) off `main`.
3. **Code Review**: PRs require 1 approval and passing CI.
4. **Testing**: All PRs must include unit tests.

## Release Workflow

1. **Version Bump**: Update version in `Cargo.toml`.
2. **Changelog**: Update `ChangeLog.md`.
3. **Tag**: Create git tag (e.g., `v0.1.0`).
4. **Build**: CI pipeline builds release binaries.
5. **Publish**: Upload binaries to GitHub Releases and Package Registry.