
use crate::player::*;
use crate::map::*;
use crate::room::*;
use crate::util::*;
use crate::entity::*;

pub struct Game {
    pub player: Player,
    pub map: Map,
    pub enemies: Vec<Enemy>,

    // Current room
    pub cr: Vec2<i32>,

    // Current floor
    pub cf: usize,
}

impl Game {
    pub fn new() -> Game {
        let mut enemies = Vec::new();
        enemies.push(Enemy::new( Vec2::new((LEFT_WALL + 12 * 64) as f32 + 32.0, (TOP_WALL + 7 * 64) as f32 + 40.0), EnemyKind::Speed, Vec2::new(3,4), 0));
        enemies.push(Enemy::new( Vec2::new((LEFT_WALL + 14 * 64) as f32 + 32.0, (TOP_WALL + 9 * 64) as f32 + 40.0), EnemyKind::Health, Vec2::new(4,4), 1));
        enemies.push(Enemy::new( Vec2::new((LEFT_WALL + 3 * 64) as f32 + 32.0, (TOP_WALL + 6 * 64) as f32 + 40.0), EnemyKind::Attack, Vec2::new(4,3), 0));
        Game {
            player: Player::new(),
            map: Map::new(),
            cr: Vec2::new(3, 4),
            cf: 0,
            enemies: enemies,
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
