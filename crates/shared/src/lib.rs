use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomState {
    pub id: String,
    pub tracks: Vec<Track>,
    pub bpm: u16,
    pub active_users: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub emoji: String,
    pub steps: [u8; 16],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    Toggle { track: u8, step: u8 },
    Bpm { value: u16 },
    State(RoomState),
}
