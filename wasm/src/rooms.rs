use crate::doors::{Door, DoorLock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Room {
    entry: Door,
    exit: Door,
}

impl Room {
    pub fn new(entry: Door, exit: Door) -> Self { // <-- Add pub
        Room { entry, exit }
    }
}

#[wasm_bindgen]
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct RoomId(usize);

#[wasm_bindgen]
pub struct RoomGrid {
    rooms: Vec<Room>,
}

impl RoomGrid {
    pub fn new(num_rooms: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rooms = Vec::with_capacity(num_rooms);

        for _ in 0..num_rooms {
            let entry = Door::new(DoorLock::random(&mut rng));
            let exit = Door::new(DoorLock::random(&mut rng));
            rooms.push(Room::new(entry, exit));
        }

        RoomGrid { rooms }
    }

    pub fn neighbors(&self, room_id: &RoomId) -> Vec<RoomId> {
        if room_id.0 + 1 < self.rooms.len() {
            vec![RoomId(room_id.0 + 1)]
        } else {
            Vec::new()
        }
    }

    pub fn is_accessible(&self, current: &RoomId, neighbor: &RoomId) -> bool {
        if let Some(current_room) = self.rooms.get(current.0) {
            if let Some(neighbor_room) = self.rooms.get(neighbor.0) {
                return current_room.exit.is_unlocked() && neighbor_room.entry.is_unlocked();
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn door_lock_randomization() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let state = DoorLock::random(&mut rng);
            assert!(matches!(state,
                DoorLock::LockedFromInside |
                DoorLock::LockedFromOutside |
                DoorLock::UnlockedFromInside |
                DoorLock::UnlockedFromOutside
            ));
        }
    }

    #[test]
    fn room_grid_generation() {
        let num_rooms = 10;
        let grid = RoomGrid::new(num_rooms);
        assert_eq!(grid.rooms.len(), num_rooms);
    }
}
