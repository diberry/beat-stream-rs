# Beat Stream — Architecture Overview

## System Diagram

```
┌─────────────────────────────────────────────────────────┐
│                      Clients                            │
│  (Browser / Mobile — HTML + JS served from /dist)       │
└────────────────────────┬────────────────────────────────┘
                         │  HTTP / WebSocket
                         ▼
┌─────────────────────────────────────────────────────────┐
│              beat-stream-server  (:8080)                 │
│                                                         │
│  ┌──────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │ REST API │  │  WebSocket   │  │  Static Files     │  │
│  │ /api/*   │  │  /ws/:room   │  │  tower-http       │  │
│  └────┬─────┘  └──────┬───────┘  │  ServeDir         │  │
│       │               │          └───────────────────┘  │
│       ▼               ▼                                 │
│  ┌─────────────────────────────┐                        │
│  │   Room State Manager        │                        │
│  │   (in-memory + sync)        │                        │
│  └──────────┬──────────────────┘                        │
│             │                                           │
└─────────────┼───────────────────────────────────────────┘
              │
              ▼
┌──────────────────────────┐  ┌──────────────────────────┐
│   Azure Cosmos DB        │  │   Azure Key Vault        │
│   (room persistence)     │  │   (secrets management)   │
└──────────────────────────┘  └──────────────────────────┘
```

## Component Responsibilities

### `crates/server` — `beat-stream-server`
- **HTTP API**: Health check (`/api/health`), room CRUD endpoints.
- **WebSocket**: Real-time beat-pattern synchronization per room.
- **Static Serving**: Serves the frontend SPA from `frontend/dist/`.
- **Tracing**: Structured JSON logging via `tracing-subscriber`.
- **Graceful Shutdown**: Handles SIGTERM for clean container stops.

### `crates/shared` — `beat-stream-shared`
- **`RoomState`**: Canonical room representation (tracks, BPM, user count).
- **`Track`**: Individual track with a 16-step pattern grid.
- **`WsMessage`**: Tagged enum for all WebSocket message types.
- Shared between server and any future CLI or WASM client.

### Azure Services
- **Cosmos DB**: Persists room state so rooms survive restarts.
- **Key Vault**: Stores connection strings and secrets; accessed via
  `azure_identity::DefaultAzureCredential`.

## Data Flow

1. Client opens WebSocket to `/ws/:room_id`.
2. Server loads or creates `RoomState` from Cosmos DB.
3. Server broadcasts current `State(RoomState)` to the new client.
4. Client sends `Toggle` or `Bpm` messages.
5. Server applies the mutation, broadcasts updated state to all
   connected clients in the room, and persists to Cosmos DB.
