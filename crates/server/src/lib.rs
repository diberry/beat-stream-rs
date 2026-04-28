pub mod room;
pub mod ws;

/// Public helper so integration tests can construct a `RoomManager`.
pub fn room_manager_new() -> room::RoomManager {
    room::RoomManager::new()
}

/// Public wrapper around `ws::handle_socket` for integration tests.
pub async fn handle_socket_pub(
    socket: axum::extract::ws::WebSocket,
    mgr: std::sync::Arc<room::RoomManager>,
    room_id: String,
) {
    ws::handle_socket(socket, mgr, room_id).await
}
