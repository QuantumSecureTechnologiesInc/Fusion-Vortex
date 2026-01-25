# Fusion Java Interop

**Version:** 0.2.0
**Type:** Language Bridge
**License:** MIT

## Overview

Fusion Java Interop (`interop-java`) facilitates seamless communication between Fusion and the Java Virtual Machine (JVM). It allows Fusion code to instantiate Java classes, call methods, and handle exceptions.

## Features

- **JNI Integration**: High-performance bindings via JNI
- **Type Mapping**: Auto-conversion between Fusion types and Java primitives
- **Class Loading**: Dynamic loading of JARs and classes
- **Exception handling**: Maps Java exceptions to Fusion errors

## Usage

```rust
use interop_java::{Jvm, Class};

let jvm = Jvm::new()?;
let string_class = jvm.find_class("java/util/ArrayList")?;
let list = string_class.new_object()?;
list.call_method("add", &["Hello from Fusion".into()])?;
```text

## Dependencies

- `jni`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)