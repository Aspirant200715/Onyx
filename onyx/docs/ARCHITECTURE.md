# Workspace Architecture

Onyx is organized as a Cargo workspace instead of a single crate.

## Crates

- ember-core: Core framework functionality.
- ember-http: HTTP protocol parsing and related types.
- ember-macros: Custom procedural macros for developer ergonomics.

This separation improves maintainability, allows independent testing, and follows the architecture defined in the project roadmap.

## Crate Layout

Each Ember crate starts as a library crate (`lib.rs`) because the framework is designed as a collection of reusable components rather than a standalone executable.

Crate-level documentation (`//!`) is used to describe the purpose of each crate and serves as the entry point for generated API documentation.

## Module Organization

The `ember-core` crate is organized into Rust modules.

Modules allow related functionality to be grouped into separate files, improving readability, maintainability, and future extensibility.

Every new subsystem (server, router, middleware, request, response, etc.) will be introduced as its own module.