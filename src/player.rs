use crate::util::*;
use sdl2::rect::Rect;

pub struct Player {
    pos: Vec2<f32>, // Position of middle of player.
    hitbox: Vec2<f32>, // Hitbox where player takes damage.
    walkbox: Hitbox, // Hitbox involved in collision with rooms.
    speed: f32,
    dir: Direction,
}


impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::new(3.0, 3.0),
            hitbox: Vec2::new(0.75, 0.8125),
            walkbox: Hitbox::new(1.0, 1.0, 0.625, 0.375),
            speed: 5.0,
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


        // Wall collision
        self.pos.x = self.pos.x.clamp(LEFT_WALL as f32 + (self.walkbox.x/2.0) as f32, RIGHT_WALL as f32 - (self.walkbox.x/2.0) as f32);
        self.pos.y = self.pos.y.clamp(TOP_WALL as f32 + (self.walkbox.y/2.0) as f32, BOT_WALL as f32 - (self.walkbox.y/2.0) as f32);

        // Hacky af block collision that needs to be changed later
        let inter_rect = self.get_walkbox_world().intersection(Rect::new(174 * 4, 82 * 4, 64, 64));
        let mut boop = match inter_rect {
            Some(x) => x,
            None => return, // If no intersection just leave function, we're done
        };

        //println!("{}, {}", boop.width(), boop.height());
        let mut x_offset = boop.width() as i32;
        let mut y_offset = boop.height() as i32;

        if self.pos.x < boop.x() as f32 {
            // TO THE LEFT OF ROCK
            y_offset = 0;
        }
        if self.pos.x > (boop.x() + boop.width() as i32) as f32 {
            // TO THE RIGHT OF ROCK
            x_offset *= -1;
            y_offset = 0;
        }
        if self.pos.y < boop.y() as f32 {
            // ABOVE ROCK
            x_offset = 0;
        }
        if self.pos.y > (boop.y() + boop.height() as i32) as f32 {
            // BELOW ROCK
            x_offset = 0;
            y_offset *= -1;
        }

        self.pos.x -= x_offset as f32;
        self.pos.y -= y_offset as f32;
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32}
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32}

    pub fn get_walkbox(&self) -> Hitbox { self.walkbox }
    pub fn get_walkbox_world(&self) -> Rect { Rect::new(
                                                    self.pos.x as i32 - self.walkbox.x,
                                                    self.pos.y as i32 - self.walkbox.y,
                                                    self.walkbox.width,
                                                    self.walkbox.height,
                                                    )
    }

    pub fn get_hitbox_x(&self) -> f32 { self.hitbox.x }
    pub fn get_hitbox_y(&self) -> f32 { self.hitbox.y }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }
}
