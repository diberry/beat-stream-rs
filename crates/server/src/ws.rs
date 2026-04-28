use axum::extract::ws::{Message, WebSocket};
use beat_stream_shared::{ClientMessage, ServerMessage};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Instant;

use crate::room::RoomManager;

/// Max messages per second per client.
const RATE_LIMIT: u32 = 20;

pub async fn handle_socket(socket: WebSocket, mgr: Arc<RoomManager>, room_id: String) {
    let user_id = uuid::Uuid::new_v4().to_string();

    let room = mgr.get_or_create(&room_id);

    // Subscribe to broadcast before sending initial state
    let mut rx = room.tx.subscribe();

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Per-client channel for direct messages (errors, unicast state responses)
    let (direct_tx, mut direct_rx) = tokio::sync::mpsc::channel::<ServerMessage>(64);

    // Send initial full state BEFORE incrementing join count (fix #2: memory leak).
    // If this fails, we never called join(), so no cleanup needed.
    let state = mgr.get_state(&room_id).unwrap_or_else(|| {
        beat_stream_shared::RoomState::new(
            room_id.clone(),
            &beat_stream_shared::PatternName::Chill.grid(),
            beat_stream_shared::DEFAULT_BPM,
        )
    });
    let init_msg = ServerMessage::State { room: state };
    if let Ok(json) = serde_json::to_string(&init_msg) {
        if ws_sender.send(Message::Text(json.into())).await.is_err() {
            return;
        }
    }

    // Now that the client is confirmed connected, increment join count.
    let count = mgr.join(&room_id);

    // Forward task: multiplex broadcast + direct channels to the WebSocket sender
    let forward_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(msg) => {
                            if let Ok(json) = serde_json::to_string(&msg) {
                                if ws_sender.send(Message::Text(json.into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
                msg = direct_rx.recv() => {
                    match msg {
                        Some(msg) => {
                            if let Ok(json) = serde_json::to_string(&msg) {
                                if ws_sender.send(Message::Text(json.into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                        None => break,
                    }
                }
            }
        }
    });

    // Broadcast UserJoined AFTER forward_task is spawned (fix #1: race condition)
    let _ = room.tx.send(ServerMessage::UserJoined { count });

    // Token-bucket rate limiter (fix #6: prevents cross-window burst)
    let mut tokens: f64 = RATE_LIMIT as f64;
    let mut last_refill = Instant::now();

    while let Some(Ok(msg)) = ws_receiver.next().await {
        let text = match msg {
            Message::Text(t) => t.to_string(),
            Message::Close(_) => break,
            _ => continue,
        };

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(last_refill).as_secs_f64();
        last_refill = now;
        tokens = (tokens + elapsed * RATE_LIMIT as f64).min(RATE_LIMIT as f64);
        if tokens < 1.0 {
            continue; // silently drop
        }
        tokens -= 1.0;

        let client_msg: ClientMessage = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(_) => {
                // Fix #4: send error only to this client, not broadcast
                let err = ServerMessage::Error {
                    message: "invalid message format".to_string(),
                };
                let _ = direct_tx.send(err).await;
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
                    // Out-of-range error only to this client
                    let _ = direct_tx
                        .send(ServerMessage::Error {
                            message: "track/step out of range".to_string(),
                        })
                        .await;
                }
            }
            ClientMessage::SetBpm { bpm } => match mgr.set_bpm(&room_id, bpm) {
                Ok(bpm) => {
                    let _ = room.tx.send(ServerMessage::BpmChanged { bpm });
                }
                Err(e) => {
                    let _ = direct_tx.send(ServerMessage::Error { message: e }).await;
                }
            },
            ClientMessage::RequestState => {
                if let Some(state) = mgr.get_state(&room_id) {
                    let _ = direct_tx
                        .send(ServerMessage::State { room: state })
                        .await;
                }
            }
        }
    }

    // Client disconnected — always clean up
    forward_task.abort();
    let count = mgr.leave(&room_id);
    if count > 0 {
        let room = mgr.rooms.get(&room_id).map(|r| Arc::clone(r.value()));
        if let Some(room) = room {
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
    }

    #[test]
    fn token_bucket_prevents_burst() {
        // Simulate token bucket: start full, drain 20, then no more tokens
        let mut tokens: f64 = RATE_LIMIT as f64;
        let mut accepted = 0u32;
        let mut dropped = 0u32;

        // All 30 messages arrive instantly (elapsed = 0)
        for _ in 0..30 {
            // No time passes between messages → no refill
            if tokens < 1.0 {
                dropped += 1;
            } else {
                tokens -= 1.0;
                accepted += 1;
            }
        }
        assert_eq!(accepted, RATE_LIMIT);
        assert_eq!(dropped, 10);
    }
}
