use crate::util::*;
use crate::entity::Health;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use crate::tile::*;

const PLAYER_SPEED: f32 = 3.5;

pub struct Player {
    pub pos: Vec2<f32>, // Position of middle of player.

    // TODO: REWORK INTO INDIVIDUAL TRAITS SO THEY CAN BE USED WITH ENEMIES
    pub hitbox: Vec2<u32>, // Hitbox where player takes damage.
    pub walkbox: Rect, // Hitbox involved in collision with rooms.
    pub attackbox: Vec2<i32>, //Attack box where player does damage

    pub speed: f32,
    pub dir: Direction,
    pub hp: i32,    //store the health for player
    pub m_hp: i32,
    pub death: bool, //trying bool for death state
    pub attack: i32, // modifier for attack

    pub power_up_vec: Vec<i32>, //[Health, Speed, Attack]

    pub prev_frame_tile: Vec2<i32>,
    pub current_frame_tile: Vec2<i32>,

    pub has_key: bool,
    pub last_invincibility_time: Option<Instant>,

    pub is_attacking: bool,
    pub last_attack_time: Option<Instant>,

    pub walkover_action: WalkoverAction,


}

pub trait PowerUp {
    fn plus_power_health(&mut self);
    fn plus_power_speed(&mut self);
    fn plus_power_attack(&mut self);
}



/*
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    self.core.wincan.draw_rect(Rect::new(self.game.player.get_pos_x() - (self.game.player.get_walkbox_x()/2) as i32,
                                                        self.game.player.get_pos_y() - (self.game.player.get_walkbox_y()/2) as i32,
                                                        self.game.player.get_walkbox_x(),
                                                        self.game.player.get_walkbox_y())
                                                        */

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 5 * 64) as f32 + 40.0),
            hitbox: Vec2::new(48, 52),
            walkbox: Rect::new(20, 12, 40, 24),
            attackbox: Vec2::new(32, 48),
            speed: PLAYER_SPEED,
            dir: Direction::Down,
            hp: MAX_HP,
            m_hp: MAX_HP,
            death: false,
            attack: 1,

            power_up_vec: vec![0; 3],

            prev_frame_tile: Vec2::new(8, 5),
            current_frame_tile: Vec2::new(8, 5),

            has_key: false,
            last_invincibility_time: None,

            //timing attacks so they aren't just 'on'
            is_attacking: false,
            last_attack_time: None,

            walkover_action: WalkoverAction::DoNothing,

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

        // COLLISION CODE HAS BEEN MOVED TO MANAGER STRUCT, AS NEW COLLISION REQUIRES KNOWLEDGE OF
        // MAP STATE WHICH IS ABOVE THE PLAYER
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32}
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32}

    pub fn get_walkbox(&self) -> Rect { self.walkbox }
    pub fn get_walkbox_world(&self) -> Rect { Rect::new(
                                                    self.pos.x as i32 - self.walkbox.x(),
                                                    self.pos.y as i32 - self.walkbox.y(),
                                                    self.walkbox.width(),
                                                    self.walkbox.height(),
                                                    )
    }

    pub fn update_invincibility_time(&mut self) {
        self.last_invincibility_time = Some(Instant::now());
    }

    pub fn was_attacked(&mut self) -> bool {
        match self.last_invincibility_time {
            Some( time ) => time.elapsed() <= Duration::from_millis(500),
            None => false
        }
    }

    pub fn get_hitbox_x(&self) -> u32 { self.hitbox.x }
    pub fn get_hitbox_y(&self) -> u32 { self.hitbox.y }

    pub fn get_attackbox_x(&self) -> i32 { self.attackbox.x }
    pub fn get_attackbox_y(&self) -> i32 { self.attackbox.y }

    pub fn get_attackbox_world(&self) -> Rect {
        match self.dir {
            Direction::Up => {
                Rect::new(self.pos.x as i32 - ( self.attackbox.x / 2 ) as i32, self.pos.y as i32 - (self.hitbox.y as i32) - (self.attackbox.y / 2 as i32) - 16,
                        self.attackbox.x as u32, self.attackbox.y as u32)
            }
            Direction::Down => {
                Rect::new(self.pos.x as i32 - ( self.attackbox.x / 2 ) as i32, self.pos.y as i32 + 16,
                        self.attackbox.x as u32, self.attackbox.y as u32)
            }
            Direction::Left => {
                Rect::new(self.pos.x as i32 - 48 - self.attackbox.x, self.pos.y as i32 - 32,
                        self.attackbox.y as u32, self.attackbox.x as u32)
            }
            Direction::Right => {
                Rect::new(self.pos.x as i32 + self.hitbox.x as i32 - 16, self.pos.y as i32 - 32,
                        self.attackbox.y as u32, self.attackbox.x as u32)
            }

        }
    }

    pub fn signal_attack(&mut self) {
        self.is_attacking = true;
        self.last_attack_time = Some(Instant::now());
    }

    pub fn recently_attacked(&mut self) -> bool {
        match self.last_attack_time {
            Some( time ) => {
                let res = time.elapsed() <= Duration::from_millis(500);
                if !res {
                    self.is_attacking = false;
                }

                res
            },
            None => false
        }
    }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }

    pub fn power_image_health(&mut self) -> u32 {
        let mut ret_int = 0;
        if let Some(temp) = self.power_up_vec.get_mut(0) {
            ret_int += *temp as u32;
        }
        return ret_int;
    }
    pub fn power_image_speed(&mut self) -> u32 {
        let mut ret_int = 0;
        if let Some(temp) = self.power_up_vec.get_mut(1) {
            ret_int += *temp as u32;
        }
        return ret_int;
    }
    pub fn power_image_attack(&mut self) -> u32 {
        let mut ret_int = 0;
        if let Some(temp) = self.power_up_vec.get_mut(2) {
            ret_int += *temp as u32;
        }
        return ret_int;
    }

    // function to adjust speed based on the walkover action of tile
    pub fn speed_adjust(&mut self, current_tile: WalkoverAction) {
        match current_tile {
            WalkoverAction::Damage => {
                //println!("{:#?}", current_tile);
                if self.speed >= PLAYER_SPEED{
                    self.speed *= 0.6666;
                }
            },
            _ => {
                //println!("{}",self.speed);
                if self.speed < PLAYER_SPEED{
                    self.speed = PLAYER_SPEED;
                }
            },
        }
    }
}

