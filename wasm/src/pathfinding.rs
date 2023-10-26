use std::collections::HashSet;
use crate::rooms::{RoomGrid, RoomId};

pub struct PathFinder {
    visited: HashSet<RoomId>,
    path: Vec<RoomId>,
}

impl PathFinder {
    pub fn new() -> Self {
        PathFinder
    }

    pub fn find_path(&self, start_room: RoomId, end_room: RoomId, grid: &RoomGrid) -> Option<Vec<RoomId>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_room, vec![start_room]));

        while let Some((current, path)) = queue.pop_front() {
            if current == end_room {
                return Some(path);
            }
            if visited.insert(current) {
                for neighbor in grid.neighbors(&current) {
                    if !visited.contains(&neighbor) && grid.is_accessible(&current, &neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        queue.push_back((neighbor, new_path));
                    }
                }
            }
        }
        None
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
        let path = pathfinder.find_path(RoomId(0), RoomId(2), &room_grid);

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
        let path = pathfinder.find_path(RoomId(0), RoomId(2), &room_grid);

        assert!(path.is_none());
    }

    #[test]
    fn test_same_start_and_end() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), RoomId(0), &room_grid);

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
        let path = pathfinder.find_path(RoomId(0), RoomId(1), &room_grid);

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
        let path = pathfinder.find_path(RoomId(0), RoomId(1), &room_grid);

        assert!(path.is_none());
    }

    #[test]
    fn test_alternate_paths() {
        let rooms = vec![
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
            Room::new(Door::new(DoorLock::LockedFromInside), Door::new(DoorLock::LockedFromOutside)),
            Room::new(Door::new(DoorLock::UnlockedFromInside), Door::new(DoorLock::UnlockedFromOutside)),
        ];

        let room_grid = RoomGrid { rooms };
        let mut pathfinder = PathFinder::new();
        let path = pathfinder.find_path(RoomId(0), RoomId(3), &room_grid);

        assert_eq!(path, Some(vec![RoomId(0), RoomId(1), RoomId(3)])); // The third room is skipped
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
        let path = pathfinder.find_path(RoomId(0), RoomId(3), &room_grid);

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
        let path = pathfinder.find_path(RoomId(0), RoomId(3), &room_grid);

        assert_eq!(path, Some(vec![RoomId(0), RoomId(1), RoomId(3)])); // The third room is skipped
    }
}
