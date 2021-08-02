use crate::blackboard::*;
use crate::attack::*;
use crate::util::*;
//use sdl2::rect::Rect;
//use std::time::{Duration, Instant};
//use crate::boxes::*;
use crate::player::*;
use crate::entity::*;
use crate::game::*;
use rand::Rng;
use std::num::*;


/*#[derive(Clone)]
pub enum State{
    Attack,
    Retreat,
    take_cover,
    Chase,
    Heal,
    Idle,
}*/

pub fn update(enemy: & mut Enemy, blackboard: &BlackBoard){

    match enemy.state {
        State::Attack => {
            attack(enemy, blackboard);
        }
        State::Retreat => {
            retreat(enemy, blackboard);
        }
        State::TakeCover => {
            take_cover(enemy, blackboard);
        }
        State::Chase => {
            chase(enemy, blackboard);
        }
        State::Heal => {
            heal(enemy, blackboard);
        }
        State::Idle => {
            idle(enemy, blackboard);
        }
    }
}

pub fn attack(enemy: & mut Enemy, blackboard: &BlackBoard){


    //println!("Daphne");
    enemy.signal_shot();
    if enemy.is_shooting{
        let mut vector = Vec2::new(blackboard.playerpos.x - enemy.pos.x, blackboard.playerpos.y - enemy.pos.y);
        let length = ((vector.x * vector.x + vector.y * vector.y) as f64).sqrt();
    
        // normalize vector
        vector.x /= length as f32;
        vector.y /= length as f32;
    
        let new_atk = AtkProjectile::new(enemy.pos, vector, &EnemyKind::Attack);
        enemy.atk_list.push(new_atk);
        
        enemy.is_shooting = false;
    }

     if blackboard.playerpos.y >= 250.0
     {
         enemy.state = State::Chase;
     }


//    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
//    (blackboard.enemy_quantity > 1 &&
//    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){
//        enemy.state = State::Retreat;
//    }

    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 {
        enemy.state = State::Heal;
    }


}

