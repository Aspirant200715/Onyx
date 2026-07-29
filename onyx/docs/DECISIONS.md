# Design Decisions

## TCP Layer

- Use Rust's standard library (`std::net::TcpListener`) for the synchronous implementation.
- Return `Result<TcpListener, EmberError>` to expose framework-specific errors.
- Keep the listener separate from the `Server` struct until successful binding occurs.

## Reading TCP Streams

- Use a fixed-size buffer initially for simplicity.
- Read only a single request.
- Delay HTTP parsing until Phase 2.

## Phase 1.4 – Writing Responses

- Responses are currently hardcoded.
- HTTP parsing has intentionally not started yet.
- The focus remains on validating the transport layer before building protocol abstractions.

## TCP Server Refactoring

Each method has one clearly defined responsibility:

- `bind()` opens the listening socket.
- `accept_connection()` accepts a client connection.
- `read_request()` reads raw bytes and converts them to text.
- `write_response()` sends an HTTP response.
- `run()` orchestrates the overall server lifecycle.

This structure improves readability, testing, and future extensibility.

## Phase 2 – HTTP Parsing

- Introduce a dedicated `HttpParser` type.
- Return `Option<&str>` to safely handle malformed or empty requests.
- Build the parser incrementally, validating each stage with unit tests before adding new protocol features.

## Phase 2.2 – HTTP Methods

- Store methods as an enum rather than strings.
- Return `Option<Method>` while the parser is still under construction.
- Keep method parsing separate from path and version parsing.

## Phase 2.3 – HTTP Version Parsing

- Represent protocol versions as an enum instead of strings.
- Keep version parsing independent of method and path parsing.
- Return `Option<HttpVersion>` until parser error handling is introduced.

## Phase 2.4 – Request Object

- Use enums for protocol-defined values (`Method`, `HttpVersion`).
- Use `String` for request paths because the set of possible paths is unbounded.
- Parse the request once and pass structured data to the rest of the framework.

## Phase 2.5 – HTTP Header Parsing

- Introduce a dedicated `Header` struct instead of tuples.
- Use `Vec<Header>` to support any number of request headers.
- Use `split_once(':')` to correctly separate header names from values.

## Phase 2.6 – Typed Parser Errors

- Introduced a dedicated `ParserError` enum.
- Preserve the specific reason for parser failures.
- Stop silently ignoring malformed headers.

## Phase 2.7 – Parser Refactoring

- Keep each parser module focused on one responsibility.
- Hide implementation details behind `HttpParser`.
- Make future protocol extensions easy to integrate.

## Phase 3.1 – HTTP Status Codes

- Represent status codes as a strongly typed enum instead of raw integers.
- Expose helper methods for the numeric code and standard reason phrase.
- Add new status codes incrementally as framework features require them.

## Phase 3.2 – Response Object

- Use the existing `Header` type for consistency.
- Use `StatusCode` instead of raw integers.
- Keep the body as `String` initially; introduce a dedicated `Body` type later.

## Phase 3.3 – Response Builder

- Builder methods consume and return `Self`.
- Accept `impl Into<String>` for ergonomic APIs.
- Keep `Response` immutable from the caller's perspective while allowing fluent construction.

## Phase 3.4 – HTTP Response Serialization

- Serialization is implemented on `Response`.
- The TCP layer only writes bytes and remains HTTP-agnostic.
- The framework automatically manages protocol-specific details such as `Content-Length`.

## Phase 3.5 – TCP Integration

- The TCP layer is responsible only for transmitting bytes.
- `Response` owns all HTTP formatting and serialization logic.
- Handlers and future routers will return `Response` objects instead of raw byte strings.

## Goal

- Place routing inside `ember-core`, not `ember-http`.
- Store routes as a collection of `Route` objects.
- Identify routes using both the HTTP method and path.
- Begin with `Vec<Route>` for simplicity before introducing more advanced data structures.

## Phase 4.2 – Route Type

- Keep routing logic inside `ember-core`.
- Represent handlers using a function pointer type alias.
- Provide a constructor for ergonomic route creation.

## Phase 4.3 – Router

- Store routes internally in a `Vec<Route>`.
- Expose registration methods for common HTTP methods.
- Return route slices (`&[Route]`) instead of exposing the internal vector.

## Phase 4.4 – Route Matching

- The router only locates routes.
- Route execution remains a separate concern.
- Return borrowed routes to avoid unnecessary cloning.

