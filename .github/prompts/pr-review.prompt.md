---
mode: "agent"
description: "Review pull requests for beat-stream-rs compliance"
---

# PR Review — beat-stream-rs Governance

You are reviewing a pull request for **beat-stream-rs**, a real-time collaborative beat sequencer built in Rust on Azure Container Apps.

## Review Checklist

### 🦀 Rust Quality

1. **No `.unwrap()` or `.expect()` in production code** — only allowed in tests and examples
2. **No `unsafe` blocks** without explicit safety comment justifying invariants
3. **All public types/functions have `///` doc comments**
4. **Error handling uses `Result<T, E>`** — prefer `thiserror` for typed errors
5. **No blocking I/O in async context** — no `std::fs`, `std::net`, or `thread::sleep` in handlers
6. **Clippy compliance** — code must pass `cargo clippy --all-features --workspace -- -D warnings`
7. **Format compliance** — code must pass `cargo fmt --all -- --check`
8. **No `clone()` without justification** — flag unnecessary allocations
9. **Atomic ordering** — use `Ordering::AcqRel` for shared state, never `Relaxed` for synchronization

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

1. **No hardcoded secrets** — use Key Vault references or environment variables
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

### 📁 File Organization

1. **Backend code** — belongs in `crates/server/src/` or `crates/shared/src/`
2. **Frontend code** — belongs in `frontend/dist/` (no build step, vanilla JS)
3. **Infrastructure** — belongs in `infra/`
4. **Documentation** — belongs in `docs/`
5. **No files in repo root** except config (Cargo.toml, Dockerfile, deny.toml, etc.)

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
- 🔴 **BLOCKING** — must fix before merge (safety, correctness, protocol violation)
- 🟡 **WARNING** — should fix, but not a merge blocker (style, performance, best practice)
- 🟢 **NOTE** — optional improvement (refactoring opportunity, documentation gap)

## Approval Criteria

- ✅ **APPROVE** if: zero 🔴 findings, and any 🟡 findings are acknowledged
- ❌ **REJECT** if: any 🔴 finding exists
- When rejecting, specify which agent should own the fix (not the original author per Reviewer Rejection Protocol)
