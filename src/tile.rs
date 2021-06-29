
pub trait Tile {
    // Determines the walkability of the tile, which informs what entities can pass over it.
    fn walkability(&self) -> Walkability;

    // Determines what happens when the player walks over this tile.
    // Note that this returns an enum that the game handles instead of directly changing the state
    // itself. This is because the tile is extremely deep in the hierarchy, and doesn't have
    // references to the data necessary to make changes. Actions that only effect the tile can be
    // done within the function, but anything that interacts with outside structs must be done
    // outside.
    fn on_walkover(&self) -> WalkoverAction;
}

pub enum Walkability {
    Floor, // Normal ground.
    Pit, // A pit. Can't be walked over but can be flown over.
    Rock, // A mid room obstacle. Can't be walked over but can be flown over. Blocks projectiles.
    Wall, // Outer walls of the room. Nothing can pass over.
}

// what is it doing to the player/entity that is walking over it
pub enum WalkoverAction {
    DoNothing,
}

pub struct Ground {}
impl Tile for Ground {
    fn walkability(&self) -> Walkability { Walkability::Floor }
    fn on_walkover(&self) -> WalkoverAction { WalkoverAction::DoNothing }
}


pub struct Rock {}
impl Tile for Rock {
    fn walkability(&self) -> Walkability { Walkability::Rock }
    fn on_walkover(&self) -> WalkoverAction { WalkoverAction::DoNothing }
}


pub struct Wall {}
impl Tile for Wall {
    fn walkability(&self) -> Walkability { Walkability::Wall }
    fn on_walkover(&self) -> WalkoverAction { WalkoverAction::DoNothing }
}

pub struct Pit {}
impl Tile for Pit {
    fn walkability(&self) -> Walkability { Walkability::Pit }
    fn on_walkover(&self) -> WalkoverAction { WalkoverAction::DoNothing }
}
