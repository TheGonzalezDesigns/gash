use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug)]
pub enum LockState {
    LockedFromInside,
    LockedFromOutside,
    Locked,
}

#[derive(Clone, Copy, Debug)]
pub struct Door {
    pub lock_state: LockState,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub entry: Door,
    pub exit: Door,
}

impl Room {
    // Function to generate a random room
    pub fn random_room() -> Self {
        let mut rng = rand::thread_rng();
        let lock_states = [
            LockState::LockedFromInside,
            LockState::LockedFromOutside,
            LockState::Locked,
        ];

        Room {
            entry: Door {
                lock_state: *lock_states.choose(&mut rng).unwrap(),
            },
            exit: Door {
                lock_state: *lock_states.choose(&mut rng).unwrap(),
            },
        }
    }
}
