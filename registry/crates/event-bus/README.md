# Event Bus

A high-performance event bus for the Fusion ecosystem, supporting pub/sub patterns and async event handling.

## Features
- Asynchronous event dispatch
- Type-safe event payloads
- Multi-producer, multi-consumer support

## Usage
```rust
use event_bus::EventBus;

let bus = EventBus::new();
bus.subscribe("topic", |msg| println!("Got: {:?}", msg));
```
