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

## Phase 4.7 – Responder Abstraction

### Motivation

Handlers should not always be required to construct `Response` values manually.

The `Responder` trait allows the framework to convert different return types into a standard HTTP response.

### Initial Implementations

- Response
- &'static str
- String

Future implementations will include:

- Json<T>
- Html<T>
- Redirect
- File

# Phase 4.7 — Responder Abstraction

## Overview

Phase 4.7 introduces the `Responder` abstraction to improve Ember's developer experience.

Previously, every route handler was required to manually construct an HTTP `Response`. This tightly coupled application code to the HTTP layer and resulted in unnecessary boilerplate.

The new `Responder` trait allows handlers to return different types while the framework automatically converts them into a standard `Response`.

---

## Motivation

The networking layer should only understand one concrete response type.

Application developers, however, should be free to return ergonomic types such as plain strings, owned strings, JSON responses, HTML responses, or other framework-specific response types.

The `Responder` trait acts as the translation layer between application code and the HTTP protocol.

---

## Request Lifecycle

Browser
        │
        ▼
TCP Server
        │
        ▼
HTTP Parser
        │
        ▼
Router
        │
        ▼
Application Handler
        │
        ▼
Responder::into_response()
        │
        ▼
HTTP Response
        │
        ▼
Socket Writer

---

## Design

Internally, the router stores handlers that always produce a concrete `Response`.

```rust
Box<dyn Fn(Request) -> Response + Send + Sync>
```

During route registration, generic handlers are wrapped so that any type implementing `Responder` is automatically converted into a `Response`.

Conceptually:

```
User Handler
        │
        ▼
&str / String / Response
        │
        ▼
Responder::into_response()
        │
        ▼
Response
        │
        ▼
Router Dispatch
```

This keeps the routing and networking layers simple while exposing a flexible API to framework users.

---

## Initial Responder Implementations

Current implementations include:

- Response
- &'static str
- String

Future implementations will include:

- Json<T>
- Html<T>
- Redirect
- File
- Custom framework response types

---

## Advantages

- Eliminates repetitive `Response::new(...)` boilerplate.
- Decouples application code from the HTTP protocol.
- Allows the framework API to grow without modifying the server implementation.
- Keeps internal architecture simple by using a single concrete response type.

---

## Lessons Learned

One of the most important architectural principles in framework design is separating public APIs from internal implementation.

Users interact with ergonomic abstractions (`Responder`), while the framework itself continues to operate on a single concrete HTTP response type.

This separation improves extensibility without increasing complexity inside the server or router.


# Phase 4.8 — Path Matching Abstraction

## Overview

The router previously performed path comparison directly using string equality.

To prepare for dynamic routes, path matching has been extracted into a dedicated `PathMatcher` component.

## Motivation

Separating path comparison from route management makes the routing system easier to extend while keeping the router focused on registration and dispatch.

## Current Matching Strategy

The initial implementation performs exact path comparison.

```
Route Path
      │
      ▼
PathMatcher
      │
      ▼
Exact Match (==)
      │
      ▼
true / false
```

Future phases will extend this component to support:

- Dynamic parameters (`:id`)
- Wildcards (`*`)
- Optional segments
- Advanced matching strategies

## Benefits

- Single responsibility for path comparison.
- Router remains simple.
- Dynamic routing can be introduced without redesigning the router.

## Phase 5.0 – Dynamic Route Matching

### Overview

The routing engine now supports dynamic path segments.

A route segment beginning with `:` is treated as a wildcard during path matching.

Examples:

- `/users/:id`
- `/posts/:post_id`

### Design

Path comparison is delegated to `PathMatcher`.

Each route and request path is split into individual segments.

Static segments must match exactly.

Dynamic segments match any value.

### Current Scope

This phase only introduces dynamic route matching.

Route parameters are **not yet extracted**.

Parameter extraction will be implemented in the next phase.

## Phase 5.1 – Request Path Parameters

### Overview

The `Request` type now stores route parameters.

Although parameters are not yet populated during routing, the request model has been extended to support dynamic route extraction in future phases.

### Design

A `HashMap<String, String>` named `params` has been added to `Request`.

A convenience method `param()` provides read-only access to extracted values.

### Outcome

The request model is now prepared for dynamic route parameter extraction without affecting the existing routing implementation.

## Phase 5.2 – Route Parameter Extraction

### Overview

Dynamic route parameters are now extracted during request dispatch.

### Design

`PathMatcher` exposes two responsibilities:

- `matches()` determines whether a route matches a request path.
- `extract_params()` collects dynamic path segments into a `HashMap`.

The router populates `Request::params` immediately before invoking the handler.

