use axum::extract::ws::{Message, WebSocket};
use beat_stream_shared::{ClientMessage, ServerMessage};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Instant;

use crate::room::RoomManager;

/// Max messages per second per client.
const RATE_LIMIT: u32 = 20;
/// Window size for rate limiting.
const RATE_WINDOW: std::time::Duration = std::time::Duration::from_secs(1);

pub async fn handle_socket(socket: WebSocket, mgr: Arc<RoomManager>, room_id: String) {
    let user_id = uuid::Uuid::new_v4().to_string();

    // Join the room (auto-creates if needed)
    let count = mgr.join(&room_id);
    let room = mgr.get_or_create(&room_id);

    // Subscribe to broadcast before sending initial state
    let mut rx = room.tx.subscribe();

    // Notify all existing clients about the new user
    let _ = room
        .tx
        .send(ServerMessage::UserJoined { count });

    let (mut sender, mut receiver) = socket.split();

    // Send initial full state
    let state = mgr.get_state(&room_id).unwrap_or_else(|| {
        beat_stream_shared::RoomState::new(
            room_id.clone(),
            &beat_stream_shared::PatternName::Chill.grid(),
            120,
        )
    });
    let init_msg = ServerMessage::State { room: state };
    if let Ok(json) = serde_json::to_string(&init_msg) {
        if sender.send(Message::Text(json.into())).await.is_err() {
            mgr.leave(&room_id);
            return;
        }
    }

    // Task: forward broadcast messages to this client's WebSocket
    let room_id_fwd = room_id.clone();
    let forward_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
        let _ = room_id_fwd; // keep borrow alive for tracing
    });

    // Task: read client messages, rate-limit, apply, broadcast
    let mut window_start = Instant::now();
    let mut msg_count: u32 = 0;

    while let Some(Ok(msg)) = receiver.next().await {
        let text = match msg {
            Message::Text(t) => t.to_string(),
            Message::Close(_) => break,
            _ => continue,
        };

        // Rate limiting
        let now = Instant::now();
        if now.duration_since(window_start) >= RATE_WINDOW {
            window_start = now;
            msg_count = 0;
        }
        msg_count += 1;
        if msg_count > RATE_LIMIT {
            continue; // silently drop
        }

        let client_msg: ClientMessage = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(_) => {
                let err = ServerMessage::Error {
                    message: "invalid message format".to_string(),
                };
                let _ = room.tx.send(err);
                continue;
            }
        };

        match client_msg {
            ClientMessage::Toggle { track, step } => {
                if mgr.toggle(&room_id, track, step).is_some() {
                    let _ = room.tx.send(ServerMessage::Toggle {
                        track,
                        step,
                        user_id: user_id.clone(),
                    });
                } else {
                    let _ = room.tx.send(ServerMessage::Error {
                        message: "track/step out of range".to_string(),
                    });
                }
            }
            ClientMessage::SetBpm { bpm } => match mgr.set_bpm(&room_id, bpm) {
                Ok(bpm) => {
                    let _ = room.tx.send(ServerMessage::BpmChanged { bpm });
                }
                Err(e) => {
                    let _ = room.tx.send(ServerMessage::Error { message: e });
                }
            },
            ClientMessage::RequestState => {
                if let Some(state) = mgr.get_state(&room_id) {
                    let _ = room.tx.send(ServerMessage::State { room: state });
                }
            }
        }
    }

    // Client disconnected
    forward_task.abort();
    let count = mgr.leave(&room_id);
    if count > 0 {
        if let Some(room) = mgr.rooms.get(&room_id) {
            let _ = room.tx.send(ServerMessage::UserLeft { count });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_constants() {
        assert_eq!(RATE_LIMIT, 20);
        assert_eq!(RATE_WINDOW, std::time::Duration::from_secs(1));
    }

    #[test]
    fn rate_limiter_logic_drops_excess() {
        // Simulate the rate limiter window logic
        let mut window_start = Instant::now();
        let mut msg_count: u32 = 0;
        let mut accepted = 0u32;
        let mut dropped = 0u32;

        for _ in 0..30 {
            let now = Instant::now();
            if now.duration_since(window_start) >= RATE_WINDOW {
                window_start = now;
                msg_count = 0;
            }
            msg_count += 1;
            if msg_count > RATE_LIMIT {
                dropped += 1;
            } else {
                accepted += 1;
            }
        }
        assert_eq!(accepted, RATE_LIMIT);
        assert_eq!(dropped, 10);
    }
}
