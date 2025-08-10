# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based asynchronous REST API for a ticket management system built with Rocket framework. It's part of a "100 exercises to learn Rust" course, specifically exercise 08 (futures/async).

## Architecture

The codebase follows a clean architecture pattern with clear separation of concerns:

- **Domain Layer** (`src/domain/`): Core business logic and data structures
  - `data.rs`: Core entities (Ticket, TicketDraft, Status enum)
  - `store.rs`: Optimized in-memory ticket storage using `Arc<Mutex<TicketStore>>` for thread-safety
  - `title.rs`, `description.rs`: Value objects with validation and `value()` accessor methods
  - `test_helpers.rs`: Testing utilities

- **DTO Layer** (`src/dto/`): Data Transfer Objects for API communication
  - `ticket_response.rs`: Response serialization with `From<Ticket>` trait implementations
  - `create_ticket_request.rs`: POST request deserialization with `TryFrom<CreateTicketRequest>` for `TicketDraft`
  - `update_ticket_request.rs`: PUT request deserialization with `TryFrom<UpdateTicketRequest>` for `TicketUpdate`
  - `ticket_update.rs`: Intermediate update structure for partial ticket modifications
  - Each DTO file contains its own comprehensive test suite

- **Routes Layer** (`src/routes/`): HTTP endpoint handlers
  - `ticket.rs`: REST endpoints using trait-based conversions for clean, type-safe request/response handling
  - `infra.rs`: Infrastructure concerns (error handlers, catchers)

- **Application Entry**: 
  - `main.rs`: Rocket server setup with `Arc<Mutex<TicketStore>>` state management
  - `lib.rs`: Module declarations

## Key Design Patterns

- **Optimized Thread-Safety**: Uses single `Arc<Mutex<TicketStore>>` instead of nested locking for better performance
- **Trait-Based Conversions**: 
  - `TryFrom<CreateTicketRequest>` for `TicketDraft` with validation
  - `TryFrom<UpdateTicketRequest>` for `TicketUpdate` with partial field updates
  - `From<Ticket>` for `TicketResponse` supporting `&Ticket`, `Ticket`, and `&mut Ticket`
- **Value Objects**: Title and Description with validation and accessor methods
- **Simplified Storage**: Direct `BTreeMap<TicketId, Ticket>` without inner Arc/RwLock
- **Modular Testing**: Tests colocated with implementation for better context and maintainability

## Common Development Commands

```bash
# Build the project
cargo build

# Run the application (starts HTTP server)
cargo run

# Run tests
cargo test

# Check code without building
cargo check

# Format code (if rustfmt is available)
cargo fmt

# Lint code (if clippy is available) 
cargo clippy
```

## API Endpoints

The server exposes these endpoints under `/api`:
- `GET /api/ticket/<id>` - Retrieve ticket details
- `POST /api/ticket` - Create a new ticket
- `PUT /api/ticket/<id>` - Update an existing ticket

## Dependencies

- **rocket**: Web framework for the REST API (with "json" feature for JSON support)
- **tokio**: Async runtime with "full" features
- **serde**: Serialization/deserialization with "derive" feature
- **serde_json**: JSON handling
- **thiserror**: Structured error handling for validation failures

## Implementation Guidelines

### Request/Response Handling
- Use trait-based conversions instead of manual field mapping
- `TryFrom` for request validation and conversion to domain objects
- `From` for response generation from domain objects
- Error handling through custom error types with `thiserror`

### Testing Strategy
- Tests are colocated with their implementations (not in separate test files)
- Each DTO module has comprehensive test coverage including edge cases
- Domain validation tests ensure business rules are enforced
- Use `#[cfg(test)]` modules within each file for context-aware testing

### Thread Safety
- Use `Arc<Mutex<T>>` only at the application boundary (Rocket state)
- Avoid nested Arc/Mutex patterns - simplify to direct ownership within synchronized contexts
- Prefer `get_mut()` methods for direct mutable access within locks