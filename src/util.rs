// Constants to clean up SDLCore initiation
pub const TITLE: &str = "Roguelike";
pub const VSYNC: bool = true;
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;


//these numbers pulled from photoshop to get exact pixel size for background
pub const LEFT_WALL: i32 = 248 - 64;
pub const RIGHT_WALL: i32 = 1208 + 64;
pub const TOP_WALL: i32 = 72 - 64;
pub const BOT_WALL: i32 = 648 + 64;

//-------------------------------Player/Creature Constants---------------------------------
pub const MAX_HP: i32 = 6;
pub const DEATH_HP: i32 = 0;

// ------------------------------------------- Vec2 --------------------------------------------
#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone)]
pub enum LockState {
    Locked,
    Unlocked,
    NA,
}

// #[derive(Copy, Clone)]
// pub enum BombState {
//     Set,
//     Explode,
//     NA,
// }


//#[derive(Copy, Clone)]
pub enum SpriteID {
    Ground,
    Wall,
    Rock,
    Pit,
    DoorLocked,
    DoorUnlocked,
    Key,
    TrapdoorLocked,
    TrapdoorUnlocked,
    Spike,
    GemRed,
    GemBlue,
    GemYellow,
    Bomb,
    Explosion,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Gem {
    Red,
    Blue,
    Yellow,
    None,
}
