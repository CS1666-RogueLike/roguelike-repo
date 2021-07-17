use crate::util::*;
use crate::entity::Health;
use crate::player::*;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use crate::tile::*;
use crate::game::*;

pub enum BoxKind {
    Attack,
    Hit,
    Walk,
}

pub struct Box {
    pub hitbox: Vec2<u32>, // Hitbox where player takes damage.
    pub walkbox: Vec2<u32>, // walkbox involved in collision with rooms.
    pub attackbox: Vec2<i32>, //Attack box where player does damage
}

impl Box {
    pub fn new(hit : Vec2<u32>, walk: Vec2<u32>, attack: Vec2<i32>) -> Box {
        Box {
            hitbox: hit,
            walkbox: walk,
            attackbox: attack,
        }
    }
    //need a function getter for each type of box
    pub fn get_walkbox(&self, pos : Vec2<f32>) -> Rect { Rect::new(
                                                    pos.x as i32 - (self.walkbox.x / 2) as i32,
                                                    pos.y as i32 - (self.walkbox.y / 2) as i32,
                                                    self.walkbox.x,
                                                    self.walkbox.y,
                                                    )
    }
    // function to get the hitbox
    pub fn get_hitbox(&self, pos : Vec2<f32>) -> Rect { Rect::new(
                                                    pos.x as i32 - (self.hitbox.x / 2) as i32,
                                                    pos.y as i32 - ((self.hitbox.y / 2) as i32 + (self.walkbox.y / 2) as i32),
                                                    self.hitbox.x,
                                                    self.hitbox.y,
                                                    )
    }
    //function to get the attackbox based on direction
    pub fn get_attackbox(&self, pos : Vec2<f32>, dir : Direction) -> Rect {
        match dir {
            Direction::Up => {
                Rect::new(pos.x as i32 - ( self.attackbox.x / 2 ) as i32, pos.y as i32 - (self.attackbox.y as i32) - (self.attackbox.y / 2 as i32) - 24,
                        self.attackbox.x as u32, self.attackbox.y as u32)
            }
            Direction::Down => {
                Rect::new(pos.x as i32 - ( self.attackbox.x / 2 ) as i32, pos.y as i32 + 16,
                        self.attackbox.x as u32, self.attackbox.y as u32)
            }
            Direction::Left => {
                Rect::new(pos.x as i32 - 48 - self.attackbox.x, pos.y as i32 - 32,
                        self.attackbox.y as u32, self.attackbox.x as u32)
            }
            Direction::Right => {
                Rect::new(pos.x as i32 + self.attackbox.x as i32, pos.y as i32 - 32,
                        self.attackbox.y as u32, self.attackbox.x as u32)
            }
        }
    }
}
