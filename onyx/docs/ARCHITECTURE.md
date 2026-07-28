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

## Router Design (Initial)

The initial router stores routes in a `Vec<String>` to keep the implementation simple and easy to understand.

This is an intentional educational trade-off. As Ember evolves, the router implementation will transition to a more efficient data structure while preserving its public API.

## Trait-Based Design

Ember favors trait-based abstractions over inheritance.

Traits define shared behavior without forcing unrelated types into a rigid hierarchy.

The `Responder` trait is the first example of this philosophy and will allow different response types to be handled through a common interface.

# Onyx Architecture Overview

## Request Lifecycle

Every HTTP request follows the same lifecycle:

Browser
→ TCP Connection
→ Server
→ HTTP Parser
→ Request
→ Router
→ Handler
→ Responder
→ HTTP Response
→ Browser

## Crate Responsibilities

### ember-core

- Server
- Router
- Request
- Response
- Middleware
- Application State
- Error Handling

### ember-http

Responsible for the HTTP protocol itself.

### ember-macros

Responsible for developer ergonomics through procedural macros.

## Design Principles

- Separation of Concerns
- Explicit APIs
- Type Safety
- Educational Design
- Stable Public Interfaces

The architecture is intentionally modular so individual components can evolve without affecting unrelated parts of the framework.

## Error Handling Strategy

Ember uses Rust's `Result<T, E>` type for recoverable errors.

Rather than relying on exceptions, functions explicitly return either a successful value (`Ok`) or an error (`Err`).

### Framework Error Type

The framework defines a central `EmberError` enum to represent domain-specific failures.

Initial variants include:

- `InvalidConfiguration`
- `Network`

As the framework grows, additional error variants will be added while preserving a consistent error handling strategy.

## TCP Layer

The first networking capability added to Ember is the ability to bind a TCP listener.

At this stage, the framework only opens a listening socket. It does not yet accept incoming connections.

### Design Decisions

- Use Rust's standard library (`std::net::TcpListener`) for the synchronous implementation.
- Return `Result<TcpListener, EmberError>` to expose framework-specific errors.
- Keep the listener separate from the `Server` struct until successful binding occurs.

### Future Work

- Accept incoming connections.
- Read bytes from clients.
- Build the HTTP layer on top of the TCP stream.

## Phase 1.2 Summary

The server can now:

- Bind to a TCP address.
- Accept a client connection.
- Retrieve the client's socket address.

At this stage, the framework validates the transport layer only. No application protocol (HTTP) is interpreted yet.

This layered approach intentionally separates transport concerns from protocol concerns, making the architecture easier to extend and test.

## Reading TCP Streams

After accepting a TCP connection, Ember reads raw bytes from the `TcpStream`.

### Current Flow

1. Accept connection.
2. Allocate a fixed-size buffer.
3. Read bytes into the buffer.
4. Convert the received bytes into UTF-8 text.
5. Display the raw HTTP request.

### Design Decisions

- Use a fixed-size buffer initially for simplicity.
- Read only a single request.
- Delay HTTP parsing until Phase 2.

### Future Improvements

- Dynamically sized buffers.
- Support multiple reads for larger requests.
- Handle partial reads correctly.