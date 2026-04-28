# Phase 1 Test Plan: Beat Stream RS

**Scope:** Core WebSocket communication, room management, BPM sync, grid state, and basic audio playback
**Target Coverage:** >80% line coverage on server crate
**Duration:** Phase 1 testing cycle

---

## 1. Unit Tests (Rust)

### 1.1 Room Creation
- **Objective:** Verify room is properly initialized with correct defaults
- **Tests:**
  - `test_room_new_initializes_4x16_grid` - Verify grid dimensions are 4 tracks × 16 steps
  - `test_room_new_default_bpm_120` - Verify BPM defaults to 120
  - `test_room_new_pattern_assigned` - Verify a starter pattern is assigned to all tracks

### 1.2 Starter Patterns
- **Objective:** Validate all 6 starter patterns (Chill, Bounce, Pulse, Sparse, Chaos, Heartbeat) have correct structure
- **Tests:**
  - `test_starter_patterns_have_correct_dimensions` - Each pattern: 4 tracks × 16 steps
  - `test_starter_patterns_all_valid_bools` - All cells are valid boolean values (true/false)
  - `test_starter_patterns_count` - Exactly 6 patterns available (`PatternName::ALL.len() == 6`)

### 1.3 WebSocket Message Serialization
- **Objective:** Ensure messages serialize/deserialize correctly per `#[serde(tag = "type")]`
- **Message format reference:**
  - ClientMessage (internally tagged): `Toggle` → `{"type":"Toggle","track":0,"step":0}`, `SetBpm` → `{"type":"SetBpm","bpm":120}`, `RequestState` → `{"type":"RequestState"}`
  - ServerMessage (internally tagged): `State` → `{"type":"State","room":{...}}`, `Toggle` → `{"type":"Toggle","track":0,"step":0,"user_id":"..."}`, `BpmChanged` → `{"type":"BpmChanged","bpm":120}`
- **Tests:**
  - `test_client_message_serde_roundtrip` - ClientMessage encode → decode matches original
  - `test_server_message_serde_roundtrip` - ServerMessage encode → decode matches original
  - `test_malformed_message_rejected` - Invalid JSON is rejected gracefully

### 1.4 BPM Validation
- **Objective:** Enforce BPM range constraints
- **Tests:**
  - `test_bpm_accepts_60_to_140` - Accept any BPM in valid range [60, 140]
  - `test_bpm_rejects_below_60` - Reject BPM < 60
  - `test_bpm_rejects_above_140` - Reject BPM > 140

### 1.5 Rate Limiting
- **Objective:** Verify >20 messages/sec from single client are dropped
- **Tests:**
  - `test_rate_limit_enforced_at_20_per_second` - >20 msg/s are dropped without disconnecting
  - `test_rate_limit_does_not_disconnect` - Client remains connected after rate limit hit
  - `test_rate_limit_resets_per_client` - Limit is per-client, not global
  - `test_rate_limit_window_resets_after_one_second` - After 1s window elapses, client can send again
  - `test_rate_limit_per_client_isolation` - Client A hitting limit does not affect Client B

### 1.6 Room Cleanup
- **Objective:** Verify rooms are removed when last client disconnects
- **Tests:**
  - `test_room_cleaned_up_after_last_client_disconnects` - Room eventually removed from manager
  - `test_room_persists_while_clients_connected` - Room exists while ≥1 client connected

### 1.7 Toggle Logic
- **Objective:** Verify grid cell toggling updates state and broadcasts
- **Tests:**
  - `test_toggle_cell_updates_state` - Toggling a cell changes its value in state
  - `test_toggle_broadcasts_to_all_clients` - Change is sent to all connected clients in room

---

## 2. Integration Tests (Rust)

### 2.1 Health Endpoint
- **Objective:** Verify server health check
- **Test:** `test_health_endpoint_returns_200_with_ok_status`
  - GET /api/health
  - Expect: 200 status, `{"status":"ok"}` JSON body

