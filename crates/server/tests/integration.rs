// Integration tests for Beat Stream RS server
// Tests WebSocket communication, room management, and API endpoints
//
// Message format reference (from crates/shared/src/lib.rs):
//   ClientMessage uses #[serde(tag = "type")] (internally tagged):
//     Toggle  → {"type":"Toggle","track":0,"step":0}
//     SetBpm  → {"type":"SetBpm","bpm":120}
//     RequestState → {"type":"RequestState"}
//   ServerMessage uses #[serde(tag = "type")] (internally tagged):
//     State      → {"type":"State","room":{...}}
//     Toggle     → {"type":"Toggle","track":0,"step":0,"user_id":"..."}
//     BpmChanged → {"type":"BpmChanged","bpm":120}
//     UserJoined → {"type":"UserJoined","count":2}
//     UserLeft   → {"type":"UserLeft","count":1}
//     Error      → {"type":"Error","message":"..."}

use tokio_tungstenite::tungstenite::Message;

// ── Health / Static Serving ─────────────────────────────────────────────────

/// Health Endpoint Tests
#[tokio::test]
async fn health_endpoint_returns_200_with_ok_status() {
    // TODO: Implement health check test
    // - Start test server via start_test_server()
    // - GET /api/health
    // - Assert: status 200, body {"status":"ok"}
    todo!("Implement health endpoint test");
}

#[tokio::test]
async fn get_root_serves_index_html() {
    // TODO: Implement static file serving test
    // - Start test server via start_test_server()
    // - GET / (root path)
    // - Assert: status 200
    // - Assert: content-type includes "text/html"
    // - Assert: body contains HTML (e.g., <html>, <body>, <script>)
    todo!("Implement static file serving test");
}

#[tokio::test]
async fn get_api_health_returns_json() {
    // TODO: Implement health endpoint JSON test
    // - Start test server via start_test_server()
    // - GET /api/health
    // - Assert: status 200
    // - Assert: content-type is "application/json"
    // - Assert: body parses as JSON
    // - Assert: body["status"] == "ok"
    todo!("Implement API health endpoint test");
}

// ── Room Creation (requires Phase 1 backend, PR #5) ────────────────────────

/// Room Creation Tests
#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — POST /api/rooms not yet implemented
async fn create_room_post_returns_room_id() {
    // TODO: Implement room creation test
    // - POST /api/rooms
    // - Assert: status 201
    // - Assert: response contains room_id field
    // - Assert: room_id is non-empty UUID or similar
    todo!("Implement room creation test");
}

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — POST /api/rooms not yet implemented
async fn room_is_accessible_after_creation() {
    // TODO: Implement room accessibility test
    // - Create room via POST /api/rooms
    // - Connect via WS to /ws/{room_id}
    // - Assert: connection succeeds
    // - Assert: receives initial State message: {"type":"State","room":{...}}
    todo!("Implement room accessibility test");
}

// ── WebSocket Connection (requires Phase 1 backend, PR #5) ─────────────────

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn websocket_connects_and_receives_state() {
    // TODO: Implement WebSocket connection test
    // - Create room via POST /api/rooms
    // - Connect client via tokio_tungstenite to /ws/{room_id}
    // - Assert: connection succeeds
    // - Assert: first message is State message:
    //   {"type":"State","room":{"id":"...","tracks":[...],"bpm":120,"active_users":1}}
    //   - room.tracks: 4 tracks × 16 steps
    //   - room.bpm: 120 (default)
    //   - room.id: matches created room
    // - Close connection gracefully
    todo!("Implement WebSocket connection test");
}

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn websocket_malformed_message_handled_gracefully() {
    // TODO: Implement malformed message test
    // - Connect to room
    // - Send invalid JSON (not a valid ClientMessage)
    // - Assert: server responds with {"type":"Error","message":"..."}
    // - Assert: connection stays open
    // - Assert: server does not crash
    // - Close connection
    todo!("Implement malformed message handling test");
}

// ── Multi-Client Synchronization (requires Phase 1 backend, PR #5) ─────────

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn two_clients_sync_grid_changes() {
    // TODO: Implement multi-client sync test
    // - Create room via POST /api/rooms
    // - Connect client_a via WS
    // - Connect client_b via WS
    // - Assert: both clients receive initial State
    // - client_a sends: {"type":"Toggle","track":0,"step":0}
    // - Assert: client_a receives Toggle broadcast:
    //   {"type":"Toggle","track":0,"step":0,"user_id":"..."}
    // - Assert: client_b receives same Toggle broadcast
    // - Verify grid state is consistent
    todo!("Implement two-client sync test");
}

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn bpm_change_syncs_to_all_clients() {
    // TODO: Implement BPM sync test
    // - Create room
    // - Connect client_a and client_b
    // - client_a sends: {"type":"SetBpm","bpm":100}
    // - Assert: client_a receives {"type":"BpmChanged","bpm":100}
    // - Assert: client_b receives {"type":"BpmChanged","bpm":100}
    // - Verify room state reflects new BPM
    todo!("Implement BPM sync test");
}

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn client_disconnect_does_not_crash_server() {
    // TODO: Implement disconnect resilience test
    // - Create room with 2 clients
    // - client_a sends: {"type":"Toggle","track":1,"step":3}
    // - client_a disconnects abruptly
    // - Assert: client_b receives {"type":"UserLeft","count":1}
    // - Assert: client_b still connected and can send messages
    todo!("Implement disconnect resilience test");
}

