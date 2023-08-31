use crate::doors::{Room, Door, LockState};
use three::Object;

pub struct Visualization {
    window: three::Window,
    rooms: Vec<Room>,
}

impl Visualization {
    pub fn new() -> Self {
        // Create a new window
        let title = "Room Visualization with three-rs";
        let window = three::Window::new(title);

        Visualization {
            window,
            rooms: vec![],
        }
    }

    pub fn run(&mut self) {
        // Generate some rooms for demonstration
        let rooms_to_visualize: Vec<Room> = (0..10).map(|_| Room::random_room()).collect();
        self.rooms.extend(rooms_to_visualize.iter().cloned());

        // Create and visualize the rooms
        for room in &rooms_to_visualize {
            self.visualize_room(room);
        }

        // Set the camera
        let center = [0.0, 0.0];
        let yextent = 1.0;
        let zrange = -1.0 .. 1.0;
        let camera = self.window.factory.orthographic_camera(center, yextent, zrange);

        // Main rendering loop
        while self.window.update() {
            self.window.render(&camera);
        }
    }

    fn visualize_room(&mut self, room: &Room) {
        // Visualize the room using basic shapes
        // For now, let's represent a room as a cube and doors as smaller cubes on its sides

        let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); // Room represented as a cuboid
        let material = three::material::Basic {
            color: 0xFFFFFF,
            .. Default::default()
        };
        let room_mesh = self.window.factory.mesh(geometry, material);
        self.window.scene.add(&room_mesh);

        // Visualize the entry and exit doors on the room
        self.visualize_door(&room.entry);
        self.visualize_door(&room.exit);
    }

    fn visualize_door(&mut self, door: &Door) {
        // Visualize the door using a basic shape
        // For simplicity, let's represent a door as a smaller cube

        let geometry = three::Geometry::cuboid(0.2, 0.5, 0.05); // Door represented as a thin cuboid
        let material = three::material::Basic {
            color: match door.lock_state {
                LockState::Locked => 0xFF0000,       // Red for locked door
                LockState::LockedFromInside => 0xFFFF00, // Yellow for locked from inside
                LockState::LockedFromOutside => 0x00FF00, // Green for locked from outside
            },
            .. Default::default()
        };
        let door_mesh = self.window.factory.mesh(geometry, material);
        self.window.scene.add(&door_mesh);
    }
}