pub fn retreat(enemy: & mut Enemy, blackboard: &BlackBoard){
    match enemy.dir {
        Direction::Up => {
            enemy.movement_vec.y = 1.0;
        }
        Direction::Down => {
            enemy.movement_vec.y = -1.0;
        }
        Direction::Right => {
            if enemy.pos.y < blackboard.playerpos.y {
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y {
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else {
                enemy.movement_vec.x = -1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
        Direction::Left => {
            if enemy.pos.y < blackboard.playerpos.y {
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y {
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else{
                enemy.movement_vec.x = 1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
    }

    enemy.pos.x += enemy.movement_vec.x * enemy.speed;
    enemy.pos.y += enemy.movement_vec.y * enemy.speed;

    if(blackboard.enemy_quantity == 1)
    {
        enemy.state = State::Chase;
    }
}

pub fn take_cover(enemy: & mut Enemy, blackboard: &BlackBoard){

}

pub fn chase(enemy: & mut Enemy, blackboard: &BlackBoard){
    enemy.pathfinding(blackboard.playerpos, blackboard);
    match enemy.dir {
        Direction::Up => {
            //println!("Up");
            enemy.movement_vec.y = -1.0;
        }
        Direction::Down => {
            //println!("Down");
            enemy.movement_vec.y = 1.0;
        }
        Direction::Right => {
            if enemy.pos.y < blackboard.playerpos.y - 5.0 {

                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y + 5.0 {
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else {
                enemy.movement_vec.x = 1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
        Direction::Left => {
            if enemy.pos.y < blackboard.playerpos.y - 5.0 {

                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y + 5.0 {
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else {
                enemy.movement_vec.x = -1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
    }

        if enemy.pos.y > 200.01 {
            enemy.movement_vec.y = 0.0;
        }

        if enemy.dir == Direction::Up {
            enemy.movement_vec.y = -1.0;
        }
        enemy.pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.box_left_final_pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.box_right_final_pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.pos.y += enemy.movement_vec.y * enemy.speed;
        enemy.box_left_final_pos.y += enemy.movement_vec.y * enemy.speed;
        enemy.box_right_final_pos.y += enemy.movement_vec.y * enemy.speed;

        let mut rng = rand::thread_rng();
        enemy.signal_attack();
        if enemy.is_attacking {
            let mut kindvec = Vec::new(); //Declared
            let mut enemyKind = EnemyKind::Final; //Declared, never actually final
            match generate_kind(enemy, blackboard){
                Attack => {
                    kindvec = vec![EnemyKind::Attack, EnemyKind::Speed, EnemyKind::Health];
                }
                Speed => {
                    kindvec = vec![EnemyKind::Speed, EnemyKind::Attack, EnemyKind::Health];
                }
                Health => {
                    kindvec = vec![EnemyKind::Health, EnemyKind::Speed, EnemyKind::Attack];
                }
                _=>{println!("This isn't right, in fact its dead wrong");}
            }

            match rng.gen_range( 0 ..= 4 ){
                0 | 1 | 2 => {
                    enemyKind = kindvec[0];
                },
                3 => {
                    enemyKind = kindvec[1];
                },
                4 => {
                    enemyKind = kindvec[2];
                },
                _ => {println!("This isnt right, in fact its.. wrong");}

            }
        
            match rng.gen_range( 0 ..= 4 ){
                0 | 1 => {
                    let mut enemies = Enemy::new(Vec2::new(enemy.box_left_final_pos.x, enemy.box_left_final_pos.y), enemyKind);
                    enemy.add_enemies(enemies);
                },
                2 | 3 => {
                    let mut enemies = Enemy::new(Vec2::new(enemy.box_right_final_pos.x, enemy.box_right_final_pos.y), enemyKind);
                    enemy.add_enemies(enemies);
                },
                _ => {enemy.is_attacking = false}

            }
        }

    if blackboard.playerpos.y < 250.0 {
        enemy.state = State::Attack;
    }


    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0{

        enemy.state = State::Heal;
    }
}

pub fn heal(enemy: & mut Enemy, blackboard: &BlackBoard){
    //let mut heal_close = false;
        
    if !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health) {
        let mut rng = rand::thread_rng();
        enemy.signal_attack();
        if enemy.is_attacking {
            match rng.gen_range( 0 ..= 3 ){
                0 | 1 => {
                    let mut enemies = Enemy::new(Vec2::new(enemy.box_left_final_pos.x, enemy.box_left_final_pos.y), EnemyKind::Health);
                    enemy.add_enemies(enemies);
                },
                2 | 3 => {
                    let mut enemies = Enemy::new(Vec2::new(enemy.box_right_final_pos.x, enemy.box_right_final_pos.y), EnemyKind::Health);
                    enemy.add_enemies(enemies);
                },
                _ => {enemy.is_attacking = false}

            }
        }
    }
    else{
        if enemy.box_es.get_hitbox(enemy.pos).has_intersection(blackboard.health_enemy_hitbox[0])
            && (enemy.hp as f32) < enemy.m_hp as f32 * 0.75 {
            enemy.is_healing = true;
            enemy.take_damage(-1, HEAL_TIME);
            println!("{}", enemy.hp);
 //           if(enemy.hp as f32 > enemy.m_hp as f32*0.5){
//                enemy.state = State::Attack;
//            }
            //println!("{}", enemy.hp);
    
        }
        else{
            enemy.is_healing = false;
        }
    
        if !enemy.is_healing && blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health){
            enemy.update_dir(blackboard.health_enemy_tile[0]);
    
            match enemy.dir {
                Direction::Up => {
                    enemy.movement_vec.y = -1.0;
                }
                Direction::Down => {
                    enemy.movement_vec.y = 1.0;
                }
                Direction::Right => {
                    if(enemy.pos.y < blackboard.health_enemy_pos[0].y-5.0){
                        enemy.movement_vec.x = DIAGONAL_VEC;
                        enemy.movement_vec.y = DIAGONAL_VEC;
                    }
                    else if(enemy.pos.y > blackboard.health_enemy_pos[0].y+5.0){
                        enemy.movement_vec.x = DIAGONAL_VEC;
                        enemy.movement_vec.y = -DIAGONAL_VEC;
                    }
                    else{
                        enemy.movement_vec.x = 1.0;
                        enemy.movement_vec.y = 0.0;
                    }
                }
                Direction::Left => {
                    if(enemy.pos.y < blackboard.health_enemy_pos[0].y-5.0){
                        enemy.movement_vec.x = -DIAGONAL_VEC;
                        enemy.movement_vec.y = DIAGONAL_VEC;
                    }
                    else if(enemy.pos.y > blackboard.health_enemy_pos[0].y+5.0){
                        enemy.movement_vec.x = -DIAGONAL_VEC;
                        enemy.movement_vec.y = -DIAGONAL_VEC;
                    }
                    else{
                        enemy.movement_vec.x = -1.0;
                        enemy.movement_vec.y = 0.0;
                    }
                }
            }
    
            if enemy.pos.y > 200.01 {
                enemy.movement_vec.y = 0.0;
            }

            if enemy.dir == Direction::Up {
                enemy.movement_vec.y = -1.0;
            }
            
            enemy.pos.x += enemy.movement_vec.x * enemy.speed;
            enemy.box_left_final_pos.x += enemy.movement_vec.x * enemy.speed;
            enemy.box_right_final_pos.x += enemy.movement_vec.x * enemy.speed;
            enemy.pos.y += enemy.movement_vec.y * enemy.speed;
            enemy.box_left_final_pos.y += enemy.movement_vec.y * enemy.speed;
            enemy.box_right_final_pos.y += enemy.movement_vec.y * enemy.speed;
        }


        //enemy.update_dir(blackboard.health_enemy_pos.pop());
        
//        if !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health){
//            enemy.is_healing = false;
//            enemy.state = State::Chase;
//        }
        if (enemy.hp as f32) >= enemy.m_hp as f32 * 0.5 {
            enemy.is_healing = false;
            enemy.state = State::Attack;
        }
    }
}

pub fn idle(enemy: & mut Enemy, blackboard: &BlackBoard){
    if blackboard.playerpos.x > 300.0
    {
        enemy.state = State::Attack;
    }
}

pub fn generate_kind(enemy: & mut Enemy, blackboard: &BlackBoard) -> EnemyKind{
    let speed_powerups = ((blackboard.player_speed - PLAYER_SPEED)/20.0) as i32;
    let atk_powerups = blackboard.player_attack - P_DEFAULT_ATK;
    let health_powerups= (blackboard.player_max_health - P_MAX_HP)/2;
    let mut high_kind = EnemyKind::Final;
    
    if speed_powerups > atk_powerups && speed_powerups > health_powerups{
        high_kind = EnemyKind::Speed;
    }
    else if atk_powerups > speed_powerups && atk_powerups > health_powerups{
        high_kind = EnemyKind::Health;
    }
    else if health_powerups > atk_powerups && health_powerups > speed_powerups{
        high_kind = EnemyKind::Attack;
    }
    else if health_powerups == atk_powerups && atk_powerups == speed_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 2 ){
            0=>{
                high_kind = EnemyKind::Speed;  
            }
            1=>{
                high_kind = EnemyKind::Health;
            }
            2=>{
                high_kind = EnemyKind::Attack;
            }
            _ => {}
        }
    }
    else if health_powerups == atk_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 1 ){
            0=>{
                high_kind = EnemyKind::Attack;  
            }
            1=>{
                high_kind = EnemyKind::Health;
            }
            _ => {}
        }
    }
    else if health_powerups == speed_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 1 ){
            0=>{
                high_kind = EnemyKind::Speed;  
            }
            1=>{
                high_kind = EnemyKind::Attack;
            }
            _ => {}
        }
    }
    else if atk_powerups == speed_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 1 ){
            0=>{
                high_kind = EnemyKind::Health;  
            }
            1=>{
                high_kind = EnemyKind::Speed;
            }
            _ => {}
        }
    }
    else{
        println!{"Scrappy Doo, i guess"};
    }
    
    return high_kind;
}