// ── Room Lifecycle (requires Phase 1 backend, PR #5) ────────────────────────

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn full_room_lifecycle() {
    // TODO: Implement full lifecycle test
    // - POST /api/rooms → get room_id
    // - WS connect to /ws/{room_id} with client
    // - client sends: {"type":"Toggle","track":0,"step":5}
    // - Assert: client receives Toggle broadcast
    // - client disconnects
    // - Wait a short time for cleanup
    // - Assert: room is removed from server
    todo!("Implement full room lifecycle test");
}

// ── Rate Limiting (requires Phase 1 backend, PR #5) ────────────────────────

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn rate_limit_enforced_at_20_per_second() {
    // TODO: Implement rate limiting test
    // - Create room and connect client
    // - Send 25 Toggle messages rapidly (within 1 second)
    //   e.g. {"type":"Toggle","track":0,"step":N} for N in 0..25
    // - Assert: first 20 messages are processed (Toggle broadcasts received)
    // - Assert: messages 21-25 are dropped (not echoed back)
    // - Assert: client is NOT disconnected
    // - Assert: client can send messages again after rate limit window resets
    todo!("Implement rate limiting test");
}

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn rate_limit_window_resets_after_one_second() {
    // TODO: Implement rate limit window reset test
    // - Create room and connect client
    // - Send 20 messages (at the limit)
    // - Sleep 1 second (wait for window reset)
    // - Send 5 more messages
    // - Assert: all 5 post-reset messages are processed
    todo!("Implement rate limit window reset test");
}

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room management not yet implemented
async fn rate_limit_is_per_client() {
    // TODO: Implement per-client rate limit isolation test
    // - Create room with client_a and client_b
    // - client_a sends 20 messages (at the limit)
    // - client_b sends 5 messages
    // - Assert: client_b's messages are all processed (not affected by client_a's limit)
    todo!("Implement per-client rate limit isolation test");
}

// ── Starter Patterns ────────────────────────────────────────────────────────

#[tokio::test]
#[ignore] // Requires Phase 1 backend (PR #5) — room creation with patterns not yet implemented
async fn all_six_starter_patterns_available() {
    // TODO: Verify all 6 starter patterns exist and produce valid grids
    // PatternName::ALL should contain: Chill, Bounce, Pulse, Sparse, Chaos, Heartbeat
    // Each pattern.grid() should be 4 tracks × 16 steps
    // NOTE: unit tests in crates/shared already verify dimensions;
    //       this integration test verifies patterns work end-to-end via room creation
    todo!("Implement starter pattern integration test");
}

// ── Test Helpers ────────────────────────────────────────────────────────────

/// Start test server on an OS-assigned random port (port 0) and return the base URL.
///
/// Uses `TcpListener::bind("127.0.0.1:0")` so each test gets its own port,
/// avoiding collisions when tests run in parallel.
async fn start_test_server() -> String {
    // TODO: Bind TcpListener to 127.0.0.1:0 to get a random available port
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    // let addr = listener.local_addr().unwrap();
    // Spawn the Axum server on that listener
    // Return format!("http://{addr}")
    todo!("Implement start_test_server with port 0 binding");
}

/// Establish a WebSocket connection to ws://{base_url}/ws/{room_id}.
async fn connect_ws_client(
    base_url: &str,
    room_id: &str,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    // TODO: Convert base_url http:// → ws://, connect to /ws/{room_id}
    // let ws_url = base_url.replace("http://", "ws://");
    // let (ws_stream, _) = tokio_tungstenite::connect_async(
    //     format!("{ws_url}/ws/{room_id}")
    // ).await.unwrap();
    // ws_stream
    todo!("Implement connect_ws_client");
}

/// Read the next text message from a WebSocket stream.
async fn receive_message(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> String {
    // TODO: Read next message from WebSocket
    // use futures_util::StreamExt;
    // match ws.next().await {
    //     Some(Ok(Message::Text(text))) => text,
    //     other => panic!("Expected text message, got: {other:?}"),
    // }
    todo!("Implement receive_message");
}
