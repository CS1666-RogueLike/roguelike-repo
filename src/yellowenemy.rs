use crate::blackboard::*;
//use crate::attack::*;
use crate::util::*;
//use sdl2::rect::Rect;
//use std::time::{Duration, Instant};
//use crate::boxes::*;
use crate::entity::*;


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
    enemy.signal_attack();

    if !Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Chase;
    }

    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){
        enemy.state = State::Retreat;
    }

    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){
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
            if enemy.pos.y < blackboard.playerpos.y {

                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y {
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else {
                enemy.movement_vec.x = 1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
        Direction::Left => {
            if enemy.pos.y < blackboard.playerpos.y {

                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y {
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else {
                enemy.movement_vec.x = -1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
    }
        enemy.pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.pos.y += enemy.movement_vec.y * enemy.speed;

    if Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Attack;
    }

    //println!("{} vs {}", enemy.hp as f32, (enemy.m_hp as f32 / 3.0));
    //println!("{}", blackboard.enemy_quantity);

    /*if(!blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){
        println!("true");
    }*/

    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){ //True if there isn't a health enemy
        enemy.state = State::Retreat;
    }

    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){ //True if there is a health enemy
        enemy.state = State::Heal;
    }
}

pub fn heal(enemy: & mut Enemy, blackboard: &BlackBoard){
    //let mut heal_close = false;


    if enemy.box_es.get_hitbox(enemy.pos).has_intersection(blackboard.health_enemy_hitbox[0])
        && (enemy.hp as f32) < enemy.m_hp as f32 * 0.75 {
        enemy.is_healing = true;
        enemy.take_damage(-1, HEAL_TIME);
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
                if(enemy.pos.y < blackboard.health_enemy_pos[0].y){
                    enemy.movement_vec.x = DIAGONAL_VEC;
                    enemy.movement_vec.y = DIAGONAL_VEC;
                }
                else if(enemy.pos.y > blackboard.health_enemy_pos[0].y){
                    enemy.movement_vec.x = DIAGONAL_VEC;
                    enemy.movement_vec.y = -DIAGONAL_VEC;
                }
                else{
                    enemy.movement_vec.x = 1.0;
                    enemy.movement_vec.y = 0.0;
                }
            }
            Direction::Left => {
                if(enemy.pos.y < blackboard.health_enemy_pos[0].y){
                    enemy.movement_vec.x = -DIAGONAL_VEC;
                    enemy.movement_vec.y = DIAGONAL_VEC;
                }
                else if(enemy.pos.y > blackboard.health_enemy_pos[0].y){
                    enemy.movement_vec.x = -DIAGONAL_VEC;
                    enemy.movement_vec.y = -DIAGONAL_VEC;
                }
                else{
                    enemy.movement_vec.x = -1.0;
                    enemy.movement_vec.y = 0.0;
                }
            }
        }

        enemy.pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.pos.y += enemy.movement_vec.y * enemy.speed;
    

    }

    //enemy.update_dir(blackboard.health_enemy_pos.pop());

    if !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health){
        enemy.is_healing = false;
        enemy.state = State::Chase;
    }

    if (enemy.hp as f32) >= enemy.m_hp as f32 * 0.75 {
        enemy.is_healing = false;
        enemy.state = State::Chase;
    }
}

pub fn idle(enemy: & mut Enemy, blackboard: &BlackBoard){
    if blackboard.playerpos.x > 400.0
    {
        enemy.state = State::Chase;
    }
}
