# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- Initialized Cargo workspace.
- Created `ember-core`, `ember-http`, and `ember-macros` crates.
- Added initial project documentation.
- Established multi-crate project architecture.
- Introduced the first framework trait: `Responder`.
- Documented Ember's trait-based architecture.
- Implemented core framework types including `Server` and `Router` structs.
- Documented module communication, core types, ownership strategy, and router design in `ARCHITECTURE.md`.
- Added module structure (`server`, `router`, `response`) to `ember-core`.
- Added the initial Ember architecture overview.
- Documented request lifecycle and crate responsibilities.
- Defined core engineering principles for the framework.
- Introduced the `EmberError` enum.
- Documented Ember's error handling strategy.
- Added initial TCP binding support to `Server`.
- Introduced conversion from I/O errors to `EmberError`.
- Server now accepts incoming TCP connections.
- Initial runnable networking example completed.
- Added TCP stream reading support.
- Display raw HTTP requests received from clients.
- Added TCP response writing support.
- Completed the first end-to-end TCP request/response cycle.
- Refactored the TCP server into smaller, single-purpose methods.
- Improved maintainability without changing runtime behavior.
- Created the `HttpParser` type.
- Added request line extraction.
- Added initial unit tests for the HTTP parser.
- Added the `Method` enum.
- Added HTTP method parsing.
- Added unit tests for supported and unsupported methods.
- Added the `Request` struct.
- Added request path parsing.
- Added complete request-line parsing.
- Added unit tests for `Request` parsing.
- Added the `Header` struct.
- Added HTTP header parsing.
- Added header storage to the `Request` type.
- Added unit tests for header parsing.
- Replaced `Option`-based HTTP parsing with `Result`.
- Added the `ParserError` enum.
- Improved parser diagnostics for malformed requests.
- Refactored the HTTP parser into multiple focused modules.
- Improved separation of concerns within `ember-http`.
- Preserved the public `HttpParser` API while simplifying internal organization.
- Added the `StatusCode` enum.
- Added numeric status code conversion.
- Added HTTP reason phrase support.
- Added unit tests for status codes.
- Added the `Response` struct.
- Added `Response::new()`.
- Added unit tests for response creation.
- Added fluent builder methods for `Response`.
- Added `header()`, `body()`, and `status()`.
- Added unit tests for the response builder.
- Added HTTP response serialization.
- Automatically generates the status line.
- Automatically includes `Content-Length`.
- Added serialization unit tests.
- Replaced hardcoded HTTP response strings with the `Response` abstraction.
- Integrated response serialization into the TCP server.
- Added the `Route` type.
- Added the `Handler` type alias.
- Added unit tests for route creation.
- Added `Router` type.
- Added route registration methods (`get`, `post`, `put`, `delete`).
- Added router unit tests.
- Added `Router::dispatch()`.
- Added automatic 404 responses.
- Added dispatch unit tests.
- Introduced the `Responder` trait.
- Added implementations for `Response`, `&'static str`, and `String`.
- Generic route registration supporting responder return types.
- Automatic conversion of handler return values into HTTP responses.

### Improved

- Simplified application handler API.
- Removed the requirement for handlers to manually construct `Response` objects.
- Preserved a concrete internal response type while exposing a more ergonomic public API.

- Implemented `FromRequest` for `Path<String>`.
- Added extraction from `Request::params`.
- Added unit tests for successful and failed extraction.
- Updated the example application to use the extractor API.

### Improved

- Application handlers no longer need to access `request.params` directly.

### Added

- Introduced the `Query<T>` extractor wrapper.
- Implemented the `FromRequest` interface placeholder for query extraction.
- Exported the new `query` module.

### Internal

- Prepared the framework for query string parsing in the next phase.

### Added

- Added `query` storage to `Request`.
- Added `Request::query()` helper.
- Implemented query string parsing.
- Separated URL path from query parameters during request parsing.

### Improved

- Request representation now distinguishes between route path and URL query parameters.

Implemented `FromRequest` for `Query<String>`.
Added unit tests for query extraction.
Updated the example application to demonstrate query extraction.
Introduced the `Header<T>` extractor.
Implemented `FromRequest` for `Header<String>`.
Added unit tests for header extraction.
Updated the example application to demonstrate header extraction.
Refactored `Path<T>`, `Query<T>` and `Header<T>` to support deterministic lookups.
Removed reliance on collection iteration order.
Prepared the extraction system for future typed extractors.
- Introduced the `Json<T>` responder.
- Added Serde integration.
- Added automatic JSON serialization.
- Added `application/json` response support.
- Added `EmberError::status_code()`.
- Centralized framework error to HTTP status mapping.
- Implemented `Responder` for `EmberError`.
- Added automatic conversion from framework errors to HTTP responses.
- Added `Responder` implementation for `Result<T, EmberError>`.