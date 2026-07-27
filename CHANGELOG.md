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