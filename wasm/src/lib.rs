mod doors;
mod rooms;
mod pathfinding;

pub use rooms::{Room, RoomGrid};
pub use pathfinding::PathFinder;  // Expose the PathFinder struct

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen]
pub fn greet() {
    web_sys::console::log_1(&"Hello from Rust!".into());
}

#[wasm_bindgen]
pub fn wasm_find_path(start_room: usize, room_grid_js: &JsValue) -> Option<Vec<usize>> {
    let grid: rooms::RoomGrid = serde_wasm_bindgen::from_value(room_grid_js.clone()).unwrap();
    let mut pathfinder = pathfinding::PathFinder::new();
    let path = pathfinder.find_path(rooms::RoomId(start_room), &grid);
    path.map(|p| p.into_iter().map(|room_id| room_id.0).collect())
}

#[wasm_bindgen]
pub fn generate_random_rooms(num_rooms: usize) -> JsValue {
    let grid = RoomGrid::new(num_rooms);
    serde_wasm_bindgen::to_value(&grid).unwrap()
}
