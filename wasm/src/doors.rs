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
    pub fn new(lock: DoorLock) -> Self {
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
