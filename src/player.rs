use crate::util::*;

pub struct Player {
    pos: Vec2<f32>, // Position of middle of player.
    hitbox: Vec2<u32>, // Hitbox where player takes damage.
    walkbox: Vec2<u32>, // Hitbox involved in collision with rooms.
    speed: f32,
    dir: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::new(512.0, 256.0),
            hitbox: Vec2::new(48, 52),
            walkbox: Vec2::new(40, 24),
            speed: 2.75,
            dir: Direction::Down,
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


        self.pos.x = self.pos.x.clamp(LEFT_WALL as f32 + (self.walkbox.x/2) as f32, RIGHT_WALL as f32 - (self.walkbox.x/2) as f32);
        self.pos.y = self.pos.y.clamp(TOP_WALL as f32 + (self.walkbox.y/2) as f32, BOT_WALL as f32 - (self.walkbox.y/2) as f32);
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32}
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32}

    pub fn get_walkbox_x(&self) -> u32 { self.walkbox.x }
    pub fn get_walkbox_y(&self) -> u32 { self.walkbox.y }

    pub fn get_hitbox_x(&self) -> u32 { self.hitbox.x }
    pub fn get_hitbox_y(&self) -> u32 { self.hitbox.y }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }
}
