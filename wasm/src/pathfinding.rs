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

