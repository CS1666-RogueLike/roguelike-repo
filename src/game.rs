
use crate::player::*;
use crate::map::*;

pub struct Game {
    pub player: Player,
    pub map: Map,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(),
            map: Map::new(),
        }
    }
}
