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