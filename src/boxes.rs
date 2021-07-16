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

pub trait Box {
    //fn box_type(&self) -> BoxKind; //return the box type being USED
    //fn set_box(&self, x: u32, y: u32) -> Vec2<u32>;
    fn get_box(&self, game: &mut Game) -> Rect;
}

pub struct HitBox {
    pub box_type: BoxKind,
    //pub box_size: Vec2<u32>, // Box parameters
    pub box_rect: Rect, // Box parameters
}

impl HitBox {
    pub fn new() -> HitBox {
        HitBox {
            box_type: BoxKind::Hit,
            box_rect: Rect::new(20, 12, 40, 24),
        }
    }
    pub fn get_box(mut game : &mut Game) -> Rect { Rect::new(
                                                    game.player.pos.x as i32 - 0,
                                                    game.player.pos.y as i32 - 0,
                                                    50,
                                                    50,
                                                    )
    }
}

impl Box for HitBox {
    //fn box_type(&self) -> BoxKind {self.box_type}
    // fn set_box(&self, x: u32, y: u32) -> Vec2 {
    //     Vec2::new(x, y);
    // }
    fn get_box(&self, game : &mut Game) -> Rect { Rect::new(
                                                    game.player.pos.x as i32 - 0,
                                                    game.player.pos.y as i32 - 0,
                                                    50,
                                                    50,
                                                    )
    }

}


// pub struct AttackBox {
//     pub box_size: Vec2<u32>, // Box parameters
// }
// impl Box for AttackBox {
//     fn box_type(&self) -> BoxKind; //return the box type being USED
//     fn set_box(&self, box: BoxKind) -> Vec2;
//     fn get_box(&self) -> Vec2;
// }
//
//
//
// pub struct WalkBox {
//     pub box_size: Vec2<u32>, // Box parameters
// }
// impl Box for WalkBox {
//     fn box_type(&self) -> BoxKind; //return the box type being USED
//     fn set_box(&self, box: BoxKind) -> Vec2;
//     fn get_box(&self) -> Vec2;
// }
