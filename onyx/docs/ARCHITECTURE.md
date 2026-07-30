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

## Phase 2 – HTTP Parsing

The HTTP parser is implemented in the `ember-http` crate to separate protocol handling from server runtime responsibilities.

### Phase 2.1 Scope

The first parser extracts only the HTTP request line.

Example:

GET /users HTTP/1.1

No headers or body are parsed yet.

## Phase 2.2 – HTTP Methods

Added a strongly typed `Method` enum to represent supported HTTP request methods.

### Benefits

- Compile-time type safety.
- Easier pattern matching.
- Eliminates string comparisons throughout the framework.

## Phase 2.3 – HTTP Version Parsing

Added a strongly typed `HttpVersion` enum.

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

## Phase 2.5 – HTTP Header Parsing

Added support for parsing HTTP headers into structured `Header` objects.

### Responsibilities

- Parse individual header lines.
- Collect all request headers.
- Store headers inside the `Request` type.

## Phase 2.6 – Typed Parser Errors

Replaced `Option`-based parsing with `Result<Request, ParserError>`.

### Benefits

- Better debugging.
- Better testability.
- Foundation for centralized framework error handling.

## Phase 2.7 – Parser Refactoring

The HTTP parser has been reorganized into smaller modules.

### Module Responsibilities

- `request_line.rs` – Parse the HTTP request line.
- `headers.rs` – Parse HTTP headers.
- `request.rs` – Assemble a complete `Request`.
- `mod.rs` – Expose the parser's public API.

### Benefits

- Smaller files.
- Better readability.
- Easier testing.
- Easier long-term maintenance.

## Phase 3.1 – HTTP Status Codes

Introduced the `StatusCode` enum to represent standard HTTP response codes.

### Benefits

- Compile-time type safety.
- Easier pattern matching.
- Eliminates magic numbers throughout the framework.

## Phase 3.2 – Response Object

Added the `Response` type to represent outgoing HTTP responses.

### Responsibilities

- Store the HTTP status code.
- Store response headers.
- Store the response body.

### Benefits

- Strongly typed response model.
- Clean separation between response creation and serialization.

## Phase 3.3 – Response Builder

Added a fluent builder API to `Response`.

### Builder Methods

- `new()`
- `header()`
- `body()`
- `status()`

### Benefits

- Cleaner handler code.
- Easier response construction.
- Foundation for future responder traits.

## Phase 3.4 – HTTP Response Serialization

Added response serialization support.

### Responsibilities

- Convert a `Response` into valid HTTP/1.1 bytes.
- Automatically generate the status line.
- Serialize headers.
- Automatically add `Content-Length`.
- Separate headers from the body using the required blank line.

## Phase 3.5 – TCP Integration

Integrated the `Response` system into the TCP server.

### Benefits

- Clear separation of concerns.
- Easier testing.
- Prepares the framework for routing and middleware.

# Phase 3 – HTTP Response System

## Overview

Implemented a complete HTTP response system for Ember.

### Components

- StatusCode
- Response
- Response Builder
- Response Serialization

### Responsibilities

ember-http

- Parse HTTP requests.
- Represent HTTP requests.
- Represent HTTP responses.
- Serialize HTTP responses.

ember-core

- Accept TCP connections.
- Read incoming bytes.
- Parse requests.
- Produce responses.
- Send serialized bytes.

### Benefits

- Strong separation between networking and HTTP.
- Strongly typed protocol representation.
- Easy extension for routing and middleware.

# Phase 4.1 – Router Architecture

## Goal

Design the routing system before implementing it.

### Responsibilities

Router

- Register routes.
- Match incoming requests.
- Dispatch the matching handler.

Route

- HTTP method.
- URL path.
- Handler function.

### Future Improvements

- Dynamic routes.
- Nested routers.
- Route groups.
- Middleware.
- Radix tree optimization.

## Phase 4.2 – Route Type

Introduced the `Route` abstraction.

### Responsibilities

- Store the HTTP method.
- Store the route path.
- Store the handler function.

### Benefits

- Strongly typed route representation.
- Clear separation between routing and HTTP protocol types.
- Foundation for the router implementation.

## Phase 4.3 – Router

Implemented the `Router` abstraction.

### Responsibilities

- Own all registered routes.
- Provide an ergonomic API for route registration.
- Hide the underlying storage implementation.

## Phase 4.4 – Route Matching

### Added

- Route lookup using method and path.
- Borrow-based matching (`&Request` → `Option<&Route>`).

### Complexity

Current implementation uses linear search (`O(n)`).

Future versions may replace the internal storage with a radix tree while preserving the same public API.

## Phase 4.5 – Handler Dispatch

### Added

- `Router::dispatch(Request) -> Response`

### Responsibilities

- Locate the matching route.
- Execute the registered handler.
- Return a 404 response if no route matches.

## Phase 4.6 – Server and Router Integration

### Overview

The server no longer constructs HTTP responses directly.

Instead, it delegates request handling to the router.

### Request Pipeline

TCP
→ Read Request
→ Parse HTTP
→ Router Dispatch
→ Handler
→ Response
→ Serialize
→ Socket

### Responsibilities

- Server: networking
- HttpParser: protocol parsing
- Router: request dispatch
- Handler: application logic
- Response: HTTP serialization