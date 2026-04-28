# AI Agent Guidelines for beat-stream-rs

This document guides AI assistants (GitHub Copilot, MCP servers, Squad agents) working on beat-stream-rs.

## Project Overview

- **Language:** Rust (edition 2021)
- **Runtime:** Tokio async
- **Framework:** Axum 0.8
- **Platform:** Azure Container Apps (Consumption tier)
- **Real-time:** WebSocket via tokio-tungstenite
- **Frontend:** Vanilla JS + Tone.js (no build step)
- **Container:** Multi-stage Docker build, target <15 MB

## Repository Structure

```
.
├── crates/
│   ├── server/          # Axum web server + WebSocket handler
│   │   ├── src/
│   │   │   ├── main.rs  # Routes, state, startup
│   │   │   ├── room.rs  # Room manager (DashMap + broadcast)
│   │   │   └── ws.rs    # WebSocket handler + rate limiter
│   │   └── tests/       # Integration tests
│   └── shared/          # Shared types (ClientMessage, ServerMessage, RoomState)
├── frontend/
│   └── dist/            # Static assets (index.html, app.js, style.css)
├── infra/               # Azure Bicep templates
├── docs/                # Documentation and test plans
├── .github/
│   ├── workflows/       # CI/CD pipelines
│   └── instructions/    # AI task-specific guidance
├── Cargo.toml           # Workspace root
└── Dockerfile           # Multi-stage container build
```

## Coding Standards

### Rust
- `cargo fmt` required before all commits (uses `.rustfmt.toml` config)
- `cargo clippy --all-features --workspace` must pass with no warnings
- All public types and functions documented with `///`
- Error handling via `Result<T, E>` — no `.unwrap()` in production code
- Use `thiserror` for library errors, `anyhow` only in main/tests
- Async code uses `tokio` runtime — never block the executor
- Prefer `Arc<T>` over `Rc<T>` (multi-threaded by default)

### Serde Conventions
- All message types use `#[serde(tag = "type")]` (internally tagged)
- Field names are camelCase in JSON, snake_case in Rust (use `#[serde(rename_all = "camelCase")]`)
- Enum variants are PascalCase

### Frontend
- Vanilla JavaScript (ES2020+), no framework, no build step
- Web Audio API via Tone.js for audio synthesis
- WebSocket reconnection with exponential backoff

## Architecture Decisions

- **Concurrency:** DashMap for room registry, tokio::sync::broadcast per room
- **Rate limiting:** Token bucket (20 msg/s per client), silent drop
- **State conflicts:** Last-write-wins with server timestamp
- **Scaling (future):** Azure Service Bus for cross-replica pub/sub

## Recommended AI Actions

- ✅ Write or modify Rust code following the standards above
- ✅ Write unit tests (`#[cfg(test)]` modules) and integration tests
- ✅ Improve documentation (`///` doc comments, markdown docs)
- ✅ Suggest clippy fixes and idiomatic refactors
- ✅ Generate Bicep infrastructure changes
- ✅ Modify CI/CD workflows

## Restricted Actions

- ❌ Never commit secrets, API keys, or connection strings
- ❌ Never bypass `cargo fmt` or `cargo clippy` checks
- ❌ Never use `.unwrap()` or `.expect()` in production code paths
- ❌ Never add `unsafe` blocks without explicit justification
- ❌ Never modify `Cargo.lock` manually (let cargo manage it)
- ❌ Never use synchronous I/O in async contexts (no `std::fs` in handlers)

## Building and Testing

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run clippy
cargo clippy --all-features --workspace -- -D warnings

# Format check
cargo fmt --all -- --check

# Build container
docker build -t beat-stream-rs .

# Run locally
cargo run -p beat-stream-server
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| axum 0.8 | HTTP framework + WebSocket upgrade |
| tokio | Async runtime |
| tokio-tungstenite | WebSocket protocol |
| dashmap | Concurrent hash map for rooms |
| serde / serde_json | Serialization |
| tower-http | CORS, static file serving |
| tracing | Structured logging |
