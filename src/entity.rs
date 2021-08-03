
use crate::attack::*;
use crate::util::*;
use std::time::{Duration, Instant};
use crate::boxes::*;
use crate::yellowenemy::*;
use crate::finalenemy::*;
use crate::blackboard::*;
use crate::tile::*;
use std::collections::VecDeque;
use crate::room::ROOM_HEIGHT;
use crate::room::ROOM_WIDTH;
use crate::room::*;

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

#[derive(Debug)]
#[derive (Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum EnemyKind {
    Attack,
    Health,
    Speed,
    Final
}

#[derive(Clone)]
pub enum State{
    Attack,
    Retreat,
    TakeCover,
    Chase,
    Heal,
    Idle,
}


#[derive(Clone)]
pub struct Enemy {
    pub pos: Vec2<f32>,
    pub lastpos:Vec2<f32>,
    pub box_es: Box,
    pub speed: f32,
    pub dir: Direction,
    pub hp: i32,    //store the health for speed enemy
    pub m_hp: i32,
    pub movement_vec: Vec2<f32>,
    pub last_dir_update: Option< Instant >,
    pub kind: EnemyKind,
    pub death: bool,
    pub power: bool,
    pub atk_list: Vec<AtkProjectile>,
    pub last_invincibility_time: Option<Instant>,
    pub state: State,
    pub is_attacking: bool,
    pub last_attack_time: Option<Instant>,
    pub current_frame_tile: Vec2<i32>,
    pub is_healing: bool,
    pub last_damage_taken: i32,
    pub is_ranged: bool,

    pub time_scale: f32,

    //BOSS ONLY
    pub box_left_final: Box,
    pub box_left_final_pos: Vec2<f32>,
    pub box_right_final: Box,
    pub box_right_final_pos: Vec2<f32>,
    pub final_enemies_to_spawn: Vec<Enemy>,
    pub last_shot_time: Option<Instant>,
    pub is_shooting: bool,
    pub g_kind: EnemyKind,
    pub state_timer: Instant,


}

impl Health for Enemy {
    fn max_hp(&self) -> i32 { self.m_hp }
    fn health(&self) -> i32 { self.hp }
    fn damage(&mut self, d: i32) -> i32 {
        self.last_damage_taken = d;
        self.hp = (self.hp - d).max(P_DEATH_HP);
        self.death();
        self.hp
    }

    fn heal(&mut self, h: i32) -> i32 {
        self.hp = (self.hp + h).min(self.m_hp);
        self.hp
    }

    fn death(&mut self) -> bool {
        if self.hp <= P_DEATH_HP {
            self.death = true;
        }
        self.death
    }
}

impl Enemy {
    pub fn new(position: Vec2<f32>, kind: EnemyKind) -> Enemy {
        Enemy {
            pos: position,
            lastpos: Vec2::new(-1.0, 0.0),
            //box_es: Box::new(Vec2::new(40, 30), Vec2::new(40, 40), Vec2::new(40, 30)),
            box_es: box_kind(kind),
            speed: speed_kind(kind),
            dir: Direction::Right,
            hp: health_kind(kind),
            m_hp: health_kind(kind),
            movement_vec: Vec2::new(-1.0, 0.0),
            last_dir_update: None,
            kind: kind,
            death: false,
            power: false,
            atk_list: Vec::new(),
            state: State::Idle,
            last_damage_taken: 0,
            is_ranged: set_ranged(),

            current_frame_tile: Vec2::new(0,0),
            last_invincibility_time: None,

            //timing attacks so they aren't just 'on'
            is_attacking: false,
            last_attack_time: None,

            is_healing: false,

            time_scale: 1.0,

            //FINAL BOSS ONLY
            box_left_final: Box::new(Vec2::new(30, 50), Vec2::new(0, 0), Vec2::new(0, 0)),
            box_left_final_pos: Vec2::new(position.x - 30.0, position.y + 5.0),
            box_right_final: Box::new(Vec2::new(30, 50), Vec2::new(0, 0), Vec2::new(0, 0)),
            box_right_final_pos: Vec2::new(position.x + 30.0, position.y + 5.0),
            final_enemies_to_spawn: Vec::<Enemy>::new(),
            last_shot_time: None,
            is_shooting: false,
            g_kind: EnemyKind::Attack,
            state_timer: Instant::now(),

        }
    }

