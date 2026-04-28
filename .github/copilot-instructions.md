# beat-stream-rs

## Project Overview
A real-time collaborative beat sequencer built in Rust on Azure Container Apps.
Showcases Azure SDK for Rust across 8+ service crates.

## Tech Stack
- Rust (2021 edition), Axum web framework, Tokio async runtime
- Azure SDK for Rust: identity, cosmos, keyvault, eventhubs, servicebus, storage
- Frontend: Vanilla JS + Tone.js + Web Audio API
- Infrastructure: Azure Container Apps, Cosmos DB serverless, Key Vault

## Code Conventions
- Use `thiserror` for error types, `anyhow` for application errors
- Async everywhere — no blocking calls in the WebSocket path
- All Azure SDK usage goes through `azure_identity::DefaultAzureCredential`
- Emoji-based sound labels in UI (no music terminology)
- Keep container image < 15MB (static linking, no glibc)

## Architecture
- Single Axum binary serving both API and static files
- One `tokio::broadcast` channel per room for real-time sync
- Cosmos DB for persistence (rooms, patterns)
- Key Vault for secrets (loaded once at startup, cached)

## Testing
- Unit tests: `cargo test`
- Integration tests: `cargo test --features integration` (requires Azure resources)
- Frontend: manual testing via browser (no JS test framework yet)

## CI Pipeline
- `cargo fmt --check` → `cargo clippy` → `cargo test` → `cargo deny check`
- Deploy: Docker build → ACR push → Container Apps update (OIDC auth, no stored secrets)
