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

## Phase 1.4 – Writing Responses

The server can now complete a full TCP request-response cycle.

### Current Flow

1. Bind TCP listener.
2. Accept client connection.
3. Read incoming bytes.
4. Send a valid HTTP response.
5. Close the connection.

### Design Decisions

- Responses are currently hardcoded.
- HTTP parsing has intentionally not started yet.
- The focus remains on validating the transport layer before building protocol abstractions.

### Future Work

- Parse the incoming HTTP request.
- Replace hardcoded responses with `Response` objects.
- Introduce a response builder.

## TCP Server Refactoring

The TCP server implementation has been refactored into smaller methods following the Single Responsibility Principle (SRP).

### Current Structure

Server

- bind()
- accept_connection()
- read_request()
- write_response()
- run()

### Design Rationale

Each method has one clearly defined responsibility:

- `bind()` opens the listening socket.
- `accept_connection()` accepts a client connection.
- `read_request()` reads raw bytes and converts them to text.
- `write_response()` sends an HTTP response.
- `run()` orchestrates the overall server lifecycle.

This structure improves readability, testing, and future extensibility.

## Phase 2 – HTTP Parsing

The HTTP parser is implemented in the `ember-http` crate to separate protocol handling from server runtime responsibilities.

### Phase 2.1 Scope

The first parser extracts only the HTTP request line.

Example:

GET /users HTTP/1.1

No headers or body are parsed yet.

### Design Decisions

- Introduce a dedicated `HttpParser` type.
- Return `Option<&str>` to safely handle malformed or empty requests.
- Build the parser incrementally, validating each stage with unit tests before adding new protocol features.

## Phase 2.2 – HTTP Methods

Added a strongly typed `Method` enum to represent supported HTTP request methods.

### Design Decisions

- Store methods as an enum rather than strings.
- Return `Option<Method>` while the parser is still under construction.
- Keep method parsing separate from path and version parsing.

### Benefits

- Compile-time type safety.
- Easier pattern matching.
- Eliminates string comparisons throughout the framework.

## Phase 2.3 – HTTP Version Parsing

Added a strongly typed `HttpVersion` enum.

### Design Decisions

- Represent protocol versions as an enum instead of strings.
- Keep version parsing independent of method and path parsing.
- Return `Option<HttpVersion>` until parser error handling is introduced.

### Benefits

- Compile-time validation.
- Cleaner pattern matching.
- Easier protocol-specific behavior in future phases.

## Phase 2.4 – Request Object

Added the first `Request` type to represent parsed HTTP requests.

### Responsibilities

- Store the parsed HTTP method.
- Store the request path.
- Store the HTTP version.

### Design Decisions

- Use enums for protocol-defined values (`Method`, `HttpVersion`).
- Use `String` for request paths because the set of possible paths is unbounded.
- Parse the request once and pass structured data to the rest of the framework.

## Phase 2.5 – HTTP Header Parsing

Added support for parsing HTTP headers into structured `Header` objects.

### Responsibilities

- Parse individual header lines.
- Collect all request headers.
- Store headers inside the `Request` type.

### Design Decisions

- Introduce a dedicated `Header` struct instead of tuples.
- Use `Vec<Header>` to support any number of request headers.
- Use `split_once(':')` to correctly separate header names from values.

## Phase 2.6 – Typed Parser Errors

Replaced `Option`-based parsing with `Result<Request, ParserError>`.

### Design Decisions

- Introduced a dedicated `ParserError` enum.
- Preserve the specific reason for parser failures.
- Stop silently ignoring malformed headers.

### Benefits

- Better debugging.
- Better testability.
- Foundation for centralized framework error handling.