use crate::util::*;
use crate::entity::Health;
use crate::boxes::*;
use crate::boxes::Box;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use crate::tile::*;


pub const PLAYER_SPEED: f32 = 300.0;

pub struct Player {
    pub pos: Vec2<f32>, // Position of middle of player.
    pub pos_static: Vec2<f32>,

    // TODO: REWORK INTO INDIVIDUAL TRAITS SO THEY CAN BE USED WITH ENEMIES
    pub box_es: Box,
    pub speed: f32,
    pub stored_speed: f32,
    pub dir: Direction,
    pub hp: i32,    //store the health for player
    pub m_hp: i32,
    pub death: bool, //trying bool for death state
    pub attack: i32, // modifier for attack
    pub last_damage_taken: i32, // amount of hp lost in last damage taken

    pub power_up_vec: Vec<i32>, //[Health, Speed, Attack]

    pub prev_frame_tile: Vec2<i32>,
    pub current_frame_tile: Vec2<i32>,

    pub has_bomb: bool,
    pub using_bomb: bool,
    pub last_bomb_time: Option<Instant>,
    pub has_key: bool,
    pub last_invincibility_time: Option<Instant>,
    pub time_between_frames: Option<Instant>,
    

    pub is_attacking: bool,
    pub is_charging: bool,
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
            pos_static: Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 5 * 64) as f32 + 40.0),
            box_es: Box::new(Vec2::new(48, 52), Vec2::new(40, 24), Vec2::new(48, 64)),
            speed: PLAYER_SPEED,
            stored_speed: PLAYER_SPEED,
            dir: Direction::Down,
            hp: P_MAX_HP,
            m_hp: P_MAX_HP,
            death: false,

            attack: P_DEFAULT_ATK,

            last_damage_taken: 0,


            power_up_vec: vec![0; 3],

            prev_frame_tile: Vec2::new(8, 5),
            current_frame_tile: Vec2::new(8, 5),

            has_bomb: false,
            using_bomb: false,
            last_bomb_time: None,
            has_key: false,
            last_invincibility_time: None,
            time_between_frames: None,

            //timing attacks so they aren't just 'on'
            is_attacking: false,
            is_charging: false,
            last_attack_time: Some(Instant::now()),

            walkover_action: WalkoverAction::DoNothing,

        }
    }

    pub fn update_pos(& mut self, mut mov_vec: Vec2<f32>) {

        // Fix diagonal directions giving more speed than one direction
        if mov_vec.x != 0.0 && mov_vec.y != 0.0 {
            // The number approximates sqrt(2)/2, the position on the unit circle at 45 degrees
            // that is 1 unit away from the center.
            mov_vec.x *= DIAGONAL_VEC;
            mov_vec.y *= DIAGONAL_VEC;
        }

         // Update position using movement vector and speed
        // (now factors in time between frames for frame-independent movement)
        match self.time_between_frames {
            Some( delta ) => {
                let val = delta.elapsed().as_millis() as f32;

                // This check prevents the position from abruptly speeding up during the very short period
                // that the delta isn't updating, as we are transitioning between doors (around a 300-400ms period)
                
                // This also prevents the player from flying immediately into a spike as soon
                // as they enter the room. It adds a natural snappiness to each room transition;
                // still allowing player movement to occur, but delaying it slightly until
                // the room transition is essentially finished.
                if ( val < 300.0 ) {
                    self.pos.x += mov_vec.x * self.speed * ( val / 1000.0 );
                    self.pos.y += mov_vec.y * self.speed * ( val / 1000.0 );
                }
            },
            None => {}
        }

        // Update timestamp to get next delta
        self.time_between_frames = Some( Instant::now() );

        // self.pos.x += mov_vec.x * self.speed;
        // self.pos.y += mov_vec.y * self.speed;

        // COLLISION CODE HAS BEEN MOVED TO MANAGER STRUCT, AS NEW COLLISION REQUIRES KNOWLEDGE OF
        // MAP STATE WHICH IS ABOVE THE PLAYER
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32}
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32}

    pub fn update_invincibility_time(&mut self) {
        self.last_invincibility_time = Some(Instant::now());
    }

    pub fn was_attacked(&mut self) -> bool {
        match self.last_invincibility_time {
            Some( time ) => time.elapsed() <= Duration::from_millis(500),
            None => false
        }
    }

    pub fn update_static_pos(&mut self)
    {
        self.pos_static = self.pos;
    }

    pub fn signal_charge(&mut self) {
        self.is_charging = true;
        self.last_attack_time = Some(Instant::now());
    }

    pub fn recently_charged(&mut self) -> bool {
        match self.last_attack_time {
            Some( time ) => {
                let res = time.elapsed() <= Duration::from_millis(1000);
                if !res {
                    self.is_charging = false;
                }

                res
            },
            None => false
        }
    }

    pub fn signal_attack(&mut self) {
        self.is_attacking = true;
        self.last_attack_time = Some(Instant::now());
    }

    pub fn recently_attacked(&mut self) -> bool {
        match self.last_attack_time {
            Some( time ) => {
                let res = time.elapsed() <= Duration::from_millis(250);
                if !res {
                    self.is_attacking = false;
                }

                res
            },
            None => false
        }
    }
    pub fn recently_bombed(&mut self) -> bool {
        match self.last_bomb_time {
            Some( time ) => {
                let res = time.elapsed() <= Duration::from_millis(500);
                if !res {
                    self.using_bomb = false;
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
                if self.speed >= self.stored_speed{
                    self.stored_speed = self.speed;
                    self.speed *= 0.6666;
                }
            },
            _ => {
                //println!("{}",self.speed);
                if self.speed < self.stored_speed{
                    self.speed = self.stored_speed;
                }
            },
        }
    }

    pub fn take_damage(&mut self, amount: i32, cooldown_window_ms: u64) {
        match self.last_invincibility_time {
            // If there is an old invincibility time for the player,
            // see if the "invincibility window" has elapsed since then...
            Some( time ) => {
                if time.elapsed() >= Duration::from_millis(cooldown_window_ms) {
                    // If so, update the invincibility time and take damage to the player.
                    self.update_invincibility_time();
                    self.damage(amount);
                }
            },
            None => {
                // Otherwise, take damage as there was
                // no previous "invincibility window" to account for
                self.update_invincibility_time();
                self.damage(amount);
            }
        }
    }

    pub fn use_bomb(&mut self) {
        self.update_static_pos();
        self.has_bomb = false;
        self.using_bomb = true;
        self.last_bomb_time = Some(Instant::now());
    }
}

impl Health for Player {
    fn max_hp(&self) -> i32 { return self.m_hp; }
	fn health(&self) -> i32 { return self.hp; }
    fn damage(&mut self, d: i32) -> i32 {
        self.last_damage_taken = d;
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
                *temp = 1;
            }
            if self.hp != self.m_hp {
                self.heal(self.m_hp);
            }
            else{
                self.m_hp += 2;
                self.heal(self.m_hp);
            }
        }
    }
    fn plus_power_speed(&mut self){
        if let Some(temp) = self.power_up_vec.get_mut(1){
            *temp += 1;
        }
        if self.power_up_vec[1] > 3 {
            if let Some(temp) = self.power_up_vec.get_mut(1){
                *temp = 1;
            }
            self.speed += 20.0;
        }
    }
    fn plus_power_attack(&mut self){
        if let Some(temp) = self.power_up_vec.get_mut(2){
            *temp += 1;
        }
        if self.power_up_vec[2] > 3 {
            if let Some(temp) = self.power_up_vec.get_mut(2){
                *temp = 1;
            }
            self.attack += 1;
        }
    }
}
