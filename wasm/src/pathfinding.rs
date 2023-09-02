use std::collections::HashSet;
use crate::rooms::Room;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
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

    pub fn find_path(&mut self, start_room: RoomId, grid: &RoomGrid) -> Option<Vec<RoomId>> {
        self.dfs(start_room, grid);
        if !self.path.is_empty() {
            Some(self.path.clone())
        } else {
            None
        }
    }

    fn dfs(&mut self, current: RoomId, grid: &RoomGrid) {
        if self.visited.contains(&current) {
            return;
        }

        self.visited.insert(current.clone());
        self.path.push(current.clone());

        for neighbor in grid.neighbors(&current) {
            if !self.visited.contains(&neighbor) {
                if grid.is_accessible(&current, &neighbor) {
                    self.dfs(neighbor, grid);
                }
            }
        }
    }
}

pub struct RoomGrid {
    rooms: Vec<Room>,
}

impl RoomGrid {
    pub fn neighbors(&self, room_id: &RoomId) -> Vec<RoomId> {
        vec![RoomId(room_id.0 + 1)]
    }

    pub fn is_accessible(&self, current: &RoomId, neighbor: &RoomId) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doors::{Door, DoorLock};
    use crate::rooms::Room;

    #[test]
    fn test_basic_pathfinding() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![RoomId(0), RoomId(1), RoomId(2)]);
    }

    #[test]
    fn test_no_path() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert!(path.is_none());
    }

    #[test]
    fn test_same_start_and_end() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert_eq!(path, Some(vec![RoomId(0)])); // Only the start room
    }

    #[test]
    fn test_one_step_path() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert_eq!(path, Some(vec![RoomId(0), RoomId(1)]));
    }

    #[test]
    fn test_all_rooms_locked() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert!(path.is_none());
    }

    #[test]
    fn test_alternate_paths() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert!(path.is_some());
        assert_ne!(path, Some(vec![RoomId(0), RoomId(1), RoomId(2)])); // The middle room should be skipped
    }

    #[test]
    fn test_long_chain_blocked() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

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

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), &room_grid);

        assert_eq!(path, Some(vec![RoomId(0), RoomId(1), RoomId(3)])); // The third room is skipped
    }
}

