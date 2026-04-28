use beat_stream_server::room::RoomManager;

mod ws_bridge {
    pub use beat_stream_server::ws::handle_socket;
}

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{fmt, EnvFilter};

pub type AppState = Arc<RoomManager>;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_room(State(mgr): State<AppState>) -> Json<serde_json::Value> {
    let room_id = mgr.create_room();
    Json(json!({ "room_id": room_id }))
}

async fn get_room(
    State(mgr): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match mgr.get_state(&id) {
        Some(state) => Ok(Json(state)),
        None => Err((axum::http::StatusCode::NOT_FOUND, "room not found")),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(mgr): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_bridge::handle_socket(socket, mgr, id))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./static".into());

    let state: AppState = Arc::new(RoomManager::new());

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/rooms", post(create_room))
        .route("/api/rooms/{id}", get(get_room))
        .route("/api/rooms/{id}/ws", get(ws_handler))
        .with_state(state)
        .fallback_service(ServeDir::new(&static_dir));

    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = ctrl_c => {}
            _ = sigterm.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        ctrl_c.await.expect("failed to listen for Ctrl+C");
    }

    tracing::info!("Shutdown signal received");
}
