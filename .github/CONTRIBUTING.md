# Contributing to beat-stream-rs

Thank you for your interest in contributing! This guide will help you get started.

## Prerequisites

- **Rust 1.82+** — install via [rustup](https://rustup.rs/)
- **Azure CLI** — [install guide](https://learn.microsoft.com/cli/azure/install-azure-cli)
- **Docker** — for container builds and local testing
- **cargo-deny** — `cargo install cargo-deny --locked`

## Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/diberry/beat-stream-rs.git
   cd beat-stream-rs
   ```

2. Run the server:
   ```bash
   cargo run
   ```
   The server starts on `http://localhost:8080`.

3. Run tests:
   ```bash
   cargo test --workspace
   ```

4. Check formatting and lints:
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets
   ```

## Azure Resources

For integration tests and deployment, you need Azure resources provisioned.
See `infra/README.md` for resource provisioning instructions.

## PR Process

1. **Branch from main** — create a feature branch (`feat/my-feature`) or bugfix branch (`fix/my-fix`).
2. **CI must pass** — the PR pipeline runs `fmt`, `clippy`, `test`, and `cargo deny check`.
3. **One reviewer required** — request a review before merging.
4. **Squash merge** — keep the commit history clean.

## Code Style

- Follow standard Rust conventions (`cargo fmt` enforces formatting).
- Use `thiserror` for library error types, `anyhow` for application errors.
- Keep async — no blocking calls in the WebSocket path.
- All Azure SDK calls go through `DefaultAzureCredential`.

## Questions?

Open an issue or start a discussion in the repository.
