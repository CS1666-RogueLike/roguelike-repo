
use crate::player::*;
use crate::map::*;
use crate::room::*;
use crate::util::*;
use crate::entity::*;

pub struct Game {
    pub player: Player,
    pub map: Map,
    pub enemies: Vec<Enemy>,

    pub cr: Vec2<i32>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(),
            map: Map::new(),
            cr: Vec2::new(3, 4),
            enemies: Vec::new(),
            //cr: Vec2::new(1, 3),
        }
    }

    pub fn current_room(&self) -> &Room {
        // & is to return ref, * is to unbox
        &*self.map.floors[0].rooms[self.cr.y as usize][self.cr.x as usize]
    }

    pub fn current_room_mut(& mut self) -> &mut Room {
        // & is to return ref, * is to unbox
        &mut*self.map.floors[0].rooms[self.cr.y as usize][self.cr.x as usize]
    }
}
