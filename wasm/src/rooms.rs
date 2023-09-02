use crate::doors::Door;
use rand::Rng;

pub struct Room {
    entry_door: Door,
    exit_door: Door,
}

impl Room {
    // Create a new room with two doors
    pub fn new(entry_door: Door, exit_door: Door) -> Self {
        Room { entry_door, exit_door }
    }

    // Generate a random room from available doors
    pub fn random(available_doors: &Vec<Door>) -> Self {
        let mut rng = rand::thread_rng();
        let entry_index = rng.gen_range(0..available_doors.len());
        let exit_index = rng.gen_range(0..available_doors.len());

        // Ensure the entry and exit doors are not the same
        while entry_index == exit_index {
            exit_index = rng.gen_range(0..available_doors.len());
        }

        Room::new(available_doors[entry_index].clone(), available_doors[exit_index].clone())
    }

    // Check if a room is accessible based on the state of its doors
    pub fn is_accessible(&self) -> bool {
        // Logic to determine if the room is accessible based on the state of its doors
        // For example, a room is not accessible if both doors are locked from the outside.
        !(self.entry_door.is_locked_from_outside && self.exit_door.is_locked_from_outside)
    }
}
