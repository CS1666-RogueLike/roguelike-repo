use sdl2::rect::Rect;
//these numbers pulled from photoshop to get exact pixel size for background
pub const LEFT_WALL: f32 = 248.0;
pub const RIGHT_WALL: f32 = 1208.0;
pub const TOP_WALL: f32 = 72.0;
pub const BOT_WALL: f32 = 648.0;

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

pub struct Hitbox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Hitbox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Hitbox {
        Hitbox {x, y, width, height}
        }

    pub fn translate(&self) -> Rect {
        Rect::new(
        (self.x*64.0+LEFT_WALL) as i32,
        (self.y*64.0+TOP_WALL) as i32,
        (self.width*64.0) as u32,
        (self.height*64.0) as u32,
        //u32::try_from(self.width*64.0).Ok(),
        //u32::try_from(self.height*64.0).Ok(),
        )
    }
}

pub trait Translate {
    fn translate(&self) -> i32;

}

impl Translate for f32 {
    fn translate(&self) -> i32
    {
        2
    }

}

//
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
