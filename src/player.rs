use crate::util::*;
use sdl2::rect::Rect;

pub struct Player {
    pub pos: Vec2<f32>, // Position of middle of player.

    // TODO: REWORK INTO INDIVIDUAL TRAITS SO THEY CAN BE USED WITH ENEMIES
    pub hitbox: Vec2<u32>, // Hitbox where player takes damage.
    pub walkbox: Rect, // Hitbox involved in collision with rooms.

    pub speed: f32,
    pub dir: Direction,
    pub hp: i32,    //store the health for player
    pub m_hp: i32,

    pub prev_frame_tile: Vec2<i32>,
    pub current_frame_tile: Vec2<i32>,
}

pub trait Health {
    // TODO: Add iframe setup so player can't be damaged for a bit after being damaged.
    // Otherwise a spike tile doing half a heart of damage would kill a 3 heart player in 6 frames.

    fn max_hp(&self) -> i32;  //the maximum HP the entity can have
    fn health(&self) -> i32;    // the current HP the entity has
    fn damage(&mut self, d: i32) -> i32;  // applying the amount of damage received
    //fn damage_type(&self) -> i32; // the damage category
    //fn bonus_type(&self) -> i32;    // the type of bonus dropped by enemy
    //fn percent_damaged(&self) -> f32;
    fn heal(&mut self, h: i32) -> i32;

}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 5 * 64) as f32 + 40.0),
            hitbox: Vec2::new(48, 52),
            walkbox: Rect::new(20, 12, 40, 24),
            speed: 3.5,
            dir: Direction::Down,
            hp: MAX_HP,
            m_hp: MAX_HP,
            prev_frame_tile: Vec2::new(8, 5),
            current_frame_tile: Vec2::new(8, 5),
        }
    }

    pub fn update_pos(& mut self, mut mov_vec: Vec2<f32>) {

        // Fix diagonal directions giving more speed than one direction
        if mov_vec.x != 0.0 && mov_vec.y != 0.0 {
            // The number approximates sqrt(2)/2, the position on the unit circle at 45 degrees
            // that is 1 unit away from the center.
            mov_vec.x *= 0.707106;
            mov_vec.y *= 0.707106;
        }

        // Udate position using movement vector and speed
        self.pos.x += mov_vec.x * self.speed;
        self.pos.y += mov_vec.y * self.speed;

        // COLLISION CODE HAS BEEN MOVED TO MANAGER STRUCT, AS NEW COLLISION REQUIRES KNOWLEDGE OF
        // MAP STATE WHICH IS ABOVE THE PLAYER
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32}
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32}

    pub fn get_walkbox(&self) -> Rect { self.walkbox }
    pub fn get_walkbox_world(&self) -> Rect { Rect::new(
                                                    self.pos.x as i32 - self.walkbox.x(),
                                                    self.pos.y as i32 - self.walkbox.y(),
                                                    self.walkbox.width(),
                                                    self.walkbox.height(),
                                                    )
    }

    pub fn get_hitbox_x(&self) -> u32 { self.hitbox.x }
    pub fn get_hitbox_y(&self) -> u32 { self.hitbox.y }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }
}

impl Health for Player {
    fn max_hp(&self) -> i32 { return self.m_hp; }
	fn health(&self) -> i32 { return self.hp; }
    fn damage(&mut self, d: i32) -> i32 {
        self.hp -= d;
        if self.hp <= 0 {
            self.hp = 0;
            }
        self.hp // I changed this and the next one to use rust style implicit returns
    }
    fn heal(&mut self, h: i32) -> i32 {
        self.hp = self.hp + h;
        if self.hp > self.m_hp {
            self.hp = self.m_hp;
        }
        self.hp
    }

	}