### Example

Route:

/users/:id

Request:

/users/42

Extracted Parameters:

id → "42"

### Outcome

Handlers can now access route parameters using `request.param("name")` without manually parsing the URL.

## Phase 6.0 – FromRequest Foundation

### Overview

Introduced the `FromRequest` trait as the request-side counterpart to the existing `Responder` trait.

### Design

`FromRequest` defines a common interface for constructing application-specific types from an HTTP request.

The first extractor type, `Path<T>`, has been introduced as the foundation for future path parameter extraction.

### Outcome

The framework now has a common extraction API that will support:

- Path extractors
- Query extractors
- Header extractors

without changing the router or server architecture.

## Phase 6.1 – Path Extractor

### Overview

The first concrete implementation of the `FromRequest` trait has been completed.

`Path<String>` can now construct itself directly from the route parameters stored in `Request`.

### Design

The extractor reads values from `Request::params`, which are populated by the router during dynamic route dispatch.

Current implementation:

- Supports extracting the first path parameter.
- Returns an error if no parameters exist.

### Request Flow

Incoming Request
        │
        ▼
Router
        │
        ▼
Request.params
        │
        ▼
Path<String>::from_request()
        │
        ▼
Application Handler

### Outcome

The framework now supports request-side extraction through the `FromRequest` abstraction.

Future phases will extend this design to support typed path parameters, query strings, headers and JSON payloads.

## Phase 6.2 – Query Extractor Foundation

### Overview

Introduced the `Query<T>` extractor as part of Ember's request extraction system.

The extractor follows the same design pattern as `Path<T>` and implements the `FromRequest` trait.

### Design

Current extractors:

- Path<T>
- Query<T>

Future extractors:

- Header<T>
- Json<T>

### Outcome

The extraction API has been expanded while keeping a consistent interface across all extractor types.

Actual query string parsing will be implemented after the request model is extended to store parsed query parameters.

## Phase 6.3 – Query Parameter Storage

### Overview

The HTTP request model has been extended to store parsed query parameters separately from the request path.

### Design

Incoming request URLs are now split into two components:

- Path
- Query parameters

Example:
Request:
/search?q=rust&page=2
Stored as:

Path:
/search

Query:
q → rust
page → 2

### Outcome
The request model now represents URL structure more accurately and provides the foundation required for implementing the `Query<T>` extractor.

## Phase 6.4 – Query Extractor

### Overview

The `Query<T>` extractor now supports extracting query string values from an HTTP request.

### Request Flow
Incoming Request
        │
        ▼
HTTP Parser
        │
        ▼
Request.query
        │
        ▼
Query<String>::from_request()
        │
        ▼
Application Handler
### Outcome
The extraction pipeline now supports both path parameters and query parameters using the common `FromRequest` abstraction.

## Phase 6.5 – Header Extractor

### Overview

The request extraction system now supports HTTP header extraction.

### Design

The new `Header<T>` extractor implements the `FromRequest` trait and reads values from the parsed request headers.

Current extractor implementations include:

- `Path<T>`
- `Query<T>`
- `Header<T>`

All share the same extraction interface.

### Outcome

The framework now provides a consistent API for accessing route parameters, query parameters and request headers through the extractor system.

## Phase 6.6 – Extractor API Refinement

### Overview

The initial extractor implementations have been refined to provide deterministic extraction.

### Design

Earlier implementations returned the first available value from the request.

The new API performs lookups using explicit keys.

Examples:

- Path parameter by name
- Query parameter by name
- Header by name

### Outcome

The extraction system is now deterministic, predictable and suitable for future strongly typed extractors.

## Phase 7.0 – JSON Responses
### Overview
Phase 7 introduces JSON responses through a dedicated `Json<T>` responder.
### Design
`Json<T>` implements the existing `Responder` trait.
The responder serializes any `T: Serialize` into JSON using `serde_json` and automatically produces an HTTP response with the `Content-Type: application/json` header.
### Request Flow
Application
↓
Json<T>
↓
serde_json
↓
Response
↓
HTTP Client
### Outcome
The framework can now return structured JSON responses without modifying the HTTP response implementation.

## Phase 8.1 – Unified Framework Error Type

### Overview

Introduced a unified framework error type through `EmberError`.

### Layering

TCP
↓
HTTP Parser
↓
ParserError
↓
Framework
↓
EmberError
↓
Application

## Phase 8.2 – Error to HTTP Status Mapping
### Overview
Framework errors are now responsible for determining the HTTP status code returned to clients.
### Flow
Application Error
↓
EmberError
↓
StatusCode
↓
HTTP Response