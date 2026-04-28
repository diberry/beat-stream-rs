//! Integration tests for WebSocket lifecycle.
//!
//! Run with: cargo test -p beat-stream-server --features integration --test ws_integration

#![cfg(feature = "integration")]

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

/// Boot a one-shot test server and return its base URL (e.g. "127.0.0.1:PORT").
async fn spawn_server() -> String {
    use axum::{
        extract::{Path, WebSocketUpgrade},
        routing::{get, post},
        Json, Router,
    };
    use serde_json::json;
    use std::sync::Arc;

    // We replicate the app setup inline so tests don't depend on `main`.
    mod inline {
        // Re-export the server crate modules we need (they're pub(crate), so we
        // work through the binary's public API instead).
    }

    // Use the actual server binary types via integration-test linking.
    // Since integration tests can't access pub(crate) items directly, we build
    // a minimal router that exercises the same code paths.

    let mgr = Arc::new(beat_stream_server::room_manager_new());

    let app = Router::new()
        .route(
            "/api/rooms",
            post({
                let mgr = mgr.clone();
                move |_: ()| {
                    let mgr = mgr.clone();
                    async move {
                        let id = mgr.create_room();
                        Json(json!({ "room_id": id }))
                    }
                }
            }),
        )
        .route(
            "/api/rooms/{id}/ws",
            get({
                let mgr = mgr.clone();
                move |ws: WebSocketUpgrade, Path(id): Path<String>| {
                    let mgr = mgr.clone();
                    async move {
                        ws.on_upgrade(move |socket| {
                            beat_stream_server::handle_socket_pub(socket, mgr, id)
                        })
                    }
                }
            }),
        );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr.to_string()
}

#[tokio::test]
async fn test_ws_connect_receives_state() {
    let addr = spawn_server().await;
    let room_id = "test-room-1";
    let url = format!("ws://{addr}/api/rooms/{room_id}/ws");

    let (mut ws, _) = connect_async(&url).await.expect("connect failed");

    // First message should be State
    let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws.next())
        .await
        .expect("timeout")
        .expect("stream ended")
        .expect("read error");

    let text = msg.into_text().expect("not text");
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(value["type"], "State");
    assert!(value["room"].is_object());
    assert_eq!(value["room"]["id"], room_id);

    ws.close(None).await.ok();
}

#[tokio::test]
async fn test_ws_toggle_broadcast() {
    let addr = spawn_server().await;
    let room_id = "test-room-2";
    let url = format!("ws://{addr}/api/rooms/{room_id}/ws");

    // Connect client 1
    let (mut ws1, _) = connect_async(&url).await.expect("c1 connect");
    // Consume State
    let _ = ws1.next().await;
    // Consume UserJoined for client 1
    let _ = ws1.next().await;

    // Connect client 2
    let (mut ws2, _) = connect_async(&url).await.expect("c2 connect");
    // Consume State
    let _ = ws2.next().await;

    // Client 1 should receive UserJoined for client 2
    let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws1.next())
        .await
        .expect("timeout")
        .expect("stream ended")
        .expect("read error");
    let text = msg.into_text().unwrap();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["type"], "UserJoined");

    // Client 2 sends a toggle
    let toggle = serde_json::json!({"type": "Toggle", "track": 0, "step": 0});
    ws2.send(Message::Text(toggle.to_string().into()))
        .await
        .unwrap();

    // Client 1 should receive the Toggle broadcast
    let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws1.next())
        .await
        .expect("timeout")
        .expect("stream ended")
        .expect("read error");
    let text = msg.into_text().unwrap();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["type"], "Toggle");
    assert_eq!(v["track"], 0);
    assert_eq!(v["step"], 0);

    ws1.close(None).await.ok();
    ws2.close(None).await.ok();
}

#[tokio::test]
async fn test_ws_disconnect_cleanup() {
    let addr = spawn_server().await;
    let room_id = "test-room-3";
    let url = format!("ws://{addr}/api/rooms/{room_id}/ws");

    let (mut ws, _) = connect_async(&url).await.expect("connect");
    // Consume State
    let _ = ws.next().await;

    // Disconnect
    ws.close(None).await.ok();
    // Give server time to process disconnect
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Reconnecting should work (room was cleaned up or re-created)
    let (mut ws2, _) = connect_async(&url).await.expect("reconnect");
    let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws2.next())
        .await
        .expect("timeout")
        .expect("stream ended")
        .expect("read error");
    let text = msg.into_text().unwrap();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["type"], "State");
    // active_users should be 0 or 1 (this is a fresh room or just joined)
    assert!(v["room"]["active_users"].as_u64().unwrap() <= 1);

    ws2.close(None).await.ok();
}
