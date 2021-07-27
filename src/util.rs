// Constants to clean up SDLCore initiation
pub const TITLE: &str = "Roguelike";
pub const VSYNC: bool = true;
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub const TILE_WIDTH: i32 = 64;

//these numbers pulled from photoshop to get exact pixel size for background
pub const LEFT_WALL: i32 = 248 - TILE_WIDTH;
pub const RIGHT_WALL: i32 = 1208 + TILE_WIDTH;
pub const TOP_WALL: i32 = 72 - TILE_WIDTH;
pub const BOT_WALL: i32 = 648 + TILE_WIDTH;

//-------------------------------Player/Creature Constants---------------------------------

//----Player Stats----
pub const P_MAX_HP: i32 = 6;
pub const P_DEATH_HP: i32 = 0; 
pub const P_INVINCIBILITY_TIME: u64 = 1750; //The time (in milliseconds) that the player is invunerable
//after they take damage

//pub const P_SPEED: 

pub const ENEMY_INTERSECTION_DAMAGE: i32 = 1; //Damage taken by player when they collide with enemy

//----Health Enemy Stats----
pub const HEALTH_ENEMY_ATK_DMG: i32 = 2; //The damage of this enemy's projectile attack
pub const HEALTH_ENEMY_ATK_PROJSPEED: f32 = 2.0; //The speed of this enemy's projectile attack
pub const HEALTH_ENEMY_ATK_WIDTH: u32 = 30; //The width of this enemy's projectile attack
pub const HEALTH_ENEMY_ATK_HEIGHT: u32 = 30; //The height of this enemy's projectile attack

//----Speed Enemy Stats----
pub const SPEED_ENEMY_ATK_DMG: i32 = 1; //The damage of this enemy's projectile attack
pub const SPEED_ENEMY_ATK_PROJSPEED: f32 = 4.0; //The speed of this enemy's projectile attack
pub const SPEED_ENEMY_ATK_WIDTH: u32 = 10; //The width of this enemy's projectile attack
pub const SPEED_ENEMY_ATK_HEIGHT: u32 = 10; //The height of this enemy's projectile attack

//----Attack Enemy Stats----
pub const ATTACK_ENEMY_ATK_DMG: i32 = 3; //The damage of this enemy's projectile attack
pub const ATTACK_ENEMY_ATK_PROJSPEED: f32 = 3.0; //The speed of this enemy's projectile attack
pub const ATTACK_ENEMY_ATK_WIDTH: u32 = 20; //The width of this enemy's projectile attack
pub const ATTACK_ENEMY_ATK_HEIGHT: u32 = 20; //The height of this enemy's projectile attack

// ------------------------------------------- Vec2 --------------------------------------------
#[derive(Copy, Clone, PartialEq, Debug)]
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
}

#[derive(Copy, Clone, PartialEq)]
pub enum Gem {
    Red,
    Blue,
    Yellow,
    None,
}
