use crate::doors::{Room, Door, LockState};

pub struct Visualization {
    rooms: Vec<Room>,
}

impl Visualization {
    pub fn new() -> Self {
        Visualization {
            rooms: vec![],
        }
    }

    pub fn run(&mut self) {
        // Here, you will add the code to set up and run your visualization
        println!("Running the visualization...");
    }
}
