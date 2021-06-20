
use crate::player::*;

pub struct Game {
    pub player: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(),
        }
    }
}
