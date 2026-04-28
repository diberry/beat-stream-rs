use beat_stream_shared::{PatternName, RoomState, ServerMessage, NUM_TRACKS, NUM_STEPS};
use dashmap::DashMap;
use rand::Rng;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;

const BROADCAST_CAPACITY: usize = 256;

pub struct Room {
    pub state: parking_lot::RwLock<RoomState>,
    pub tx: broadcast::Sender<ServerMessage>,
    pub connections: AtomicU32,
}

impl Room {
    fn new(id: String, pattern: PatternName) -> Self {
        let grid = pattern.grid();
        let state = RoomState::new(id, &grid, 120);
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Self {
            state: parking_lot::RwLock::new(state),
            tx,
            connections: AtomicU32::new(0),
        }
    }
}

pub struct RoomManager {
    pub rooms: Arc<DashMap<String, Arc<Room>>>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(DashMap::new()),
        }
    }

    /// Creates a new room with a random starter pattern and returns the room ID.
    pub fn create_room(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let pattern = random_pattern();
        let room = Arc::new(Room::new(id.clone(), pattern));
        self.rooms.insert(id.clone(), room);
        id
    }

    /// Returns a snapshot of the room state, or None if the room doesn't exist.
    pub fn get_state(&self, id: &str) -> Option<RoomState> {
        self.rooms.get(id).map(|r| r.state.read().clone())
    }

    /// Returns (or auto-creates) the room for the given id.
    pub fn get_or_create(&self, id: &str) -> Arc<Room> {
        if let Some(r) = self.rooms.get(id) {
            return Arc::clone(r.value());
        }
        let pattern = random_pattern();
        let room = Arc::new(Room::new(id.to_string(), pattern));
        self.rooms.entry(id.to_string()).or_insert(room).clone()
    }

    /// Increments connection count and returns the new count.
    pub fn join(&self, id: &str) -> u32 {
        let room = self.get_or_create(id);
        let count = room.connections.fetch_add(1, Ordering::SeqCst) + 1;
        room.state.write().active_users = count;
        count
    }

    /// Decrements connection count; removes the room if zero remain. Returns new count.
    pub fn leave(&self, id: &str) -> u32 {
        if let Some(room) = self.rooms.get(id) {
            let prev = room.connections.fetch_sub(1, Ordering::SeqCst);
            let count = prev.saturating_sub(1);
            room.state.write().active_users = count;
            if count == 0 {
                drop(room); // release DashMap ref before removal
                self.rooms.remove(id);
            }
            count
        } else {
            0
        }
    }

    /// Toggles a cell and returns the updated value (true = on).
    pub fn toggle(&self, id: &str, track: u8, step: u8) -> Option<bool> {
        if track as usize >= NUM_TRACKS || step as usize >= NUM_STEPS {
            return None;
        }
        self.rooms.get(id).map(|r| {
            let mut state = r.state.write();
            let cell = &mut state.tracks[track as usize].steps[step as usize];
            *cell = !*cell;
            *cell
        })
    }

    /// Sets BPM if in valid range and returns Ok(clamped_bpm) or Err message.
    pub fn set_bpm(&self, id: &str, bpm: u16) -> Result<u16, String> {
        use beat_stream_shared::{BPM_MAX, BPM_MIN};
        if !(BPM_MIN..=BPM_MAX).contains(&bpm) {
            return Err(format!("BPM must be between {BPM_MIN} and {BPM_MAX}"));
        }
        match self.rooms.get(id) {
            Some(r) => {
                r.state.write().bpm = bpm;
                Ok(bpm)
            }
            None => Err("room not found".to_string()),
        }
    }
}

fn random_pattern() -> PatternName {
    let mut rng = rand::thread_rng();
    PatternName::ALL[rng.gen_range(0..PatternName::ALL.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_room_assigns_pattern() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        let state = mgr.get_state(&id).unwrap();
        assert_eq!(state.tracks.len(), NUM_TRACKS);
        for t in &state.tracks {
            assert_eq!(t.steps.len(), NUM_STEPS);
        }
        assert_eq!(state.bpm, 120);
        assert_eq!(state.active_users, 0);
    }

    #[test]
    fn join_and_leave_updates_count() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        assert_eq!(mgr.join(&id), 1);
        assert_eq!(mgr.join(&id), 2);
        assert_eq!(mgr.leave(&id), 1);
        assert_eq!(mgr.get_state(&id).unwrap().active_users, 1);
        assert_eq!(mgr.leave(&id), 0);
        // Room should be removed after last leave
        assert!(mgr.get_state(&id).is_none());
    }

    #[test]
    fn auto_create_on_join() {
        let mgr = RoomManager::new();
        assert_eq!(mgr.join("new-room"), 1);
        assert!(mgr.get_state("new-room").is_some());
    }

    #[test]
    fn toggle_flips_cell() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        let initial = mgr.get_state(&id).unwrap().tracks[0].steps[0];
        let toggled = mgr.toggle(&id, 0, 0).unwrap();
        assert_eq!(toggled, !initial);
        let toggled_back = mgr.toggle(&id, 0, 0).unwrap();
        assert_eq!(toggled_back, initial);
    }

    #[test]
    fn toggle_rejects_out_of_bounds() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        assert!(mgr.toggle(&id, 4, 0).is_none());
        assert!(mgr.toggle(&id, 0, 16).is_none());
    }

    #[test]
    fn set_bpm_valid_range() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        assert!(mgr.set_bpm(&id, 100).is_ok());
        assert_eq!(mgr.get_state(&id).unwrap().bpm, 100);
    }

    #[test]
    fn set_bpm_rejects_out_of_range() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        assert!(mgr.set_bpm(&id, 59).is_err());
        assert!(mgr.set_bpm(&id, 141).is_err());
        assert!(mgr.set_bpm(&id, 60).is_ok());
        assert!(mgr.set_bpm(&id, 140).is_ok());
    }

    #[test]
    fn broadcast_channel_works() {
        let mgr = RoomManager::new();
        let id = mgr.create_room();
        let room = mgr.get_or_create(&id);
        let mut rx = room.tx.subscribe();
        let msg = ServerMessage::BpmChanged { bpm: 90 };
        room.tx.send(msg.clone()).unwrap();
        let received = rx.try_recv().unwrap();
        assert_eq!(received, msg);
    }
}
