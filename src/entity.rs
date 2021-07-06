
use crate::util::*;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use rand::Rng;

pub trait Health {
    // TODO: Add iframe setup so player can't be damaged for a bit after being damaged.
    // Otherwise a spike tile doing half a heart of damage would kill a 3 heart player in 6 frames.

    fn max_hp(&self) -> i32;  //the maximum HP the entity can have
    fn health(&self) -> i32;    // the current HP the entity has
    fn damage(&mut self, d: i32) -> i32;  // applying the amount of damage received
    //fn damage_type(&self) -> i32; // the damage category
    //fn bonus_type(&self) -> i32;    // the type of bonus dropped by enemy
    //fn percent_damaged(&self) -> f32;
    fn heal(&mut self, h: i32) -> i32;
    fn death(&mut self) -> bool;
}

pub enum EnemyKind {
    Attack,
    Health,
    Speed
}

pub struct Enemy {
    pub pos: Vec2<f32>,
    pub hitbox: Vec2<u32>, // Hitbox where speed enemy takes damage.
    pub walkbox: Rect, // Hitbox involved in collision with rooms.
    pub speed: f32,
    pub dir: Direction,
    pub hp: i32,    //store the health for speed enemy
    pub m_hp: i32,
    pub movement_vec: Vec2<f32>,
    pub cr: Vec2<i32>,
    pub cf: usize,
    pub last_dir_update: Option< Instant >,
    pub kind: EnemyKind,
    pub death: bool,
    pub power: bool,
}

impl Health for Enemy {
    fn max_hp(&self) -> i32 { self.m_hp }
    fn health(&self) -> i32 { self.hp }
    fn damage(&mut self, d: i32) -> i32 {
        self.hp = (self.hp - d).max(DEATH_HP);
        self.death();
        self.hp
    }

    fn heal(&mut self, h: i32) -> i32 {
        self.hp = (self.hp + h).min(self.m_hp);
        self.hp
    }

    fn death(&mut self) -> bool {
        if self.hp <= DEATH_HP {
            self.death = true;
            self.power = true;
        }
        self.death
    }
}

impl Enemy {
    pub fn new(position: Vec2<f32>, kind: EnemyKind, cr: Vec2<i32>, cf: usize) -> Enemy {
        Enemy {
            pos: position,
            hitbox: Vec2::new(40, 30),
            walkbox: Rect::new(20, 32, 40, 40),
            speed: 2.8,
            dir: Direction::Left,
            hp: 2,
            m_hp: 2,
            cr: cr,
            cf: cf,
            movement_vec: Vec2::new(-1.0, 0.0),
            last_dir_update: None,
            kind: kind,
            death: false,
            power: false,
        }
    }

    // Using Connor's player implementation for this design:

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32 }
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32 }

    pub fn get_walkbox(&self) -> Rect { self.walkbox }
    pub fn get_walkbox_world(&self) -> Rect {
        Rect::new(
            self.pos.x as i32 - self.walkbox.x(),
            self.pos.y as i32 - self.walkbox.y(),
            self.walkbox.width(),
            self.walkbox.height(),
        )
    }

    pub fn update_pos(& mut self) {
        if self.death {
            self.movement_vec.x = 0.0;
            self.movement_vec.y = 0.0;
            return;
        }
        let now = Instant::now();

        let mut rng = rand::thread_rng();

        match self.last_dir_update {
            Some(update_time) => {
                if update_time.elapsed() >= Duration::from_secs(2) {
                    match rng.gen_range( 0 ..= 15 ) {
                        0 => {
                            self.movement_vec.x = 0.0;
                            self.movement_vec.y = -1.0;
                        },
                        1 | 2 => {
                            self.movement_vec.x = 0.0;
                            self.movement_vec.y = 1.0;
                        },
                        3 | 4 => {
                            self.movement_vec.x = -1.0;
                            self.movement_vec.y = 0.0;
                        },
                        5 | 6 => {
                            self.movement_vec.x = 1.0;
                            self.movement_vec.y = 0.0;
                        },
                        7 | 8 => {
                            self.movement_vec.x = 0.7071067;
                            self.movement_vec.y = 0.7071067;
                        },
                        9 | 10 => {
                            self.movement_vec.x = -0.7071067;
                            self.movement_vec.y = -0.7071067;
                        },
                        11 | 12 => {
                            self.movement_vec.x = 0.7071067;
                            self.movement_vec.y = -0.7071067;
                        },
                        13 | 14 => {
                            self.movement_vec.x = -0.7071067;
                            self.movement_vec.y = 0.7071067;
                        },
                        15 => {
                            self.movement_vec.x = 0.0;
                            self.movement_vec.y = 0.0;
                        }
                        _ => {}
                    }
                    //self.movement_vec.x = -self.movement_vec.x;
                    //self.movement_vec.y = rng.gen_range(-1.0 ..= 1.0);
                    self.last_dir_update = Some(now);
                }
            },
            None => {
                self.last_dir_update = Some(now);
            }
        }

        // Update position using movement vector and speed
        self.pos.x += self.movement_vec.x * self.speed;
        self.pos.y += self.movement_vec.y * self.speed;
    }

    pub fn get_hitbox_x(&self) -> u32 { self.hitbox.x }
    pub fn get_hitbox_y(&self) -> u32 { self.hitbox.y }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }
}
