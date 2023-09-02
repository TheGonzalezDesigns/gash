use std::collections::HashSet;

// Assuming Room has a unique identifier (like a usize or u32)
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct RoomId(usize);

pub struct PathFinder {
    visited: HashSet<RoomId>,
    path: Vec<RoomId>,
}

impl PathFinder {
    pub fn new() -> Self {
        PathFinder {
            visited: HashSet::new(),
            path: Vec::new(),
        }
    }

    pub fn find_path(&mut self, start_room: RoomId, grid: &RoomGrid) -> &Vec<RoomId> {
        self.dfs(start_room, grid);
        &self.path
    }

    fn dfs(&mut self, current: RoomId, grid: &RoomGrid) {
        if self.visited.contains(&current) {
            return;
        }

        self.visited.insert(current.clone());
        self.path.push(current.clone());

        // Assuming RoomGrid provides a method `neighbors` that gives the neighbors of a room
        for neighbor in grid.neighbors(&current) {
            if !self.visited.contains(&neighbor) {
                // Check if the door between current and neighbor is not locked
                // This assumes some functionality in Room or Door to check if a room is accessible from another
                if grid.is_accessible(&current, &neighbor) {
                    self.dfs(neighbor, grid);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doors::{Door, DoorLock};
    use crate::rooms::Room;

    #[test]
    fn test_basic_pathfinding() {
        // Simple scenario: 3 rooms in a row, all doors unlocked
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        struct MockRoomGrid {
            rooms: Vec<Room>,
        }

        impl MockRoomGrid {
            pub fn neighbors(&self, room: &RoomId) -> Vec<RoomId> {
                vec![RoomId(room.0 + 1)]
            }

            pub fn is_accessible(&self, current: &RoomId, neighbor: &RoomId) -> bool {
                true
            }
        }

        let room_grid = MockRoomGrid { rooms };

        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn test_no_path() {
        // Scenario: 3 rooms in a row, middle room has locked doors
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 2);
        assert!(path.is_none());
    }

    #[test]
    fn test_same_start_and_end() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 0);
        assert_eq!(path, Some(vec![0])); // Only the start room
    }

    #[test]
    fn test_one_step_path() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 1);
        assert_eq!(path, Some(vec![0, 1]));
    }

    #[test]
    fn test_all_rooms_locked() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 1);
        assert!(path.is_none());
    }

    #[test]
    fn test_alternate_paths() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 2);
        assert!(path.is_some());
        assert_ne!(path, vec![0, 1, 2]); // The middle room should be skipped
    }

    #[test]
    fn test_long_chain_blocked() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 3);
        assert!(path.is_none()); // No path since the middle rooms block the way
    }

    #[test]
    fn test_non_linear_paths() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let path = find_path(&rooms, 0, 3);
        assert_eq!(path, Some(vec![0, 1, 3])); // The third room is skipped
    }
}

