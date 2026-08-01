# Onyx Framework

> A modern, modular web framework written entirely in Rust from first principles.

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-v0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

---

## About
Onyx is a modular web framework that provides foundational layers to build robust web applications. Built from the ground up to emphasize performance, safety, and developer experience.

## Tier Progress

### 🚀 v0.1 Released

Tier 1 of Onyx is now complete.

The framework includes:
- Raw TCP HTTP server
- HTTP parser
- Request/Response types
- Router
- Dynamic routing
- Path parameters
- Query parsing
- Request extractors
- JSON responses
- Centralized error handling
- Result-based handlers

**Current Status:** Tier 1 ✅ Complete

## Features

### HTTP
- HTTP/1.1 parser
- Request model
- Response builder
- Header parsing
- Query parsing

### Routing
- Static routes
- Dynamic routes
- Path matching
- Route dispatch

### Request Extraction
- `Path<T>`
- `Query<T>`
- `Header<T>`

### Responses
- String responder
- JSON responder
- Custom responder trait

### Errors
- Typed framework errors
- HTTP status mapping
- `Result<T, EmberError>`

## Example

```rust
use ember_core::prelude::*;

fn home(_: Request) -> &'static str {
    "Hello Ember!"
}

fn user(_: Request) -> Json<User> {
    Json(User {
        id: 1,
        name: "Soham".into(),
    })
}

fn missing(_: Request) -> Result<&'static str, EmberError> {
    Err(EmberError::NotFound)
}
```

## Installation
*(Coming soon)*

## Quick Start
*(Coming soon)*

## Project Structure
*(Coming soon)*

## Roadmap

### Tier 1
- [x] Workspace
- [x] TCP Server
- [x] HTTP Parser
- [x] Request
- [x] Response
- [x] Router
- [x] Dynamic Routing
- [x] Extractors
- [x] JSON
- [x] Error Handling

✅ Completed in v0.1

## Documentation
See `docs/ARCHITECTURE.md` and `docs/DECISIONS.md` for deep dives into the framework's design and progression.

## License
MIT