### 2.2 Room Creation via POST
- **Objective:** Verify room creation endpoint
- **Test:** `test_create_room_post_returns_room_id`
  - POST /api/rooms
  - Expect: 201 status, JSON with `room_id` field
  - Verify room is accessible via subsequent WS connection

### 2.3 WebSocket Connection and State Reception
- **Objective:** Verify client connects and receives initial state
- **Test:** `test_websocket_connects_and_receives_state`
  - Connect via WS to /ws/{room_id}
  - Verify: First message is `{"type":"State","room":{"id":"...","tracks":[...],"bpm":120,"active_users":1}}`

### 2.4 Multi-Client Sync
- **Objective:** Verify two clients in same room receive state updates
- **Test:** `test_two_clients_sync_grid_changes`
  - Client A connects to room
  - Client B connects to same room
  - Client A sends Toggle: `{"type":"Toggle","track":0,"step":0}`
  - Verify: Client B receives Toggle broadcast: `{"type":"Toggle","track":0,"step":0,"user_id":"..."}`

### 2.5 BPM Sync
- **Objective:** Verify all clients receive BPM updates
- **Test:** `test_bpm_change_syncs_to_all_clients`
  - Client A connects
  - Client B connects
  - Client A sends: `{"type":"SetBpm","bpm":80}`
  - Verify: Both A and B receive `{"type":"BpmChanged","bpm":80}`

### 2.6 Room Lifecycle
- **Objective:** Verify create → connect → disconnect → cleanup flow
- **Test:** `test_full_room_lifecycle`
  - POST /api/rooms → get room_id
  - WS connect to /ws/{room_id}
  - Send: `{"type":"Toggle","track":0,"step":5}`
  - Disconnect
  - Verify: Room is cleaned up (room no longer exists in manager)

### 2.7 Static File Serving
- **Objective:** Verify frontend assets are served
- **Test:** `test_get_root_serves_index_html`
  - GET /
  - Expect: 200 status, HTML content-type, HTML body (index.html)

---

## 3. Frontend Smoke Tests (Manual Checklist for Phase 1)

| Test Case | Expected Result | Status |
|-----------|-----------------|--------|
| Page loads without JS errors | Console has no JS errors, page is interactive | ☐ |
| Grid renders 4×16 cells | 64 cells visible in grid layout | ☐ |
| Clicking cell toggles it visually | Cell background changes when clicked | ☐ |
| Audio plays after user gesture | Playback starts after first user interaction | ☐ |
| BPM slider changes tempo | Tempo audibly increases/decreases with slider | ☐ |
| Connection indicator shows green | Green dot/icon visible when WS connected | ☐ |
| Multiple browser tabs show synchronized state | Toggle in Tab 1 reflects in Tab 2 | ☐ |

---

## 4. Test Execution Strategy

### Unit Tests
- Run via `cargo test --lib` in `crates/server`
- All tests should pass before merge

### Integration Tests
- Run via `cargo test --test integration` in `crates/server`
- Requires server to start (tests use `axum::test`)
- All tests should pass before merge

### Frontend Smoke Tests
- Manual testing in Chrome/Firefox
- Must pass before Phase 1 release
- Documented in test run log

### Coverage Requirements
- Target: **>80% line coverage** on `crates/server` main code
- Measured via: `cargo tarpaulin --out Html`
- Excluded: test utilities, dead code

---

## 5. Known Constraints & Deferred Tests

- **Audio latency:** Phase 1 does not test sub-100ms latency (deferred to Phase 2)
- **Scalability:** Phase 1 tests assume <10 rooms, <20 clients/room (deferred to Phase 2)
- **Persistence:** No database testing (all state in-memory, deferred to Phase 2)
- **Mobile UI:** Phase 1 smoke tests use desktop browser only (mobile deferred to Phase 2)

---

## 6. Test Maintenance Notes

- Update test plan after Phase 1 release to reflect new constraints discovered
- Add tests for any production bugs found post-Phase 1
- Maintain >80% coverage requirement as new features added
