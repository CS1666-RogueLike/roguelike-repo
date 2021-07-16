use crate::util::*;

pub trait Tile {
    // Determines what tile sprite to use when drawing.
    fn sprite(&self) -> SpriteID;

    // Determines the walkability of the tile, which informs what entities can pass over it.
    fn walkability(&self) -> Walkability;

    // Determines what happens when the player walks over this tile.
    // Note that this returns an enum that the game handles instead of directly changing the state
    // itself. This is because the tile is extremely deep in the hierarchy, and doesn't have
    // references to the data necessary to make changes. Actions that only effect the tile can be
    // done within the function, but anything that interacts with outside structs must be done
    // outside.
    fn on_walkover(& mut self) -> WalkoverAction;

    // Methods for locking and unlocking doors. For all tiles that do not lock/unlock, do nothing for
    // lock and unlock and for get_lock_state return NA (Not Applicable)
    // By implementing these for all tiles we don't have to do weird casting, replacement in place with
    // a new tile, or keeping track of what doors a room has for unlocking.
    fn lock(& mut self);
    fn unlock(& mut self);
    fn get_lock_state(&self) -> LockState;

    // Used for dropping the gem. Should only do something for ground tiles
    fn place_gem(&mut self, color: Gem);
}

pub enum Walkability {
    Floor, // Normal ground.
    Pit, // A pit. Can't be walked over but can be flown over.
    Rock, // A mid room obstacle. Can't be walked over but can be flown over. Blocks projectiles.
    Wall, // Outer walls of the room. Nothing can pass over.
    Spike, // Spike tile that causes damage when a player crosses
}

// what is it doing to the player/entity that is walking over it
#[derive(Debug)]
pub enum WalkoverAction {
    DoNothing,
    ChangeRooms,
    GivePlayerKey,
    GoToNextFloor,
    Damage,
    BuffHealth,
    BuffDamage,
    BuffSpeed,
}

pub struct Ground {
    pub gem: Gem,
}
impl Tile for Ground {
    fn sprite(&self) -> SpriteID {
        match self.gem {
            Gem::Red => SpriteID::GemRed,
            Gem::Blue => SpriteID::GemBlue,
            Gem::Yellow => SpriteID::GemYellow,
            Gem::None => SpriteID::Ground,
        }
    }
    fn walkability(&self) -> Walkability { Walkability::Floor }
    fn on_walkover(& mut self) -> WalkoverAction {
        let ret = match self.gem {
            Gem::Red => WalkoverAction::BuffHealth,
            Gem::Blue => WalkoverAction::BuffSpeed,
            Gem::Yellow => WalkoverAction::BuffDamage,
            Gem::None => WalkoverAction::DoNothing,
        };
        if self.gem != Gem::None {
            self.gem = Gem::None;
        }
        // Variable that stores result of match so we can change gem state
        ret
    }

    fn lock(& mut self) {}
    fn unlock(& mut self) {}
    fn get_lock_state(&self) -> LockState { LockState::NA }
    fn place_gem(&mut self, color: Gem) { self.gem = color; }
}


pub struct Rock {}
impl Tile for Rock {
    fn sprite(&self) -> SpriteID { SpriteID::Rock }
    fn walkability(&self) -> Walkability { Walkability::Rock }
    fn on_walkover(& mut self) -> WalkoverAction { WalkoverAction::DoNothing }
    fn lock(& mut self) {}
    fn unlock(& mut self) {}
    fn get_lock_state(&self) -> LockState { LockState::NA }
    fn place_gem(&mut self, color: Gem) {}
}

pub struct Wall {}
impl Tile for Wall {
    fn sprite(&self) -> SpriteID { SpriteID::Wall }
    fn walkability(&self) -> Walkability { Walkability::Wall }
    fn on_walkover(& mut self) -> WalkoverAction { WalkoverAction::DoNothing }
    fn lock(& mut self) {}
    fn unlock(& mut self) {}
    fn get_lock_state(&self) -> LockState { LockState::NA }
    fn place_gem(&mut self, color: Gem) {}
}

pub struct Pit {}
impl Tile for Pit {
    fn sprite(&self) -> SpriteID { SpriteID::Pit }
    fn walkability(&self) -> Walkability { Walkability::Pit }
    fn on_walkover(& mut self) -> WalkoverAction { WalkoverAction::DoNothing }
    fn lock(& mut self) {}
    fn unlock(& mut self) {}
    fn get_lock_state(&self) -> LockState { LockState::NA }
    fn place_gem(&mut self, color: Gem) {}
}

pub struct Spike {}
impl Tile for Spike {
    fn sprite(&self) -> SpriteID { SpriteID::Spike }
    fn walkability(&self) -> Walkability { Walkability::Spike }
    fn on_walkover(& mut self) -> WalkoverAction { WalkoverAction::Damage }
    fn lock(& mut self) {}
    fn unlock(& mut self) {}
    fn get_lock_state(&self) -> LockState { LockState::NA }
    fn place_gem(&mut self, color: Gem) {}
}

pub struct Door {
    pub(crate) lock: LockState,
    pub(crate) position: Direction,
}
impl Tile for Door {
    fn sprite(&self) -> SpriteID {
        match self.lock {
            LockState::Locked => SpriteID::DoorLocked,
            LockState::Unlocked => SpriteID::DoorUnlocked,
            LockState::NA => panic!("Locking tile shouldn't have NA!!!")
        }
    }
    fn walkability(&self) -> Walkability {
        match self.lock {
            LockState::Locked => Walkability::Wall,
            LockState::Unlocked => Walkability::Floor,
            LockState::NA => panic!("Locking tile shouldn't have NA!!!")
        }
    }
    fn on_walkover(& mut self) -> WalkoverAction { WalkoverAction::ChangeRooms }
    fn lock(&mut self) { self.lock = LockState::Locked; }
    fn unlock(&mut self) { self.lock = LockState::Unlocked; }
    fn get_lock_state(&self) -> LockState { self.lock }
    fn place_gem(&mut self, color: Gem) {}
}

pub struct Key {
    pub(crate) has_key: bool
}
impl Tile for Key {
    fn sprite(&self) -> SpriteID {
        if self.has_key { SpriteID::Key }
        else            { SpriteID::Ground }
    }
    fn walkability(&self) -> Walkability { Walkability::Floor }
    fn on_walkover(& mut self) -> WalkoverAction {
        if self.has_key {
            self.has_key = false;
            WalkoverAction::GivePlayerKey
        }
        else {
            WalkoverAction::DoNothing
        }
    }
    fn lock(& mut self) {}
    fn unlock(& mut self) {}
    fn get_lock_state(&self) -> LockState { LockState::NA }
    fn place_gem(&mut self, color: Gem) {}
}

pub struct Trapdoor {
    pub(crate) lock: LockState,
}
impl Tile for Trapdoor {
    fn sprite(&self) -> SpriteID {
        match self.lock {
            LockState::Locked => SpriteID::TrapdoorLocked,
            LockState::Unlocked => SpriteID::TrapdoorUnlocked,
            LockState::NA => panic!("Locking tile shouldn't have NA!!!")
        }
    }
    fn walkability(&self) -> Walkability { Walkability::Floor }
    fn on_walkover(&mut self) -> WalkoverAction {
        WalkoverAction::GoToNextFloor
    }
    fn lock(&mut self) { self.lock = LockState::Locked; }
    fn unlock(&mut self) { self.lock = LockState::Unlocked; }
    fn get_lock_state(&self) -> LockState { self.lock }
    fn place_gem(&mut self, color: Gem) {}
}
