# beat-stream-rs — Product Requirements Document

> **A collaborative real-time beat sequencer built in Rust, deployed on Azure Container Apps.**

| Field | Value |
|-------|-------|
| **Product** | beat-stream-rs |
| **Repository** | [github.com/diberry/beat-stream-rs](https://github.com/diberry/beat-stream-rs) |
| **One-liner** | A fidget toy that happens to make music. |
| **Stack** | Rust · Axum · WebSockets · Azure Container Apps · Cosmos DB |
| **Timing** | Aligned with the Azure SDK for Rust reaching GA |

---

## 1. Overview

**beat-stream-rs** is a browser-based beat sequencer that lets anyone — musician or not — click a grid and immediately hear a rhythm. It runs as a single Rust container on Azure Container Apps, streams state over WebSockets, and persists rooms in Cosmos DB. One player can jam solo; two or more can join the same room and edit the grid together in real time.

The core design philosophy is simple:

> *"It's a fidget toy that happens to make music."*

The experience is designed for a single player who knows absolutely nothing about music, but it scales gracefully to multiplayer collaboration.

---

## 2. Problem Statement

### Beat-making tools are intimidating

Digital Audio Workstations (DAWs) greet new users with hundreds of knobs, tracks, and music-theory terminology. The learning curve keeps casual users from ever making a sound.

### No "click and hear something cool" experience exists on the web

Most browser-based music tools still assume the user knows what a hi-hat is or how a 4/4 time signature works. There is no equivalent of a fidget spinner for rhythm — something you pick up, interact with instantly, and put down when you're done.

### Multiplayer music creation is locked behind expensive software

Collaborative music tools either cost money, require accounts, or demand that every participant already understands music production.

**beat-stream-rs solves all three problems** by stripping beat-making down to a colorful grid of tappable cells, hiding every piece of music theory behind smart defaults, and making multiplayer as simple as sharing a link.

---

## 3. Target Users

| Persona | Description | Needs |
|---------|-------------|-------|
| 🎯 **Primary — The Fidgeter** | Non-musician who wants to play with sound. Clicks things to see what happens. Zero music vocabulary. | Instant feedback, no sign-up, no jargon, satisfying visually and sonically. |
| 🎸 **Secondary — The Jammer** | Musician who wants a quick, low-friction way to loop ideas with a friend. | Shareable rooms, real-time sync, responsive grid. |
| 🚫 **Anti-persona — The Producer** | Serious music producer looking for a DAW replacement. | *Not our user.* We will never add multi-track mixing, audio recording, or MIDI export to the MVP. |

---

## 4. UX Philosophy & Design Principles

### No music terminology

Traditional labels are replaced with playful, intuitive names and emoji:

| Track | Traditional Name | beat-stream-rs Label |
|-------|-----------------|----------------------|
| 1 | Kick drum | **BOOM** 💥 |
| 2 | Snare / clap | **CRACK** ⚡ |
| 3 | Hi-hat | **TICK** ✨ |
| 4 | Rim / snap | **SNAP** 👏 |

The user never sees the words "kick," "snare," "hi-hat," "BPM," or "time signature."

### Progressive disclosure

The grid starts with **one row** (BOOM 💥). Additional rows reveal themselves organically as the user interacts:

1. **0 s** — 1 row visible (BOOM 💥)
2. **~10 s** of interaction — 2nd row fades in (CRACK ⚡)
3. **~20 s** — 3rd row (TICK ✨)
4. **~30 s** — 4th row (SNAP 👏)

This prevents the "blank canvas" paralysis and teaches by doing.

### Never a blank grid

When a new room is created, the grid is pre-populated with one of **six starter patterns**, chosen randomly:

| Pattern | Vibe | Description |
|---------|------|-------------|
| **Chill** | Laid-back | Sparse kick, light hats |
| **Bounce** | Head-noddy | Syncopated kick + snare |
| **Pulse** | Steady | Four-on-the-floor kick |
| **Sparse** | Minimal | 2-3 hits, lots of space |
| **Chaos** | Chaotic fun | Dense, random-feeling |
| **Heartbeat** | Organic | Kick mimics a heartbeat |

### Smart constraints (invisible to the user)

These musical guardrails run silently to make everything sound good:

- **Anchor kick on beat 1** — The first step of BOOM is always active. This grounds every pattern.
- **Swing humanization** — Micro-timing offsets on even steps (~15 ms) to avoid a robotic feel.
- **Volume shaping** — Slight velocity variation so repeated hits feel alive.
- **BPM range: 60–140** — Fast enough to be energetic, slow enough to stay groovy.

### Solo features

| Feature | Description |
|---------|-------------|
| 🎲 **Surprise Me** | Generates a new random pattern using the smart constraints. One tap, fresh beat. |
| 🌊 **Mood buttons** | Three buttons — **CHILL** / **HYPE** / **WEIRD** — that shift the pattern's density, BPM, and sound character. |
| 🔓 **Hidden combos** | Secret interactions (e.g., tap BOOM 4× fast → "Earthquake" pattern). Discovered combos show a toast notification. |

### Gamification

- **Per-cell micro-animations** — Each cell pulses or ripples when toggled.
- **Streak glow** — Activating 4+ cells in a row triggers a glow effect on the row.
- **Discovered combo toasts** — Toast notifications celebrate hidden combo discoveries.

---

## 5. Feature Set (MVP)

### ✅ In Scope

| Feature | Details |
|---------|---------|
| 16-step grid | 16 columns representing one musical bar |
| 4 sound tracks | BOOM 💥, CRACK ⚡, TICK ✨, SNAP 👏 |
| Real-time WebSocket sync | All users in a room see toggles instantly |
| Room creation & joining | `POST /api/rooms` → share link |
| Solo play | Single-player works without any network dependency for audio |
| Starter patterns | 6 pre-built patterns, randomly assigned |
| Progressive reveal | Rows appear over ~30 s of interaction |
| 🎲 Surprise Me button | Random pattern generation with smart constraints |
| 🌊 Mood buttons | CHILL / HYPE / WEIRD pattern modifiers |
| Basic mobile layout | Responsive CSS Grid, tap-friendly cells |

### ❌ Out of Scope (Post-MVP)

| Feature | Reason |
|---------|--------|
| User accounts / auth | Adds friction; rooms are anonymous by design |
| Saved patterns gallery | Requires `patterns` container + UI; deferred |
| Event Hubs analytics | Not needed for core experience |
| Custom sounds | Requires audio upload pipeline |
| Sharing / export | Audio rendering adds complexity |
| Public room discovery | Needs moderation strategy first |

---

## 6. Architecture

```
┌─────────────────────────────────────────────────────┐
│              Azure Container Apps                    │
│  ┌───────────────────────────────────────────────┐  │
│  │            Axum Server (Rust)                  │  │
│  │                                               │  │
│  │  ┌──────────┐  ┌──────────┐  ┌────────────┐  │  │
│  │  │  Static  │  │  REST    │  │ WebSocket  │  │  │
│  │  │  Files   │  │  API     │  │  Rooms     │  │  │
│  │  │(tower-http) │  │         │  │(broadcast) │  │  │
│  │  └──────────┘  └──────────┘  └────────────┘  │  │
│  │                      │              │         │  │
│  └──────────────────────┼──────────────┼─────────┘  │
│                         │              │            │
└─────────────────────────┼──────────────┼────────────┘
                          │              │
                ┌─────────▼──┐    ┌──────▼───────┐
                │  Cosmos DB │    │  Key Vault   │
                │ (serverless)│    │  (secrets)   │
                └────────────┘    └──────────────┘
```

### Key decisions

| Decision | Rationale |
|----------|-----------|
| **Single Axum container** | One process serves static files, REST API, and WebSockets. No inter-service latency. |
| **`tokio::sync::broadcast` per room** | In-process fan-out with **<1 ms latency**. No external message broker needed for real-time. |
| **NOT Event Hubs for real-time** | Event Hubs adds 50–500 ms latency — unacceptable for music. Event Hubs is reserved for post-MVP analytics and edit replay. |
| **Cosmos DB serverless** | Pay-per-request pricing. Rooms have a 24-hour TTL, so storage stays minimal. |
| **Key Vault at startup** | Secrets (Cosmos connection string, etc.) are fetched once at boot via `DefaultAzureCredential`. |
| **Static frontend from same container** | `tower-http::ServeDir` eliminates the need for a CDN or separate static host at MVP scale. |

---

## 7. Technology Stack

### Rust Crates (Server)

| Crate | Version | Purpose |
|-------|---------|---------|
| `axum` | 0.8 (`ws` feature) | HTTP routing + WebSocket upgrade |
| `tokio` | 1 (`full`) | Async runtime, broadcast channels |
| `tower-http` | 0.6 | CORS middleware, static file serving |
| `serde` / `serde_json` | latest | JSON serialization / deserialization |
| `uuid` | 1 (`v4`) | Room and session ID generation |
| `azure_identity` | 0.22 | `DefaultAzureCredential` for Azure auth |
| `azure_data_cosmos` | 0.22 | Cosmos DB client for room persistence |
| `azure_security_keyvault_secrets` | 0.22 | Secret retrieval at startup |
| `azure_messaging_eventhubs` | 0.22 | **Post-MVP only** — analytics event stream |

### Frontend (Served Statically)

| Technology | Purpose |
|------------|---------|
| **Vanilla JS** | No framework overhead — keep the bundle tiny |
| **Web Audio API** | Low-level audio scheduling for precise timing |
| **Tone.js** | Higher-level audio scheduling, transport, synths |
| **CSS Grid** | Responsive beat grid layout |

> **Why no framework?** The entire frontend is a single grid with buttons. React/Vue/Svelte would add bundle size and complexity for zero benefit. Vanilla JS + Tone.js keeps the initial load under 50 KB gzipped.

---

## 8. Data Model (Cosmos DB)

**Database**: `beatstream`

### Container: `rooms`

**Partition key**: `/id`

```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "pattern": {
    "tracks": [
      { "name": "BOOM", "emoji": "💥", "steps": [1,0,0,0, 1,0,0,0, 1,0,0,0, 1,0,0,0] },
      { "name": "CRACK", "emoji": "⚡", "steps": [0,0,0,0, 1,0,0,0, 0,0,0,0, 1,0,0,0] },
      { "name": "TICK", "emoji": "✨", "steps": [1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1] },
      { "name": "SNAP", "emoji": "👏", "steps": [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,1,0] }
    ]
  },
  "bpm": 120,
  "active_users": 2,
  "created_at": "2025-07-15T10:30:00Z",
  "ttl": 86400
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` (UUID v4) | Unique room identifier, also the partition key |
| `pattern.tracks[]` | `array` | Array of 1–4 track objects |
| `pattern.tracks[].name` | `string` | Display label (BOOM, CRACK, TICK, SNAP) |
| `pattern.tracks[].emoji` | `string` | Emoji icon for the track |
| `pattern.tracks[].steps` | `int[16]` | Binary array: `1` = active, `0` = inactive |
| `bpm` | `int` | Beats per minute (60–140) |
| `active_users` | `int` | Count of connected WebSocket clients |
| `created_at` | `string` (ISO 8601) | Room creation timestamp |
| `ttl` | `int` | Time-to-live in seconds (24 hours). Cosmos DB auto-deletes expired rooms. |

### Container: `patterns` (Post-MVP)

**Partition key**: `/room_id`

Reserved for the saved patterns gallery feature. Stores named snapshots of room patterns for later recall.

---

## 9. API Design

### REST Endpoints

| Method | Path | Description | Response |
|--------|------|-------------|----------|
| `GET` | `/` | Serve static frontend (index.html) | `200` HTML |
| `GET` | `/api/health` | Health check | `200 { "status": "ok" }` |
| `POST` | `/api/rooms` | Create a new room with a random starter pattern | `201 { "room_id": "uuid" }` |
| `GET` | `/api/rooms/:id` | Get current room state | `200 { room document }` |
| `WS` | `/api/rooms/:id/ws` | WebSocket upgrade for real-time sync | `101 Switching Protocols` |

### WebSocket Protocol

All messages are JSON over a single WebSocket connection per client.

#### Client → Server

**Toggle a cell:**
```json
{ "type": "toggle", "track": 0, "step": 7 }
```

**Change BPM:**
```json
{ "type": "bpm", "value": 130 }
```

#### Server → Client

**Full state sync** (sent on connection and periodically):
```json
{
  "type": "state",
  "pattern": {
    "tracks": [
      { "name": "BOOM", "emoji": "💥", "steps": [1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0] }
    ]
  },
  "bpm": 120
}
```

**Broadcast a toggle** (relayed to all other clients in the room):
```json
{ "type": "toggle", "track": 0, "step": 7, "user": "anon-xyz" }
```

> **Note:** Audio playback is entirely client-side. The server never sends audio data — only grid state. Each client runs its own local audio loop via Tone.js, ensuring zero-latency playback regardless of network conditions.

---

## 10. Deployment

### Infrastructure

| Component | Configuration |
|-----------|--------------|
| **Azure Container Apps** | Consumption plan, `beatstream` environment |
| **Min replicas** | 1 (avoids cold-start latency) |
| **Max replicas** | 5 (scales on HTTP concurrency) |
| **Container image** | < 15 MB (statically-linked Rust binary + Azure SDK deps + static assets) |
| **Container Registry** | Azure Container Registry (ACR) |
| **Cosmos DB** | Serverless tier, `beatstream` database |
| **Key Vault** | Stores Cosmos connection string, future secrets |

### CI/CD Pipeline

```
  git push main
       │
       ▼
  GitHub Actions
       │
       ├── cargo fmt --check
       ├── cargo clippy
       ├── cargo test
       ├── docker build → ACR
       └── az containerapp update → Container Apps
```

### GitHub Actions Authentication

**Use OIDC (OpenID Connect)** — no stored secrets for Azure access.

| Component | Configuration |
|-----------|--------------|
| **Identity provider** | GitHub Actions (federated) |
| **Azure side** | App Registration with Federated Credential for `repo:diberry/beat-stream-rs:ref:refs/heads/main` |
| **Workflow auth** | `azure/login@v2` with `client-id`, `tenant-id`, `subscription-id` (all non-secret) |
| **No secrets stored** | No `AZURE_CLIENT_SECRET` in GitHub Secrets — OIDC token is minted per-run |
| **ACR push** | `az acr login` after OIDC auth (inherits token) |

> **Why OIDC over stored credentials?** Secrets can leak, expire, and require rotation. OIDC tokens are short-lived (5 min), auto-rotated, and scoped to the specific repo + branch. Zero maintenance.

### Cosmos DB Cost Model

| Scenario | Estimated RU/s | Monthly Cost (serverless) |
|----------|---------------|--------------------------|
| **Idle** (no traffic) | 0 | $0 |
| **Low** (10 rooms/day, 5 users avg) | ~50 RU/day | < $1/month |
| **Medium** (100 rooms/day, 10 users avg) | ~500 RU/day | ~$3/month |
| **High** (1,000 rooms/day, 20 users avg) | ~5,000 RU/day | ~$15/month |
| **Burst** (viral moment, 10K rooms) | ~50,000 RU/day | ~$50/month |

**RU breakdown per operation:**
- Create room: ~6 RU (single write, ~1 KB document)
- Read room state: ~1 RU (point read by ID)
- Update room state: ~6 RU (replace operation)
- TTL deletion: 0 RU (handled by Cosmos internally)

> **Cost guardrail:** Cosmos DB serverless caps at 5,000 RU/s burst. At sustained high traffic, consider switching to provisioned autoscale (400–4,000 RU/s, ~$23/month baseline). Alert at $20/month spend.

### Monitoring & Observability

| Layer | Tool | What It Monitors |
|-------|------|-----------------|
| **Application** | Azure Application Insights (post-MVP) | Request latency, WebSocket connection count, error rate, custom events (room created, pattern toggled) |
| **Infrastructure** | Azure Monitor (built-in) | Container CPU/memory, replica count, restart events |
| **Availability** | Container Apps health probes | `/api/health` liveness check every 10s |
| **Cost** | Azure Cost Management alerts | Budget alert at $20/month, anomaly detection |
| **Errors** | Structured logging (`tracing` crate) | JSON logs to Container Apps log stream, filterable by room_id |

**MVP logging** (Phase 0–2): `tracing` + `tracing-subscriber` with JSON formatting. Logs ship to Container Apps log stream automatically.

**Post-MVP observability** (Phase 4+): Application Insights SDK integration for distributed tracing, custom metrics (rooms active, WebSocket connections, toggle rate), and availability tests.

### Scaling Strategy

| Metric | Threshold | Action |
|--------|-----------|--------|
| HTTP concurrency | > 50 per replica | Scale out (up to 5 replicas) |
| HTTP concurrency | < 10 per replica | Scale in (down to 1 replica) |
| Idle | Always | 1 replica stays warm |

> **Scaling numbers:** Each replica supports approximately **200 concurrent WebSocket connections** (limited by tokio task overhead and broadcast channel fan-out). At 50 users per room average, that's ~4 active rooms per replica. The 5-replica max supports ~1,000 concurrent users / ~20 active rooms.

> **Important:** Because rooms use in-process `broadcast` channels, all users in a room must hit the same replica. At MVP scale (< 5 replicas), this is naturally handled by sticky sessions on the WebSocket connection. Azure Service Bus topics handle cross-replica sync beyond 5 replicas (see Section 15).

### WebSocket Reconnection

Clients implement automatic reconnection to handle transient network issues:

| Event | Client Behavior |
|-------|----------------|
| Connection dropped | Retry with exponential backoff: 1s → 2s → 4s → 8s → 16s (max) |
| Reconnection success | Server sends full `state` message; client replaces local state |
| Max retries exceeded (5) | Show "Connection lost" banner with manual retry button |
| Tab becomes visible | Immediately attempt reconnection if disconnected |
| Server sends `ping` | Client responds with `pong` within 10s (else server closes) |

### Infrastructure-as-Code Decision

**Chosen: Bicep** (Azure-native, zero dependencies beyond Azure CLI)

| Factor | Bicep | Terraform |
|--------|-------|-----------|
| Azure-native | ✅ First-class | ⚠️ Provider lag |
| Dependencies | Azure CLI only | Terraform binary + azurerm provider |
| Learning curve | Lower (ARM JSON → Bicep) | Higher (HCL + state management) |
| State management | None (Azure is the state) | Remote state backend required |
| Community for Azure | Growing, Microsoft-maintained | Larger, but generic |

Rationale: beat-stream-rs is all-Azure with no multi-cloud requirements. Bicep's zero-state-file model and direct ARM integration reduce operational complexity for a single-developer project.

---

## 11. Risks & Mitigations

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| 1 | **Client audio sync across browsers** — Different browsers have different audio scheduling precision. Two users may hear slightly different timing. | 🔴 High | Each client plays its own local audio loop synced to its own clock. This is the industry-standard approach (used by Splice, BandLab, etc.). We sync *state*, not *audio*. |
| 2 | **Azure SDK for Rust beta breakage** — The `azure_*` crates are pre-GA and may introduce breaking changes. | 🔴 High | Pin exact crate versions in `Cargo.toml`. Maintain a thin abstraction layer so we can fall back to raw `reqwest` HTTP calls against Azure REST APIs if a crate breaks. |
| 3 | **Container Apps WebSocket idle timeout** — Azure Container Apps may close idle WebSocket connections. | 🟡 Medium | Implement ping/pong frames every 30 seconds. Configure the maximum idle timeout on the Container App ingress. |
| 4 | **Cold-start latency** — First request after scale-to-zero takes several seconds. | 🟡 Medium | Set `minReplicas: 1` so there is always a warm instance. The Rust binary is ~15 MB and starts in < 500 ms. |
| 5 | **Cosmos DB cost at scale** — Unexpected traffic could drive up RU consumption. | 🟢 Low | Serverless tier charges ~$0.25 per million RUs. Rooms auto-delete via TTL after 24 hours. At MVP traffic levels, monthly cost should stay under $5. |
| 6 | **Multi-replica state divergence** — If scaled beyond 1 replica, rooms are isolated per process. | 🟡 Medium | MVP runs 1 replica. Post-MVP adds Azure Service Bus topics for cross-replica sync (see Section 15 — keeps the stack all-Azure, no Redis). |
| 7 | **WebSocket state conflicts** — Two users toggle the same cell simultaneously. | 🟡 Medium | Last-write-wins with server timestamp. Server is the authority — conflicting toggles resolve to the latest `toggle` message received. Clients optimistically apply local changes and reconcile on the next `state` broadcast (every 500 ms). |
| 8 | **WebSocket abuse / spam** — A single client sends unlimited toggle messages. | 🟡 Medium | Server-side rate limiting: max 20 messages/second per client. Exceeding the limit triggers a `{ "type": "rate_limited" }` warning; persistent abuse closes the connection. |

---

## 12. Timeline (MVP)

**Total estimate:** 8–10 working days for a single developer.

| Day | Focus | Deliverables |
|-----|-------|-------------|
| **1–2** | Project scaffolding | Cargo workspace, Axum server, `/api/health` endpoint, Dockerfile, CI pipeline to ACR |
| **3–4** | WebSocket rooms | Room creation, WebSocket upgrade, `broadcast` channels, in-memory room state, toggle/BPM messages |
| **5–6** | Cosmos DB integration | `azure_data_cosmos` client, room persistence, TTL, Key Vault secret loading |
| **7–8** | Frontend | HTML/CSS grid, Tone.js audio engine, WebSocket client, starter patterns, cell animations |
| **9–10** | Polish & delight | Progressive row reveal, Surprise Me button, mood buttons, hidden combos, streak glow, mobile layout, end-to-end testing |

```
Week 1                          Week 2
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│ D1  │ D2  │ D3  │ D4  │ D5  │ D6  │ D7  │ D8  │ D9  │ D10 │
│Scaff│olding│ WS  │Rooms│Cosmos│ DB  │Front│ end │Poli │ sh  │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
```

---

## 13. Success Metrics

| Metric | Target | How We Measure |
|--------|--------|---------------|
| **Time to first beat** | < 3 seconds | Timestamp from page load to first audio output (client-side analytics) |
| **Solo session length** | > 2 minutes average | Duration between WebSocket connect and disconnect for single-user rooms |
| **Return rate** | > 20% within 7 days | Returning browser fingerprints (anonymous, no accounts) |
| **Multiplayer adoption** | > 30% of sessions | Percentage of rooms that have `active_users > 1` at any point |
| **Grid interaction rate** | > 10 toggles per session | Average toggle events per WebSocket session |

> **Measurement note:** MVP metrics are collected via server-side WebSocket event counting and Cosmos DB room documents. Post-MVP, Event Hubs enables richer analytics pipelines.

---

## 13.5. Test Strategy

Testing is integral from Phase 0. Every phase has a clear testing deliverable — no feature ships without coverage.

### Test Layers

| Layer | Tool | Scope | When |
|-------|------|-------|------|
| **Unit tests** | `cargo test` | Pure logic: pattern generation, smart constraints, BPM validation, rate limiter | Phase 1+ (every PR) |
| **Integration tests** | `cargo test --features integration` | Azure SDK interactions: Cosmos CRUD, Key Vault fetch, Service Bus pub/sub | Phase 2+ (requires live Azure resources) |
| **WebSocket tests** | `tokio-tungstenite` + in-process server | Connection upgrade, message relay, broadcast fan-out, rate limiting, reconnection | Phase 1+ (every PR) |
| **API tests** | `axum::test` helpers | REST endpoints: room CRUD, health check, error responses | Phase 1+ (every PR) |
| **Frontend smoke** | Playwright (post-MVP) | Page loads, grid renders, audio plays, WebSocket connects | Phase 3+ (manual until then) |
| **Load tests** | `k6` or `drill` | WebSocket concurrency: 200 connections/replica, message throughput, latency P95 | Phase 2+ (pre-launch) |

### Coverage Targets

| Phase | Target | Focus |
|-------|--------|-------|
| Phase 1 | > 80% line coverage on server crate | Core logic: room state, broadcast, pattern generation |
| Phase 2 | > 70% on integration paths | Cosmos operations, Key Vault startup, error recovery |
| Phase 3 | Manual acceptance tests | UX flows: progressive reveal, mood buttons, combos |
| Phase 4 | > 60% on new service integrations | Blob, Service Bus, Queue interactions |

### CI Gate

Every PR must pass:
1. `cargo fmt --check` (formatting)
2. `cargo clippy -- -D warnings` (lints)
3. `cargo test` (unit + WebSocket tests)
4. `cargo deny check` (license + advisory)

Integration tests run on `main` branch merges only (require Azure credentials via OIDC).

---

## 14. Post-MVP Roadmap

| Priority | Feature | Dependencies |
|----------|---------|-------------|
| 🥇 | **Saved patterns gallery** | `patterns` Cosmos container, gallery UI |
| 🥇 | **Event Hubs integration** | `azure_messaging_eventhubs` crate, analytics pipeline |
| 🥇 | **Application Insights** | `tracing-opentelemetry` + AI SDK, custom metrics for rooms/connections |
| 🥈 | **Public room discovery** | Room listing API, moderation strategy |
| 🥈 | **Custom sound packs** | Audio file upload, Blob Storage |
| 🥈 | **Accessibility** | Screen reader support, full keyboard navigation, ARIA labels (addressed in Phase 3) |
| 🥉 | **Share / export loops** | Server-side audio rendering or client-side Tone.js offline render |
| 🥉 | **Mobile-optimized touch** | Touch event handling, gesture support, haptic feedback |
| 🏅 | **Multi-replica sync** | Azure Service Bus topics for cross-replica room state |

---

## 15. Services Expansion (Azure SDK for Rust)

The Azure SDK for Rust provides crates across identity, data, messaging, storage, and security. The MVP uses three (`azure_identity`, `azure_data_cosmos`, `azure_security_keyvault_secrets`) with a fourth (`azure_messaging_eventhubs`) planned for post-MVP analytics. The full SDK surface area offers natural expansion paths that keep beat-stream-rs on a single technology stack — no sidecar services, no polyglot dependencies.

The table below maps every remaining SDK crate to a concrete beat-stream-rs feature, prioritized by user value.

### New Crates Beyond MVP

| Crate | Feature It Enables | Priority | When to Add |
|-------|-------------------|----------|-------------|
| `azure_storage_blob` | **Custom sound packs & audio export.** Users upload WAV/MP3 files to Blob Storage; the frontend fetches them via CDN URLs. Also powers the share/export feature — rendered loops are written to Blob and returned as downloadable links. | 🥇 | First post-MVP sprint |
| `azure_messaging_servicebus` | **Cross-replica room sync.** Service Bus topics/subscriptions replace the Redis pub/sub idea from the risk table — keeping the stack all-Azure. A `room-events` topic fans out toggle/BPM messages across replicas. Also enables: room expiry notifications, moderation pipelines, and webhook triggers for integrations. | 🥇 | When scaling beyond 1 replica |
| `azure_storage_queue` | **Async job queue.** Fire-and-forget tasks (audio rendering, pattern export, abuse-detection) are enqueued without blocking the WebSocket loop. A background `tokio::spawn` worker polls the queue and processes jobs. | 🥈 | After export feature lands |
| `azure_security_keyvault_keys` | **Signed shareable URLs.** Room links are signed with Key Vault-managed keys so they can't be forged or enumerated. Enables short-lived signed URLs for private rooms and time-boxed invite links. | 🥈 | After public room discovery |
| `azure_security_keyvault_certificates` | **mTLS between replicas.** In a multi-replica deployment, certificates from Key Vault authenticate inter-container traffic. Prevents unauthorized containers from joining the Service Bus topic. | 🥉 | Production hardening phase |

### Full SDK Crate Map

For reference, the complete set of `azure-sdk-for-rust` crates and their beat-stream-rs status:

| Service Area | Crate | Status |
|-------------|-------|--------|
| Core | `azure_core` | ✅ Implicit dependency |
| Identity | `azure_identity` | ✅ MVP |
| Cosmos DB | `azure_data_cosmos` | ✅ MVP |
| Key Vault — Secrets | `azure_security_keyvault_secrets` | ✅ MVP |
| Event Hubs | `azure_messaging_eventhubs` | 📋 Planned (post-MVP analytics) |
| Storage — Blob | `azure_storage_blob` | 🔜 Post-MVP (sound packs, export) |
| Service Bus | `azure_messaging_servicebus` | 🔜 Post-MVP (cross-replica sync) |
| Storage — Queue | `azure_storage_queue` | 🔜 Post-MVP (async jobs) |
| Key Vault — Keys | `azure_security_keyvault_keys` | 🔜 Post-MVP (signed URLs) |
| Key Vault — Certificates | `azure_security_keyvault_certificates` | 🔜 Post-MVP (mTLS) |

### Showcase Value

> beat-stream-rs naturally incorporates **8+ Azure SDK for Rust crates** in a single, real-world application — spanning identity, database, messaging, storage, and security. This makes it a compelling end-to-end showcase for the SDK's breadth, demonstrating that a non-trivial production app can run entirely on Azure services with a pure-Rust dependency chain.

---

## 16. Implementation Phases

Development is organized into five sequential phases. Each phase produces a deployable increment — nothing stays on a branch for more than one phase.

### Phase 0: Infrastructure & Repo Configuration

> **Goal:** Set up everything needed before writing application code.

| Deliverable | Details |
|-------------|---------|
| **Repo structure** | Cargo workspace layout: `/crates/` (server, shared types), `/infra/` (Bicep), `/frontend/` (static assets), `/docs/` (ADRs, guides) |
| **Cargo workspace** | Root `Cargo.toml` with `[workspace]` members, shared dependency versions via `[workspace.dependencies]` |
| **Infrastructure-as-code** | **Bicep** (Azure-native, no state file): Container Apps environment, Cosmos DB serverless account + `beatstream` database, Key Vault, Azure Container Registry, OIDC federated credential |
| **CI/CD pipeline** | GitHub Actions workflow: `cargo fmt --check` → `cargo clippy` → `cargo test` → `cargo deny check` → `docker build` → push to ACR → `az containerapp update`. Auth via OIDC (no stored secrets). |
| **Dev container** | `.devcontainer/devcontainer.json` with Rust toolchain, Azure CLI, Azure Developer CLI (`azd`), `cargo-deny`, and VS Code extensions |
| **Environment config** | `.env.example` with all required variables, documentation for Azure connection setup |
| **Linting & formatting** | `rustfmt.toml`, `.clippy.toml`, `deny.toml` (license allow-list, advisory database, ban duplicates) |
| **Container setup** | Multi-stage Dockerfile: `rust:slim` build stage → `gcr.io/distroless/cc` or `scratch` runtime stage for a < 15 MB image |
| **Test foundation** | `cargo test` runs unit + WebSocket tests in CI. Integration test feature flag (`--features integration`) for Azure-dependent tests. |
| **ADR-0000: IaC choice** | Document Bicep selection rationale (see Deployment → Infrastructure-as-Code Decision) |

### Phase 1: Core Beat Engine (MVP)

> **Goal:** Single-player beat sequencer running locally — land on page, hear a beat in < 3 seconds.

| Deliverable | Details |
|-------------|---------|
| **Axum web server** | `axum 0.8` with `tower-http` serving static files from `/frontend/dist/` |
| **Beat grid data model** | In-memory `RoomState` struct: 4 tracks × 16 steps, BPM, active-user count. No persistence yet. |
| **WebSocket endpoint** | `GET /api/rooms/:id/ws` → upgrade to WS, send full `state` message on connect, relay `toggle`/`bpm` messages |
| **Frontend grid UI** | Vanilla JS + CSS Grid: 16 columns × 1–4 rows, emoji labels (BOOM 💥, CRACK ⚡, TICK ✨, SNAP 👏), per-cell toggle with micro-animation |
| **Audio engine** | Tone.js `Transport` + `Sampler`: schedules samples on the client clock, loops the 16-step pattern |
| **Starter patterns** | 6 built-in patterns (Chill, Bounce, Pulse, Sparse, Chaos, Heartbeat) — one assigned randomly on room creation |
| **BPM control** | Slider or +/− buttons, range 60–140, synced over WebSocket |
| **Health endpoint** | `GET /api/health` → `{ "status": "ok" }` for Container Apps probes |

### Phase 2: Persistence & Rooms

> **Goal:** Multi-player rooms with saved state — share a link, jam together.

| Deliverable | Details |
|-------------|---------|
| **Cosmos DB integration** | `azure_data_cosmos` client: CRUD on `rooms` container, partition key `/id`, 24-hour TTL auto-cleanup |
| **Key Vault integration** | `azure_security_keyvault_secrets` loads Cosmos connection string (and future secrets) once at startup via `DefaultAzureCredential`, cached in-process |
| **Room creation** | `POST /api/rooms` → generates UUID, persists starter pattern to Cosmos, returns `{ "room_id": "..." }` |
| **Room joining** | `GET /api/rooms/:id` → fetches state from Cosmos (or in-memory cache). Shareable URL: `https://<host>/#room=<id>` |
| **Multi-user broadcast** | One `tokio::sync::broadcast` channel per active room. Toggle/BPM messages fan out to all connected clients in < 1 ms. |
| **State reconciliation** | On WebSocket connect: client receives full `state` message. On disconnect: `active_users` decremented. Last user out → room state flushed to Cosmos. |

### Phase 3: Polish & Accessibility

> **Goal:** Solo-play delight features, progressive UX, and foundational accessibility — make it feel like a toy, not a tool, and ensure it's usable by everyone.

| Deliverable | Details |
|-------------|---------|
| **Progressive row reveal** | Grid starts with 1 row (BOOM 💥). Rows 2–4 fade in after ~10 s, ~20 s, ~30 s of interaction. CSS transitions, no layout shift. |
| **🎲 Surprise Me button** | Generates a random pattern using smart constraints (kick anchored on beat 1, density varies, BPM randomized within 60–140) |
| **🌊 Mood buttons** | Three buttons — **CHILL** / **HYPE** / **WEIRD** — that adjust pattern density, BPM range, and sample selection |
| **Hidden combos** | Secret interaction triggers (e.g., tap BOOM 4× fast → "Earthquake" pattern). Discovered combos show a toast with 🔓 emoji. |
| **Gamification** | Streak glow (4+ consecutive cells), per-cell ripple animations, combo discovery counter |
| **Mobile touch** | Touch-optimized cell size (min 44×44 px), prevent scroll-on-tap, responsive grid breakpoints |
| **Idle animations** | Subtle cell pulse on the current playback step, ambient glow on active rows |
| **Keyboard navigation** | Full grid navigation with arrow keys, Space/Enter to toggle cells, Tab to switch tracks. Focus indicators on all interactive elements. |
| **ARIA labels** | Screen reader support: grid cells announce "BOOM step 3, active" / "TICK step 7, inactive". Live region for playback status. |
| **Reduced motion** | `@media (prefers-reduced-motion)` — disable cell animations, replace with opacity changes. Audio still works. |
| **High contrast** | Active cells use high-contrast colors that pass WCAG AA (4.5:1 ratio) in both light and dark modes. |

### Phase 4: Services Expansion

> **Goal:** Integrate additional Azure SDK crates from [Section 15](#15-services-expansion-azure-sdk-for-rust) to unlock post-MVP features.

| Sprint | Crate | Feature |
|--------|-------|---------|
| **4a** | `azure_storage_blob` | Custom sound packs — upload WAV/MP3 to Blob Storage, serve via CDN URLs. Export rendered loops as downloadable audio files. |
| **4b** | `azure_messaging_servicebus` | Cross-replica room sync — Service Bus topics/subscriptions replace the single-replica constraint. Room expiry notifications and moderation pipeline. |
| **4c** | `azure_storage_queue` | Async job queue — audio rendering, pattern export, and abuse-detection tasks processed off the WebSocket hot path. |
| **4d** | `azure_security_keyvault_keys` | Signed shareable URLs — room invite links signed with KV-managed keys, time-boxed and tamper-proof. |
| **4e** | `azure_security_keyvault_certificates` | mTLS between replicas — certificate-based inter-container auth for the multi-replica Service Bus topology. |

### Phase Summary

```
Phase 0          Phase 1          Phase 2          Phase 3          Phase 4
┌────────────┐   ┌────────────┐   ┌────────────┐   ┌────────────┐   ┌────────────┐
│ Infra &    │   │ Core Beat  │   │ Persistence│   │ Polish &   │   │ Services   │
│ Repo Setup │──▶│ Engine     │──▶│ & Rooms    │──▶│ Accessi-   │──▶│ Expansion  │
│            │   │ (MVP)      │   │            │   │ bility     │   │            │
│ ~2 days    │   │ ~4 days    │   │ ~3 days    │   │ ~3 days    │   │ Ongoing    │
└────────────┘   └────────────┘   └────────────┘   └────────────┘   └────────────┘
```

---

## 17. Developer Experience & AI Configuration

beat-stream-rs is built with AI-assisted development in mind. Every contributor — human or AI — should have the context needed to make good decisions without reading the entire codebase.

### Copilot Instructions File

`.github/copilot-instructions.md` provides project context to GitHub Copilot and other AI assistants:

```markdown
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
- Cosmos DB for persistence (rooms, patterns, user prefs)
- Key Vault for secrets (loaded once at startup, cached)

## Testing
- Unit tests: `cargo test`
- Integration tests: `cargo test --features integration` (requires Azure resources)
- Frontend: manual testing via browser (no JS test framework yet)
```

### Repository Configuration Files

| File | Purpose |
|------|---------|
| `.github/copilot-instructions.md` | AI assistant context (shown above) |
| `.github/CONTRIBUTING.md` | Local dev setup, test commands, deployment guide, PR conventions |
| `rust-toolchain.toml` | Pins Rust version (e.g., `channel = "1.82"`) for reproducible builds across machines and CI |
| `.cargo/config.toml` | Workspace-wide Cargo settings: default target, linker flags, `[net]` retry config |
| `deny.toml` | `cargo-deny` configuration: license allow-list, RustSec advisory database, ban duplicate crate versions |
| `.devcontainer/devcontainer.json` | VS Code dev container: Rust toolchain + `rust-analyzer` + Azure CLI + `azd` + `cargo-deny` + Docker-in-Docker |
| `.env.example` | Template for local environment variables: Cosmos connection, Key Vault URL, Container Apps endpoint |

### AI Context Files

These files exist specifically to give AI assistants (and new contributors) fast orientation:

| File | Contents |
|------|----------|
| `ARCHITECTURE.md` | High-level system diagram, data flow from client → WebSocket → broadcast → Cosmos, component responsibilities, scaling boundaries |
| `docs/adr/0001-websocket-over-eventhubs.md` | Why we chose in-process broadcast over Event Hubs for real-time sync (latency analysis) |
| `docs/adr/0002-no-frontend-framework.md` | Why vanilla JS + Tone.js instead of React/Vue (bundle size, complexity trade-off) |
| `docs/adr/0003-cosmos-serverless.md` | Why serverless Cosmos over provisioned throughput (cost model, TTL strategy) |
| `docs/adr/0004-service-bus-over-redis.md` | Why Azure Service Bus replaces the Redis pub/sub idea for cross-replica sync (all-Azure stack) |

> **ADR format:** Each Architecture Decision Record follows the [MADR template](https://adr.github.io/madr/) — Title, Status, Context, Decision, Consequences. AI assistants can read these to understand *why* decisions were made, not just *what* was decided.

### Dev Container Specification

The `.devcontainer/devcontainer.json` ensures every contributor has an identical environment:

```jsonc
{
  "name": "beat-stream-rs",
  "image": "mcr.microsoft.com/devcontainers/rust:1",
  "features": {
    "ghcr.io/devcontainers/features/azure-cli:1": {},
    "ghcr.io/azure/azure-dev/azd:latest": {},
    "ghcr.io/devcontainers/features/docker-in-docker:2": {}
  },
  "postCreateCommand": "cargo install cargo-deny && rustup component add clippy rustfmt",
  "customizations": {
    "vscode": {
      "extensions": [
        "rust-lang.rust-analyzer",
        "ms-azuretools.vscode-docker",
        "GitHub.copilot"
      ]
    }
  },
  "forwardPorts": [8080]
}
```

---

## Appendix: Why Rust?

1. **Tiny container image** — A statically-linked Rust binary with Azure SDK crates is ~15 MB. Node.js or Python containers start at 100+ MB.
2. **Predictable latency** — No garbage collector means no GC pauses during WebSocket broadcasts.
3. **Azure SDK alignment** — The Azure SDK for Rust is approaching GA. beat-stream-rs serves as a real-world validation of the SDK.
4. **Single binary deployment** — `cargo build --release` produces one file. No `node_modules`, no virtual environments.

---

*This document is a living artifact. It will evolve as beat-stream-rs moves from MVP to production.*