impl Health for Player {
    fn max_hp(&self) -> i32 { return self.m_hp; }
	fn health(&self) -> i32 { return self.hp; }
    fn damage(&mut self, d: i32) -> i32 {
        self.hp -= d;
        if self.hp <= 0 {
            self.hp = 0;
            }
        self.hp // I changed this and the next one to use rust style implicit returns
    }
    fn heal(&mut self, h: i32) -> i32 {
        self.hp = self.hp + h;
        if self.hp > self.m_hp {
            self.hp = self.m_hp;
        }
        self.hp
    }

    //try to implement a player death
    fn death(&mut self) -> bool {
        if self.hp <= 0 {
            self.death = true;
        }
        self.death
    }

}


impl PowerUp for Player {
    fn plus_power_health(&mut self){
        if let Some(temp) = self.power_up_vec.get_mut(0){
            *temp += 1;
        }
        if self.power_up_vec[0] > 3 {
            if let Some(temp) = self.power_up_vec.get_mut(0){
                *temp = 0;
            }
            if self.hp != self.m_hp {
                self.heal(1);
            }
            else{
                self.m_hp += 1;
            }
        }
    }
    fn plus_power_speed(&mut self){
        if let Some(temp) = self.power_up_vec.get_mut(1){
            *temp += 1;
        }
        if self.power_up_vec[1] > 3 {
            if let Some(temp) = self.power_up_vec.get_mut(1){
                *temp = 0;
            }
            self.speed += 1 as f32;
        }
    }
    fn plus_power_attack(&mut self){
        if let Some(temp) = self.power_up_vec.get_mut(2){
            *temp += 1;
        }
        if self.power_up_vec[2] > 3 {
            if let Some(temp) = self.power_up_vec.get_mut(2){
                *temp = 0;
            }
            self.attack += 1;
        }
    }
}
