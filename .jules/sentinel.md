## 2026-07-04 - HTTP Parameter Pollution in Package Registry Client
**Vulnerability:** HTTP Parameter Pollution / URL injection vulnerability in `src/package_manager/registry.fu`. The `search` function constructed the URL by directly concatenating the user-provided `query` string into the URL path (`format!("{}/api/v1/search?q={}", self.registry_url, query)`).
**Learning:** Manual URL parameter concatenation creates significant injection risks when building HTTP clients. Rust's `reqwest` crate handles proper URL encoding securely, but only when used correctly through its builder pattern.
**Prevention:** Always use the `.query(&[("key", value)])` builder methods provided by HTTP clients (like `reqwest`) rather than manual string formatting for query parameters.
