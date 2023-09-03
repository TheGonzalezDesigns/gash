mod doors;
mod rooms;
mod pathfinding;

pub use rooms::{Room, RoomGrid};
pub use pathfinding::PathFinder; // Expose the PathFinder struct

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen]
pub fn greet() {
    web_sys::console::log_1(&"Hello from Rust!".into());
}

#[wasm_bindgen]
pub fn find_path(start_room: usize, end_room: usize, room_grid_js: &JsValue) -> Option<Vec<usize>> {
    let grid: rooms::RoomGrid = room_grid_js.into_serde().unwrap();
    core_find_path(start_room, end_room, &grid)
}

#[wasm_bindgen]
pub fn generate_random_rooms(num_rooms: usize) -> JsValue {
    let grid = core_generate_random_rooms(num_rooms);
    serde_wasm_bindgen::to_value(&grid).unwrap()
}

// Core logic for finding a path
fn core_find_path(start_room: usize, end_room: usize, grid: &rooms::RoomGrid) -> Option<Vec<usize>> {
    let mut pathfinder = pathfinding::PathFinder::new();
    let path = pathfinder.find_path(rooms::RoomId(start_room), rooms::RoomId(end_room), grid);
    path.map(|p| p.into_iter().map(|room_id| room_id.0).collect())
}

// Core logic for generating random rooms
fn core_generate_random_rooms(num_rooms: usize) -> RoomGrid {
    RoomGrid::new(num_rooms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use doors::DoorLock;

    #[test]
    fn test_greet() {
        // For now, this is more of an integration test.
        greet();
    }

    #[test]
    fn test_find_path_basic() {
        let room1 = Room {
            entry: doors::Door { lock: DoorLock::UnlockedFromInside },
            exit: doors::Door { lock: DoorLock::UnlockedFromOutside }
        };
        let room2 = Room {
            entry: doors::Door { lock: DoorLock::UnlockedFromInside },
            exit: doors::Door { lock: DoorLock::UnlockedFromOutside }
        };
        let room_grid = RoomGrid { rooms: vec![room1, room2] };
        let path = core_find_path(0, 1, &room_grid);
        assert_eq!(path, Some(vec![0, 1]));
    }

    #[test]
    fn test_find_path_no_exit() {
        let room1 = Room {
            entry: doors::Door { lock: DoorLock::UnlockedFromInside },
            exit: doors::Door { lock: DoorLock::LockedFromInside }
        };
        let room2 = Room {
            entry: doors::Door { lock: DoorLock::LockedFromOutside },
            exit: doors::Door { lock: DoorLock::UnlockedFromOutside }
        };
        let room_grid = RoomGrid { rooms: vec![room1, room2] };
        let path = core_find_path(0, 1, &room_grid);
        assert_eq!(path, None);
    }

    #[test]
    fn test_generate_random_rooms() {
        let num_rooms = 10;
        let generated_grid = core_generate_random_rooms(num_rooms);
        assert_eq!(generated_grid.rooms.len(), num_rooms);
    }

    #[test]
    fn test_find_path_complex() {
        let room1 = Room {
            entry: doors::Door { lock: DoorLock::UnlockedFromInside },
            exit: doors::Door { lock: DoorLock::UnlockedFromOutside }
        };
        let room2 = Room {
            entry: doors::Door { lock: DoorLock::UnlockedFromInside },
            exit: doors::Door { lock: DoorLock::LockedFromOutside }
        };
        let room3 = Room {
            entry: doors::Door { lock: DoorLock::LockedFromInside },
            exit: doors::Door { lock: DoorLock::UnlockedFromOutside }
        };
        let room_grid = RoomGrid { rooms: vec![room1, room2, room3] };
        let path = core_find_path(0, 2, &room_grid);
        assert_eq!(path, None); // No path should exist as room2's exit is locked from inside.
    }
}
