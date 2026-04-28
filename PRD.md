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
| **Container image** | ~10 MB (statically-linked Rust binary + static assets) |
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
       ├── cargo build --release
       ├── docker build → ACR
       └── az containerapp update → Container Apps
```

### Scaling Strategy

| Metric | Threshold | Action |
|--------|-----------|--------|
| HTTP concurrency | > 50 per replica | Scale out (up to 5 replicas) |
| HTTP concurrency | < 10 per replica | Scale in (down to 1 replica) |
| Idle | Always | 1 replica stays warm |

> **Important:** Because rooms use in-process `broadcast` channels, all users in a room must hit the same replica. At MVP scale (< 5 replicas), this is naturally handled by sticky sessions on the WebSocket connection. A Redis-backed pub/sub layer would be needed beyond ~5 replicas.

---

## 11. Risks & Mitigations

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| 1 | **Client audio sync across browsers** — Different browsers have different audio scheduling precision. Two users may hear slightly different timing. | 🔴 High | Each client plays its own local audio loop synced to its own clock. This is the industry-standard approach (used by Splice, BandLab, etc.). We sync *state*, not *audio*. |
| 2 | **Azure SDK for Rust beta breakage** — The `azure_*` crates are pre-GA and may introduce breaking changes. | 🔴 High | Pin exact crate versions in `Cargo.toml`. Maintain a thin abstraction layer so we can fall back to raw `reqwest` HTTP calls against Azure REST APIs if a crate breaks. |
| 3 | **Container Apps WebSocket idle timeout** — Azure Container Apps may close idle WebSocket connections. | 🟡 Medium | Implement ping/pong frames every 30 seconds. Configure the maximum idle timeout on the Container App ingress. |
| 4 | **Cold-start latency** — First request after scale-to-zero takes several seconds. | 🟡 Medium | Set `minReplicas: 1` so there is always a warm instance. The Rust binary is ~10 MB and starts in < 500 ms. |
| 5 | **Cosmos DB cost at scale** — Unexpected traffic could drive up RU consumption. | 🟢 Low | Serverless tier charges ~$0.25 per million RUs. Rooms auto-delete via TTL after 24 hours. At MVP traffic levels, monthly cost should stay under $5. |
| 6 | **Multi-replica state divergence** — If scaled beyond 1 replica, rooms are isolated per process. | 🟡 Medium | MVP runs 1 replica. Post-MVP adds Redis pub/sub or moves room state to Cosmos DB change feed for cross-replica sync. |

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

## 14. Post-MVP Roadmap

| Priority | Feature | Dependencies |
|----------|---------|-------------|
| 🥇 | **Saved patterns gallery** | `patterns` Cosmos container, gallery UI |
| 🥇 | **Event Hubs integration** | `azure_messaging_eventhubs` crate, analytics pipeline |
| 🥈 | **Public room discovery** | Room listing API, moderation strategy |
| 🥈 | **Custom sound packs** | Audio file upload, Blob Storage |
| 🥉 | **Share / export loops** | Server-side audio rendering or client-side Tone.js offline render |
| 🥉 | **Mobile-optimized touch** | Touch event handling, gesture support, haptic feedback |
| 🥉 | **Accessibility** | Screen reader support, full keyboard navigation, ARIA labels |
| 🏅 | **Multi-replica sync** | Redis pub/sub or Cosmos DB change feed for cross-replica room state |

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

## Appendix: Why Rust?

1. **Tiny container image** — A statically-linked Rust binary is ~10 MB. Node.js or Python containers start at 100+ MB.
2. **Predictable latency** — No garbage collector means no GC pauses during WebSocket broadcasts.
3. **Azure SDK alignment** — The Azure SDK for Rust is approaching GA. beat-stream-rs serves as a real-world validation of the SDK.
4. **Single binary deployment** — `cargo build --release` produces one file. No `node_modules`, no virtual environments.

---

*This document is a living artifact. It will evolve as beat-stream-rs moves from MVP to production.*
