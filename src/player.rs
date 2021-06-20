use crate::util::*;

//these numbers pulled from photoshop to get exact pixel size for background
const COLL_L: i32 = 248;
const COLL_R: i32 = 1208;
const COLL_U: i32 = 72;
const COLL_D: i32 = 648;

pub struct Player {
    pos: Vec2<i32>,
    hbox: Vec2<u32>,
    speed: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::new(512, 256),
            hbox: Vec2::new(64, 64),
            speed: 3,
        }
    }

    pub fn update_pos(& mut self, mov_x: i32, mov_y: i32) {
        self.pos.x += mov_x * self.speed;
        self.pos.y += mov_y * self.speed;


        self.pos.x = self.pos.x.clamp(COLL_L + (self.hbox.x/2) as i32, COLL_R - (self.hbox.x/2) as i32);
        self.pos.y = self.pos.y.clamp(COLL_U + (self.hbox.y/2) as i32, COLL_D - (self.hbox.y/2) as i32);
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x }
    pub fn get_pos_y(&self) -> i32 { self.pos.y }

    pub fn get_hbox_x(&self) -> u32 { self.hbox.x }
    pub fn get_hbox_y(&self) -> u32 { self.hbox.y }
}
