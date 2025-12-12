# Global Package Manager Registry (GPMR)

## Overview
The **Global Package Manager Registry (GPMR)** is the central repository and authority for the Fusion programming language ecosystem. It serves as the single source of truth for all published packages, libraries, and tools.

**Location**: `src/registry/` (Implementation)
**Definition**: "The place where packages will be stored and pulled from."

## Responsibilities
1.  **Storage**: Persists package binaries, source code, and metadata.
2.  **Retrieval**: Provides efficient APIs for the Fusion Package Manager (`pkgmgr` crate) to download dependencies.
3.  **Indexing**: Maintains a searchable index of all available crates (e.g., `fusion-math`, `llm-beam-search`).
4.  **Versioning**: Enforces semantic versioning rules and immutable release history.

## Architecture
The GPMR is implemented in Rust within the `src/registry/` module.
-   **Struct**: `GlobalRegistry`
-   **Modules**:
    -   `api`: HTTP/RPC endpoints for interaction.
    -   `storage`: Backend abstraction (Local FS / S3 / Blob).
    -   `database`: Metadata index (Postgres / SQLite).
    -   `auth`: User authentication and publishing tokens.

## Usage
The Fusion CLI and Compiler interact with the GPMR automatically when resolving dependencies in `Cargo.toml` or `Fusion.toml`.
