use crate::doors::Door;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Room {
    entry: Door,
    exit: Door,
}

impl Room {
    fn new(entry: Door, exit: Door) -> Self {
        Room { entry, exit }
    }
}

#[wasm_bindgen]
pub struct RoomGrid {
    rooms: Vec<Room>,
}

#[wasm_bindgen]
impl RoomGrid {
    pub fn new(num_rooms: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rooms = Vec::with_capacity(num_rooms);

        for _ in 0..num_rooms {
            let entry = Door::new(DoorLock::random(&mut rng), DoorLock::random(&mut rng));
            let exit = Door::new(DoorLock::random(&mut rng), DoorLock::random(&mut rng));
            rooms.push(Room::new(entry, exit));
        }

        RoomGrid { rooms }
    }
}
