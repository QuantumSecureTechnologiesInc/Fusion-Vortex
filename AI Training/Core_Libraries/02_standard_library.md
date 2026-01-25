# Standard Library

**Dataset Category**: Core Libraries
**Training Level**: Intermediate to Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion std provides safe, ergonomic APIs built on the runtime core. It wraps platform primitives, enforces security defaults, and exposes the AI/quantum runtime integration points.

## Modules

- **io**: print_line, print_int, file read/write helpers
- **mem**: alloc, release, memcpy wrappers
- **math**: numeric helpers, constants, fast paths
- **string**: dynamic strings, normalization helpers
- **ai**: session_from_env, LLM runtime helpers

## Configuration (Fusion.toml)

```toml
[stdlib]
features = ["io", "mem", "string", "ai"]

[ai]
provider = "ollama"

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"
```text

## Practical Examples

```fusion
use fusion::std::io::print_line
use fusion::std::string::String

fn main() -> int {
    let msg = String::from("Hello Fusion");
    print_line(msg);
    return 0;
}
```text

## AI Runtime Example

```fusion
use fusion::std::ai::session_from_env

fn main() -> int {
    let sess = session_from_env()?;
    let out = sess.chat("Write a function to hash a password safely");
    print_line(out);
    return 0;
}
```text

## Notes

- Fusion.toml is the canonical manifest.
- AI providers supported: ollama, qwen, deepseek, gpt-oss, mistral, phi, gemma, openai.

## References

- docs/guides/FUSION_COMPLETE_GUIDEBOOK.md
- docs/FUSION_TOML_COMPLETE_GUIDE.md