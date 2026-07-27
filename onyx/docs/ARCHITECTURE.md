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

## Public API Philosophy

Onyx follows Rust's default-private visibility model.

Only components intended for framework users will be marked `pub`. Internal implementation details remain private to reduce API surface area, improve maintainability, and allow internal refactoring without breaking user code.

This approach encourages a clean separation between the framework's public interface and its internal implementation.

## Module Communication

Modules communicate through Rust's module system.

- `crate::` references items from the crate root.
- `use` creates local aliases for cleaner code.
- Modules interact through well-defined public interfaces instead of direct file access.

This approach keeps dependencies explicit and helps maintain a scalable architecture as the framework grows.

## Core Types

The framework models major concepts as Rust structs.

The first core type is `Server`, which will eventually manage networking, configuration, and request handling.

Fields remain private by default to preserve encapsulation. Public behavior is exposed through methods defined in `impl` blocks.

## Ownership Strategy

Core framework types own their internal state. Public methods borrow data whenever possible instead of cloning it.

This approach minimizes allocations, improves performance, and follows Rust's ownership model for safe memory management.