    pub fn update(& mut self, blackboard: &BlackBoard) {
        self.current_frame_tile = Vec2::new(
            (self.get_pos_x() - LEFT_WALL) / TILE_WIDTH,
            (self.get_pos_y() - TOP_WALL) / TILE_WIDTH
        );
        //self.update_dir(blackboard.player_frame_tile);
        //println!("{:?}", self.current_frame_tile);
        match self.kind {
            EnemyKind::Health => {
                crate::redenemy::update(self, blackboard)
            }
            EnemyKind::Speed => {
                crate::blueenemy::update(self, blackboard)
            }
            EnemyKind::Attack => {
                crate::yellowenemy::update(self, blackboard);
            }
            EnemyKind::Final => {
                crate::finalenemy::update(self, blackboard);
            }
        }
    }


    //BOSS ONLY
    pub fn add_enemies(&mut self, enemy: Enemy){
        self.final_enemies_to_spawn.push(enemy);
  }

    pub fn attack_damage(&self) -> i32 {
        use EnemyKind::*;
        match self.kind {
            Health => { 1 }
            Speed  => { 1 }
            Attack => { 2 }
            Final =>  { 1 }
        }
  }

    pub fn switch_heal(&mut self) {
        if self.is_healing {
            self.is_healing = false;
        } else {
            self.is_healing = true;
        }
    }

    pub fn pathfinding(&mut self, target: Vec2<f32>, blackboard: &BlackBoard){

        let target_tile = Vec2::new(
            (target.x as i32 - LEFT_WALL) / TILE_WIDTH,
            (target.y as i32 - TOP_WALL) / TILE_WIDTH
        ); //The target tile

        let none = Vec2::new(-1, -1);

        let start_tile = self.current_frame_tile;
        let mut cur_tile = start_tile; //The current tile

        // Don't do additional code if already in target tile
        if start_tile == target_tile { return; }


        let mut queue: VecDeque<Vec2<i32>> = VecDeque::new(); //The queue of tiles to be checked
        //let mut visited:Vec<Vec2<i32>> = Vec::new(); //Tiles that have been visited
        let mut seen:Vec<Vec2<i32>> = Vec::new(); //Tiles that have been seen
        let mut parent_array:Vec<Vec<Vec2<i32>>> = Vec::new(); //Parent array ()
        let mut neighbors:Vec<Vec2<i32>> = Vec::new();
        neighbors.resize(4,none);
        parent_array.resize(ROOM_WIDTH as usize, Vec::new());


        for i in 0..ROOM_WIDTH{
            parent_array[i as usize].resize(ROOM_HEIGHT as usize, none);
        }
        //visited.push(cur_tile);
        queue.push_back(cur_tile);
        seen.push(cur_tile);


//        let right_tile = Vec2::new(cur_tile.x+1, cur_tile.y);
//        let left_tile = Vec2::new(cur_tile.x-1, cur_tile.y);
//        let up_tile = Vec2::new(cur_tile.x, cur_tile.y-1);
//        let down_tile = Vec2::new(cur_tile.x, cur_tile.y+1);
//
//
//        neighbors.push(right_tile); //neighbors[0] = right_tile
//        neighbors.push(left_tile); //neighbors[1] = left_tile
//        neighbors.push(up_tile); //neighbors[2] = up_tile
//        neighbors.push(down_tile); //neighbors[3] = down_tile
        /*
        for tile in neighbors.iter() {
            let real_tile = *tile;

            parent_array[real_tile.x as usize][real_tile.y as usize] = cur_tile;

            seen.push(real_tile);
            queue.push_back(real_tile);
        }
        */
        while !queue.is_empty() {
            cur_tile = queue.pop_front().unwrap();
            if cur_tile == target_tile {
                break;
            }
            //visited.push(cur_tile);

            neighbors[0] = Vec2::new(cur_tile.x+1, cur_tile.y); //add right neighbor
            neighbors[1] = Vec2::new(cur_tile.x-1, cur_tile.y); //add left neighbor
            neighbors[2] = Vec2::new(cur_tile.x, cur_tile.y-1); //add up neighbor
            neighbors[3] = Vec2::new(cur_tile.x, cur_tile.y+1); //add down neighbor

            for tile in neighbors.iter() {
                let real_tile = *tile;

                if(real_tile.x >= 0 && real_tile.x < ROOM_WIDTH) &&  //The tile x is within the room width
                (real_tile.y >= 0 && real_tile.y < ROOM_HEIGHT) &&  //The tile y is within the room height
                blackboard.is_walkable(real_tile) &&
                !seen.iter().any(|&i| i==real_tile) //The tile has not been seen yet
                {
                    parent_array[real_tile.x as usize][real_tile.y as usize] = cur_tile;
                    seen.push(real_tile);
                    queue.push_back(real_tile);
                }
            }
        }

        let mut path:Vec<Vec2<i32>> = Vec::new();
        while(cur_tile!=start_tile){
            path.push(cur_tile);
            cur_tile = parent_array[cur_tile.x as usize][cur_tile.y as usize];
        }

        //path.push(start_tile);
        self.update_dir(path.pop().unwrap());
    }

