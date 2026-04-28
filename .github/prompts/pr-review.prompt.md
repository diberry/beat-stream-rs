---
mode: "agent"
description: "Review pull requests for beat-stream-rs compliance"
---

# PR Review — beat-stream-rs Governance

You are reviewing a pull request for **beat-stream-rs**, a real-time collaborative beat sequencer built in Rust on Azure Container Apps.

This review enforces both project-specific rules AND [Azure SDK for Rust best practices](https://azure.github.io/azure-sdk/rust_introduction.html).

## Review Checklist

### 🦀 Rust Quality

1. **No `.unwrap()` or `.expect()` in production code** — only allowed in tests and examples. Use `map()`, `unwrap_or_else()`, or propagate with `?`
2. **No `unsafe` blocks** without explicit safety comment justifying invariants
3. **All public types/functions have `///` doc comments**
4. **Error handling uses `Result<T, E>`** — prefer `thiserror` for typed errors with `Display` impl
5. **No blocking I/O in async context** — no `std::fs`, `std::net`, or `thread::sleep` in handlers
6. **Clippy compliance** — code must pass `cargo clippy --all-features --workspace -- -D warnings`
7. **Format compliance** — code must pass `cargo fmt --all -- --check`
8. **No `clone()` without justification** — flag unnecessary allocations
9. **Atomic ordering** — use `Ordering::AcqRel` for shared state, never `Relaxed` for synchronization
10. **No `prelude` modules** — these lead to name collisions across crate versions

### 🛡️ Azure Rust Best Practices

_From [Azure SDK Rust Guidelines](https://azure.github.io/azure-sdk/rust_introduction.html) and [Implementation Guide](https://azure.github.io/azure-sdk/rust_implementation.html)_

1. **SafeDebug for PII protection** — do NOT derive `Debug` on types that may contain user data (usernames, room names, IPs). Use manual `Debug` impl with `finish_non_exhaustive()` or implement a `SafeDebug` pattern that elides sensitive fields
2. **No PII in tracing/telemetry** — never log user-identifying information at any level. Use opaque IDs in logs
3. **Structured tracing** — use the `tracing` crate with proper log levels:
   - **Error**: fatal errors returning `Result::Err`
   - **Warn**: non-fatal unexpected conditions (NOT retryable 429s)
   - **Info**: high-level diagnostics (connections, disconnections, room lifecycle)
   - **Debug**: request/response details (without PII)
   - **Trace**: internal state changes (lock acquisitions, channel ops)
4. **Naming conventions** — follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html):
   - snake_case for functions/methods/modules
   - PascalCase for types/traits
   - SCREAMING_SNAKE_CASE for constants
   - Verb patterns: `get_`, `set_`, `with_` (builder), `add_`, `delete_`, `list_`
   - No abbreviations unless industry-standard (e.g., `ws` for WebSocket is OK, `rm` for room is NOT)
5. **MSRV awareness** — do not use language features newer than the declared `rust-version` in Cargo.toml. Flag usage of nightly-only features
6. **Error types implement `Display` and `std::error::Error`** — all custom error types must have meaningful, actionable error messages. Error messages should be concise, correlated with context, and human-readable
7. **Thread-safe clients** — public service types must be `Send + Sync`. Immutable by default; interior mutability only when justified
8. **No runtime environment assumptions** — do not assume env vars exist at runtime. Use `Option<T>` for env-based config with documented fallbacks. This matters for container/WASM portability
9. **Module organization** — re-export primary types from crate root. Use submodules for organization (`models/`, `handlers/`, `protocol/`). Keep `lib.rs` as a clean module tree with `pub use` re-exports
10. **Dependency hygiene** — new deps must be justified, MIT/Apache-2.0/BSD/ISC licensed, and should not unconditionally pull in a specific async runtime (prefer feature-gated where possible)

### 🌐 WebSocket Protocol Compliance

1. **Message format** — all messages use `#[serde(tag = "type")]` internally-tagged JSON
2. **Client→Server variants:** `Toggle { track, step }`, `SetBpm { bpm }`, `RequestState`
3. **Server→Client variants:** `State`, `Toggle`, `BpmChanged`, `UserJoined`, `UserLeft`, `Error`
4. **No `data` wrapper** — messages are flat JSON objects with a `type` field
5. **Field naming** — camelCase in JSON (via `#[serde(rename_all = "camelCase")]`), snake_case in Rust

### ⚡ Concurrency Safety

1. **No holding DashMap guards across `.await`** — clone `Arc<T>` before async work
2. **Broadcast channels** — capacity must be explicit (currently 1024)
3. **Rate limiting** — token bucket at 20 msg/s per client, silent drop (no error broadcast)
4. **UserJoined broadcast** — must occur AFTER the receiver task is spawned
5. **Per-client error channel** — use `mpsc`, never broadcast errors to all clients

### ☁️ Azure & Infrastructure

1. **No hardcoded secrets** — use Key Vault references or environment variables with `Option<T>` defaults
2. **Bicep changes** — must include parameter defaults and resource naming consistency
3. **Container size** — changes should not bloat the image beyond 15 MB target
4. **OIDC auth** — CI must use federated credentials, never stored secrets
5. **Health endpoint** — `/health` must remain a simple 200 OK (no DB dependency in Phase 1)

### 🔧 Toolchain & Dependencies

1. **Axum 0.8** — use extractors correctly (`State`, `WebSocketUpgrade`, `Path`)
2. **tokio** — spawned tasks must be `Send + 'static`; prefer `tokio::select!` over manual polling
3. **tower-http** — CORS and static file serving configured via layers
4. **serde_json** — deserialize with proper error propagation, no silent swallowing
5. **New dependencies** — must be justified, MIT/Apache-2.0 licensed, and added to `deny.toml` allow list if needed

### 🧪 Testing

1. **New features require tests** — unit tests for logic, integration tests for WebSocket flows
2. **Test isolation** — each test creates its own room/state, no shared mutable state between tests
3. **Async tests** — use `#[tokio::test]` with appropriate runtime flavor
4. **Phase 2+ features** — mark with `#[ignore]` and a comment explaining the dependency
5. **Unit tests in-module** — use `#[cfg(test)] mod tests` within the module being tested
6. **Integration tests in `tests/`** — separate directory for cross-module and WebSocket flow tests
7. **Test naming** — prefix with `test_` unless disambiguation is needed

### 📁 File Organization

1. **Backend code** — belongs in `crates/server/src/` or `crates/shared/src/`
2. **Frontend code** — belongs in `frontend/dist/` (no build step, vanilla JS)
3. **Infrastructure** — belongs in `infra/`
4. **Documentation** — belongs in `docs/`
5. **No files in repo root** except config (Cargo.toml, Dockerfile, deny.toml, etc.)
6. **Module tree** — `lib.rs` should be a clean tree of `pub mod` + `pub use` re-exports, not contain implementation logic

## Review Output Format

For each finding, report:

```
### [{SEVERITY}] {title}
**File:** `{path}:{line}`
**Rule:** {category} #{number}
**Issue:** {description}
**Fix:** {suggested resolution}
```

Severity levels:
- 🔴 **BLOCKING** — must fix before merge (safety, correctness, protocol violation, PII leak)
- 🟡 **WARNING** — should fix, but not a merge blocker (style, performance, best practice)
- 🟢 **NOTE** — optional improvement (refactoring opportunity, documentation gap)

## Approval Criteria

- ✅ **APPROVE** if: zero 🔴 findings, and any 🟡 findings are acknowledged
- ❌ **REJECT** if: any 🔴 finding exists
- When rejecting, specify which agent should own the fix (not the original author per Reviewer Rejection Protocol)
