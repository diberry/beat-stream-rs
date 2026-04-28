use serde::{Deserialize, Serialize};

pub const NUM_TRACKS: usize = 4;
pub const NUM_STEPS: usize = 16;
pub const BPM_MIN: u16 = 60;
pub const BPM_MAX: u16 = 140;
pub const DEFAULT_BPM: u16 = 120;

pub type Grid = [[bool; NUM_STEPS]; NUM_TRACKS];

pub const TRACK_NAMES: [&str; NUM_TRACKS] = ["Kick", "Snare", "Hi-Hat", "Bass"];
pub const TRACK_EMOJI: [&str; NUM_TRACKS] = ["🥁", "🪘", "🎩", "🎸"];

// ── Room state ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomState {
    pub id: String,
    pub tracks: Vec<Track>,
    pub bpm: u16,
    pub active_users: u32,
}

impl RoomState {
    pub fn new(id: String, grid: &Grid, bpm: u16) -> Self {
        let tracks = grid
            .iter()
            .enumerate()
            .map(|(i, steps)| Track {
                name: TRACK_NAMES[i].to_string(),
                emoji: TRACK_EMOJI[i].to_string(),
                steps: *steps,
            })
            .collect();
        Self {
            id,
            tracks,
            bpm: bpm.clamp(BPM_MIN, BPM_MAX),
            active_users: 0,
        }
    }

    pub fn grid(&self) -> Grid {
        let mut g = [[false; NUM_STEPS]; NUM_TRACKS];
        for (i, track) in self.tracks.iter().enumerate().take(NUM_TRACKS) {
            g[i] = track.steps;
        }
        g
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Track {
    pub name: String,
    pub emoji: String,
    pub steps: [bool; NUM_STEPS],
}

// ── Client → Server ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Toggle { track: u8, step: u8 },
    SetBpm { bpm: u16 },
    RequestState,
}

// ── Server → Client ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ServerMessage {
    State {
        room: RoomState,
    },
    Toggle {
        track: u8,
        step: u8,
        user_id: String,
    },
    BpmChanged {
        bpm: u16,
    },
    UserJoined {
        count: u32,
    },
    UserLeft {
        count: u32,
    },
    Error {
        message: String,
    },
}

// ── Starter patterns ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternName {
    Chill,
    Bounce,
    Pulse,
    Sparse,
    Chaos,
    Heartbeat,
}

impl PatternName {
    pub const ALL: [PatternName; 6] = [
        PatternName::Chill,
        PatternName::Bounce,
        PatternName::Pulse,
        PatternName::Sparse,
        PatternName::Chaos,
        PatternName::Heartbeat,
    ];

    pub fn grid(self) -> Grid {
        match self {
            // Four-on-the-floor with offbeat hi-hats
            PatternName::Chill => [
                [
                    true, false, false, false, true, false, false, false, true, false, false,
                    false, true, false, false, false,
                ],
                [
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
                [
                    false, false, true, false, false, false, true, false, false, false, true,
                    false, false, false, true, false,
                ],
                [
                    true, false, false, false, false, false, false, false, true, false, false,
                    false, false, false, false, false,
                ],
            ],
            // Syncopated bounce
            PatternName::Bounce => [
                [
                    true, false, false, true, false, false, true, false, false, true, false, false,
                    true, false, false, false,
                ],
                [
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, true, false, false, true,
                ],
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true,
                ],
                [
                    false, false, true, false, false, true, false, false, true, false, false, true,
                    false, false, true, false,
                ],
            ],
            // Driving pulse
            PatternName::Pulse => [
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                [
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
                [
                    true, false, false, true, false, false, true, false, false, true, false, false,
                    true, false, false, true,
                ],
                [
                    true, false, false, false, false, false, false, false, true, false, false,
                    false, false, false, false, false,
                ],
            ],
            // Minimal / sparse
            PatternName::Sparse => [
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                [
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, true, false, false,
                    false, false, false, false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
            ],
            // Lots of notes
            PatternName::Chaos => [
                [
                    true, false, true, true, false, true, false, true, true, false, true, false,
                    true, true, false, true,
                ],
                [
                    false, true, false, true, true, false, true, false, false, true, false, true,
                    false, true, true, false,
                ],
                [
                    true, true, false, true, false, true, true, false, true, true, false, true,
                    false, true, true, false,
                ],
                [
                    false, true, true, false, true, false, false, true, false, true, true, false,
                    true, false, false, true,
                ],
            ],
            // Heartbeat rhythm (boom-boom … boom-boom)
            PatternName::Heartbeat => [
                [
                    true, false, true, false, false, false, false, false, true, false, true, false,
                    false, false, false, false,
                ],
                [
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
                [
                    false, false, true, false, false, false, true, false, false, false, true,
                    false, false, false, true, false,
                ],
                [
                    true, false, false, false, false, false, false, false, true, false, false,
                    false, false, false, false, false,
                ],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_grids_have_correct_dimensions() {
        for p in PatternName::ALL {
            let g = p.grid();
            assert_eq!(g.len(), NUM_TRACKS, "{p:?} wrong track count");
            for (i, row) in g.iter().enumerate() {
                assert_eq!(row.len(), NUM_STEPS, "{p:?} track {i} wrong step count");
            }
        }
    }

    #[test]
    fn room_state_clamps_bpm() {
        let s = RoomState::new("r1".into(), &PatternName::Chill.grid(), 200);
        assert_eq!(s.bpm, BPM_MAX);
        let s = RoomState::new("r2".into(), &PatternName::Chill.grid(), DEFAULT_BPM);
        assert_eq!(s.bpm, DEFAULT_BPM);
        let s = RoomState::new("r3".into(), &PatternName::Chill.grid(), 30);
        assert_eq!(s.bpm, BPM_MIN);
    }

    #[test]
    fn room_state_grid_roundtrip() {
        let grid = PatternName::Bounce.grid();
        let s = RoomState::new("r".into(), &grid, 120);
        assert_eq!(s.grid(), grid);
    }

    #[test]
    fn client_message_serde_toggle() {
        let msg = ClientMessage::Toggle { track: 1, step: 5 };
        let json = serde_json::to_string(&msg).unwrap();
        let back: ClientMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn client_message_serde_set_bpm() {
        let msg = ClientMessage::SetBpm { bpm: 120 };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"SetBpm\""));
        let back: ClientMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn client_message_serde_request_state() {
        let msg = ClientMessage::RequestState;
        let json = serde_json::to_string(&msg).unwrap();
        let back: ClientMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn server_message_serde_roundtrip() {
        let state = RoomState::new("r".into(), &PatternName::Chill.grid(), 100);
        let msgs = vec![
            ServerMessage::State { room: state },
            ServerMessage::Toggle {
                track: 0,
                step: 3,
                user_id: "u1".into(),
            },
            ServerMessage::BpmChanged { bpm: 90 },
            ServerMessage::UserJoined { count: 2 },
            ServerMessage::UserLeft { count: 1 },
            ServerMessage::Error {
                message: "oops".into(),
            },
        ];
        for msg in msgs {
            let json = serde_json::to_string(&msg).unwrap();
            let back: ServerMessage = serde_json::from_str(&json).unwrap();
            assert_eq!(msg, back);
        }
    }
}
