
pub struct Player {
    pos_x: i32,
    pos_y: i32,
    hbox_x: u32,
    hbox_y: u32,
    speed: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x: 128,
            pos_y: 128,
            hbox_x: 64,
            hbox_y: 32,
            speed: 1,
        }
    }

    pub fn update_pos(& mut self, mov_x: i32, mov_y: i32) {
        self.pos_x += mov_x * self.speed;
        self.pos_y += mov_y * self.speed;
    }

    pub fn get_pos_x(&self) -> i32 { self.pos_x }
    pub fn get_pos_y(&self) -> i32 { self.pos_y }

    pub fn get_hbox_x(&self) -> u32 { self.hbox_x }
    pub fn get_hbox_y(&self) -> u32 { self.hbox_y }
}