    pub fn update_invincibility_time(&mut self) {
        self.last_invincibility_time = Some(Instant::now());
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


    pub fn was_damaged(&self) -> bool {
        match self.last_invincibility_time {
            Some( time ) => time.elapsed() < Duration::from_secs( 1 ),
            None => false
        }
    }


    //Old update direction without pathfinding
    pub fn update_dir(& mut self, frame_tile: Vec2<i32>){
        let e_x = self.current_frame_tile.x;
        let e_y = self.current_frame_tile.y;

        let p_x = frame_tile.x;
        let p_y = frame_tile.y;
        if e_x == p_x && e_y < p_y
        {
            self.dir = Direction::Down;
        }

        if e_x == p_x && e_y > p_y

        {
            self.dir = Direction::Up;
        }

        if e_x > p_x
        {
            self.dir = Direction::Left;
        }

        if e_x < p_x
        {
            self.dir = Direction::Right;
        }
    }

    pub fn distance_to_player(enemy: & mut Enemy, blackboard: &BlackBoard) -> f64 {
        let mut vector = Vec2::new(blackboard.playerpos.x - enemy.pos.x, blackboard.playerpos.y - enemy.pos.y);
        let length = ((vector.x * vector.x + vector.y * vector.y) as f64).sqrt();

        return length;
    }

    pub fn player_close(enemy: & mut Enemy, blackboard: &BlackBoard) -> bool{
        if enemy.is_ranged  && Enemy::distance_to_player(enemy, blackboard) < 300.0{
                return true;
        } else {
            if enemy.box_es.get_walkbox(enemy.pos).has_intersection(blackboard.player_box.get_walkbox(blackboard.playerpos)) {

                return true;
            }
            else{
                return false;
            }
        }
    }
    // Using Connor's player implementation for this design:

    pub fn type_eq(a: EnemyKind, b: EnemyKind) -> bool{
        let num1 = Enemy::assign_num(a);
        let num2 = Enemy::assign_num(b);
        //println!("{:?}, {}", a, num1);
        //println!("{:?}, {}", b, num2);
        num1 == num2
        // if num1 == num2 {
        //     return true;
        // }
        // else {
        //     return false;
        // }
    }

    pub fn assign_num(a: EnemyKind) -> i32 {
        match a {
            EnemyKind::Health => {
                return 0;
            }
            EnemyKind::Speed => {
                return 1;
            }
            EnemyKind::Attack => {
                return 2;
            }
            EnemyKind::Final => {
                return 3;
            }
        }
    }

    pub fn signal_attack(&mut self) {
        //let res = time.elapsed() <= Duration::from_millis(500+600);
        match self.last_attack_time {
            Some (time) => {
                match self.kind {
                    EnemyKind::Final => {
                        let res = time.elapsed() <= Duration::from_millis(4000);
                        if !res {
                            self.is_attacking = true;
                            self.last_attack_time = Some(Instant::now());
                        }
                        else {
                            self.is_attacking = false;
                        }
                    }
                    _ => {
                        let res = time.elapsed() <= Duration::from_millis(500+600);
                        if !res {
                            self.is_attacking = true;
                            self.last_attack_time = Some(Instant::now());
                        }
                        else {
                            self.is_attacking = false;
                        }
                    }
                }
                //let res = time.elapsed() <= Duration::from_millis(500+600);

            }

            None => {
                self.is_attacking = true;
                self.last_attack_time = Some(Instant::now());
            }
        }
    }

    pub fn signal_shot(&mut self) {
        //let res = time.elapsed() <= Duration::from_millis(500+600);
        match self.last_shot_time {
            Some (time) => {
                    let res = time.elapsed() <= Duration::from_millis(750); //Time in between shots
                    if !res {
                        self.is_shooting = true;
                        self.last_shot_time = Some(Instant::now());
                        }
                        else {
                            self.is_shooting = false;
                        }
                    }

                //let res = time.elapsed() <= Duration::from_millis(500+600);
            None => {
                self.is_shooting = true;
                self.last_shot_time = Some(Instant::now());
            }
        }

    }

    pub fn recently_attacked(&mut self) -> bool {
        match self.last_attack_time {
            Some( time ) => {
                let res = time.elapsed() <= Duration::from_millis(500);
                if !res {
                    self.is_shooting = false;
                }

                res
            },
            None => false
        }
    }

    pub fn get_pos_x(&self) -> i32 { self.pos.x as i32 }
    pub fn get_pos_y(&self) -> i32 { self.pos.y as i32 }

    pub fn update_pos(& mut self) {

        println!("UPDATE POS CALLED");

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

                    //Make a new attack projectile every time the enemy moves. For test things
                    let new_atk = AtkProjectile::new(self.pos, self.movement_vec, &self.kind);
                    self.atk_list.push(new_atk);

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
                            self.movement_vec.x = DIAGONAL_VEC;
                            self.movement_vec.y = DIAGONAL_VEC;
                        },
                        9 | 10 => {
                            self.movement_vec.x = -DIAGONAL_VEC;
                            self.movement_vec.y = -DIAGONAL_VEC;
                        },
                        11 | 12 => {
                            self.movement_vec.x = DIAGONAL_VEC;
                            self.movement_vec.y = -DIAGONAL_VEC;
                        },
                        13 | 14 => {
                            self.movement_vec.x = -DIAGONAL_VEC;
                            self.movement_vec.y = DIAGONAL_VEC;
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
        // TODO
        self.pos.x += self.movement_vec.x * self.speed;
        self.pos.y += self.movement_vec.y * self.speed;
        //println!("Speed update!");

        //Moves all the attacks that this enemy shot

        let mut index = 0;
        let mut to_remove = Vec::new();
        for mut atk in &mut self.atk_list {
            atk.pos.x += atk.movement_vec.x * atk.speed;
            atk.pos.y += atk.movement_vec.y * atk.speed;

            //If the attack is off screen, remove it from the atk vector

            if atk.pos.x < 0.0 || atk.pos.y < 0.0 || atk.pos.x > WINDOW_WIDTH as f32|| atk.pos.y > WINDOW_HEIGHT as f32
            {
                to_remove.push(index);
            }
            index+=1;
        }

        for rmv in &mut to_remove {
            self.atk_list.remove(*rmv);
        }
    }

    pub fn move_projectile(&mut self){
        //Moves all the attacks that this enemy shot

        let mut index = 0;
        let mut to_remove = Vec::new();
        for mut atk in &mut self.atk_list {
            atk.pos.x += atk.movement_vec.x * atk.speed * self.time_scale;
            atk.pos.y += atk.movement_vec.y * atk.speed * self.time_scale;

            //If the attack is off screen, remove it from the atk vector

            //if !BlackBoard::is_walkable(current_tile)
            if atk.pos.x < 0.0 || atk.pos.y < 0.0 || atk.pos.x > WINDOW_WIDTH as f32|| atk.pos.y > WINDOW_HEIGHT as f32
            {
                to_remove.push(index);
            }
            index+=1;
        }

        for rmv in &mut to_remove {
            self.atk_list.remove(*rmv);
            //println!("Bullet Scooby Removed");
        }
    }

    pub fn set_dir(& mut self, new_dir: Direction) { self.dir = new_dir; }
    pub fn get_dir(& mut self) -> Direction { self.dir }
}

pub fn speed_kind(kind: EnemyKind) -> f32 {
    let mut speed = 0.0;
    match kind {
        EnemyKind::Health => {
            speed = 1.8;
        }
        EnemyKind::Speed =>{
            speed = 3.8;
        }
        EnemyKind::Attack => {
            speed = 2.8;
        }
        EnemyKind::Final => {
            speed = 0.75;
        }
    }
    return speed;
}
pub fn health_kind(kind: EnemyKind) -> i32 {
    let mut health = 0;
    match kind {
        EnemyKind::Health => {
            health = 5;
        }
        EnemyKind::Speed =>{
            health = 2;
        }
        EnemyKind::Attack => {
            health = 3;
        }
        EnemyKind::Final => {
            health = 10;
        }

    }
    return health;
}

pub fn set_ranged() -> bool {
    let mut rng = rand::thread_rng();

    match rng.gen_range( 0 ..= 6 ){
               0 | 1 | 2 | 4 => {
                   return false;
               },
               _ => {return true;}
           }
}

pub fn box_kind(kind: EnemyKind) -> Box {
    match kind {
        EnemyKind::Health => {
            return Box::new(Vec2::new(40, 30), Vec2::new(40, 40), Vec2::new(40, 30));
        }
        EnemyKind::Speed =>{
            return Box::new(Vec2::new(40, 30), Vec2::new(40, 40), Vec2::new(40, 30));
        }
        EnemyKind::Attack => {
            return Box::new(Vec2::new(40, 30), Vec2::new(40, 40), Vec2::new(40, 30));
        }
        EnemyKind::Final => {
            // Final doesn't attack itself so no attackbox
            return Box::new(Vec2::new(40, 30), Vec2::new(40, 40), Vec2::new(0, 0));
        }
    }
}
