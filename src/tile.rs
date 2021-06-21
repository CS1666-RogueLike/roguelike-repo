

// TODO: REWORK THIS AFTER THINKING OUT EDGE CASES

trait Tile {
    // Determines the walkability of the tile, which informs what entities can pass over it.
    fn walkability() -> Walkability;

    // Determines the behavior upon walkover and the texture used when drawing.
    fn tile_type() -> TileType;
}

enum Walkability {
    Floor, // Normal ground.
    Pit, // A pit. Can't be walked over but can be flown over.
    Rock, // A mid room obstacle. Can't be walked over but can be flown over. Blocks projectiles.
    Wall, // Outer walls of the room. Nothing can pass over.
}

enum TileType {
    Ground,
    Spike,
    Rock,
    Wall,
    Door,
    Item,
}

// ------- GROUND --------
struct Ground {}
impl Tile for Ground {
    fn walkability() -> Walkability { Walkability::Floor }

    fn tile_type() -> TileType { TileType::Ground }
}
