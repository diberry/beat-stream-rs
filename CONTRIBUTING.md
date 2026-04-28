# Contributing to beat-stream-rs

Thank you for your interest in contributing! This guide covers everything you need to get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable, edition 2021)
- [Docker](https://docs.docker.com/get-docker/) (for container builds)
- [Azure CLI](https://learn.microsoft.com/cli/azure/install-azure-cli) (for deployment)
- A modern browser with Web Audio API support

## Development Setup

```bash
# Clone the repository
git clone https://github.com/diberry/beat-stream-rs.git
cd beat-stream-rs

# Build all crates
cargo build --workspace

# Run the server locally
cargo run -p beat-stream-server

# Open frontend
# Navigate to http://localhost:3000 in your browser
```

## Development Workflow

1. Create a feature branch: `git checkout -b feat/your-feature`
2. Make your changes
3. Run quality checks (see below)
4. Commit using [conventional commits](/.github/instructions/git-commit.instructions.md)
5. Push and open a PR

## Quality Checks

All of these must pass before your PR can be merged:

```bash
# Format code
cargo fmt --all

# Run clippy (must be warning-free)
cargo clippy --all-features --workspace -- -D warnings

# Run tests
cargo test --workspace

# (Optional) Check dependencies
cargo deny check
```

## Project Structure

| Path | Description |
|------|-------------|
| `crates/server/` | Axum web server, WebSocket handler, room management |
| `crates/shared/` | Shared types (messages, room state, patterns) |
| `frontend/dist/` | Static frontend assets (HTML, CSS, JS) |
| `infra/` | Azure Bicep infrastructure templates |
| `docs/` | Documentation and test plans |

## WebSocket Protocol

The server and frontend communicate via WebSocket using JSON messages. All message types are defined in `crates/shared/src/lib.rs`. Messages use serde's internally-tagged format:

```json
{"type": "Toggle", "track": 0, "step": 4}
{"type": "SetBpm", "bpm": 128}
{"type": "RequestState"}
```

See the [PRD](docs/PRD.md) Section 9 for the full protocol specification.

## Testing

- **Unit tests:** In `#[cfg(test)]` modules within source files
- **Integration tests:** In `crates/server/tests/`
- **Manual testing:** Run the server locally, open multiple browser tabs

## Docker Build

```bash
# Build the container
docker build -t beat-stream-rs .

# Run locally
docker run -p 3000:3000 beat-stream-rs
```

## Code of Conduct

Be kind, be constructive, and have fun making beats! 🎵
