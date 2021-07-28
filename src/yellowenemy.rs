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
            if(enemy.pos.y < blackboard.playerpos.y){
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else if(enemy.pos.y > blackboard.playerpos.y){
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else{
                enemy.movement_vec.x = -1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
        Direction::Left => {
            if(enemy.pos.y < blackboard.playerpos.y){
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else if(enemy.pos.y > blackboard.playerpos.y){
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
}

pub fn take_cover(enemy: & mut Enemy, blackboard: &BlackBoard){
    
}

pub fn chase(enemy: & mut Enemy, blackboard: &BlackBoard){
    match enemy.dir {
        Direction::Up => {
            enemy.movement_vec.y = -1.0;
        }
        Direction::Down => {
            enemy.movement_vec.y = 1.0;
        }
        Direction::Right => {
            if(enemy.pos.y < blackboard.playerpos.y){
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else if(enemy.pos.y > blackboard.playerpos.y){
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else{
                enemy.movement_vec.x = 1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
        Direction::Left => {
            if(enemy.pos.y < blackboard.playerpos.y){
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else if(enemy.pos.y > blackboard.playerpos.y){
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else{
                enemy.movement_vec.x = -1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
    }
   // println!("{:?}, {:?}", enemy.lastpos, enemy.pos);
/*
    if (!Enemy::player_close(enemy, blackboard) && (enemy.lastpos.x != enemy.pos.x || enemy.lastpos.y != enemy.pos.y))
    {
        /*if !Enemy::player_close(enemy, blackboard){
            println!("1");
            }
            
        if enemy.lastpos.x == enemy.pos.x {
            println!("2");
        }
        
        if enemy.lastpos.y == enemy.pos.y {
            println!("3");
        }*/
        match enemy.dir {
            Direction::Up => {
                enemy.pos.x += 4.0;
            }
            Direction::Down => {
                enemy.pos.x -= 4.0;
            }
            Direction::Right => {
                enemy.pos.y += 4.0;
            }
            Direction::Left => {
                enemy.pos.y -= 4.0;
            }
        }
    }
    else {*/
        enemy.pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.pos.y += enemy.movement_vec.y * enemy.speed;
   // }
    if Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Attack;
    }
}

pub fn heal(enemy: & mut Enemy, blackboard: &BlackBoard){
    
}

pub fn idle(enemy: & mut Enemy, blackboard: &BlackBoard){
    if(blackboard.playerpos.x > 400.0)
    {
        enemy.state = State::Chase;
    }
}




