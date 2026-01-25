# llm-rlhf

Reinforcement Learning from Human Feedback (RLHF) framework components.

## Features

- Reward model training
- PPO implementation for LLMs
- Feedback collection interfaces

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-rlhf = "0.1.0"
```text

## Usage

```rust
use llm_rlhf::Trainer;

fn main() {
    Trainer::new().step();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.