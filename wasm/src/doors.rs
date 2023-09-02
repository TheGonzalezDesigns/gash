use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorLock {
    LockedFromInside,
    LockedFromOutside,
    UnlockedFromInside,
    UnlockedFromOutside,
}

impl DoorLock {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..=3) {
            0 => DoorLock::LockedFromInside,
            1 => DoorLock::LockedFromOutside,
            2 => DoorLock::UnlockedFromInside,
            _ => DoorLock::UnlockedFromOutside,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Door {
    pub lock: DoorLock,
}

impl Door {
    pub fn new(lock: DoorLock) -> Self { // <-- Add pub
        Door { lock }
    }

    pub fn is_accessible_from_inside(&self) -> bool {
        match self.lock {
            DoorLock::LockedFromInside | DoorLock::LockedFromOutside => false,
            DoorLock::UnlockedFromInside | DoorLock::UnlockedFromOutside => true,
        }
    }

    pub fn is_accessible_from_outside(&self) -> bool {
        match self.lock {
            DoorLock::LockedFromInside | DoorLock::LockedFromOutside => false,
            DoorLock::UnlockedFromInside | DoorLock::UnlockedFromOutside => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_accessibility_from_inside() {
        let door1 = Door::new(DoorLock::LockedFromInside);
        assert_eq!(door1.is_accessible_from_inside(), false);

        let door2 = Door::new(DoorLock::LockedFromOutside);
        assert_eq!(door2.is_accessible_from_inside(), false);

        let door3 = Door::new(DoorLock::UnlockedFromInside);
        assert_eq!(door3.is_accessible_from_inside(), true);

        let door4 = Door::new(DoorLock::UnlockedFromOutside);
        assert_eq!(door4.is_accessible_from_inside(), true);
    }

    #[test]
    fn test_door_accessibility_from_outside() {
        let door1 = Door::new(DoorLock::LockedFromInside);
        assert_eq!(door1.is_accessible_from_outside(), false);

        let door2 = Door::new(DoorLock::LockedFromOutside);
        assert_eq!(door2.is_accessible_from_outside(), false);

        let door3 = Door::new(DoorLock::UnlockedFromInside);
        assert_eq!(door3.is_accessible_from_outside(), true);

        let door4 = Door::new(DoorLock::UnlockedFromOutside);
        assert_eq!(door4.is_accessible_from_outside(), true);
    }

    #[test]
    fn test_door_creation() {
        let states = [
            DoorLock::LockedFromInside,
            DoorLock::LockedFromOutside,
            DoorLock::UnlockedFromInside,
            DoorLock::UnlockedFromOutside,
        ];

        for &state in states.iter() {
            let door = Door::new(state);
            assert_eq!(door.lock, state);
        }
    }
}
