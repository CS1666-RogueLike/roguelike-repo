
use crate::player::*;
use crate::map::*;
use crate::room::*;
use crate::util::*;

use std::time::Instant;

pub struct Game {
    pub player: Player,
    pub map: Map,

    // Current room
    pub cr: Vec2<i32>,

    // Current floor
    pub cf: usize,

    // Used to handle transitions
    pub game_state: GameState,
    pub transition_start: Instant,
    pub trans_dir: Direction,

    pub init_time: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(),
            map: Map::new(),
            cr: Vec2::new(START_X, START_Y),
            cf: 0,
            game_state: GameState::Gameplay,
            transition_start: Instant::now(),
            trans_dir: Direction::Up,
            init_time: Instant::now(),
            //cr: Vec2::new(1, 3),
        }
    }

    pub fn current_room(&self) -> &Room {
        // & is to return ref, * is to unbox
        &*self.map.floors[self.cf].rooms[self.cr.y as usize][self.cr.x as usize]
    }

    pub fn current_room_mut(& mut self) -> &mut Room {
        // & is to return ref, * is to unbox
        &mut*self.map.floors[self.cf].rooms[self.cr.y as usize][self.cr.x as usize]
    }
}

pub enum GameState {
    Gameplay,
    BetweenRooms,
}
