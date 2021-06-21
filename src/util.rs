
//these numbers pulled from photoshop to get exact pixel size for background
pub const LEFT_WALL: i32 = 248;
pub const RIGHT_WALL: i32 = 1208;
pub const TOP_WALL: i32 = 72;
pub const BOT_WALL: i32 = 648;

// ------------------------------------------- Vec2 --------------------------------------------
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {

    pub fn new(x: T, y: T) -> Vec2<T> {
        // Shorthand notation
        Vec2 { x, y }
    }

    //pub fn x(&self) -> T { self.x }
    //pub fn y(&self) -> T { self.y }
}


//
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
