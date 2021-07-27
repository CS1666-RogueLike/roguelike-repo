use crate::blackboard::*;
use crate::attack::*;
use crate::util::*;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use crate::boxes::*;
use crate::entity::*;


/*#[derive(Clone)]
pub enum State{
    Attack,
    Retreat,
    TakeCover,
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
            takeCover(enemy, blackboard);
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
    enemy.pos.x += 1.0;
    enemy.signal_attack();
    /*if(blackboard.playerpos.x < 400.0)
    {
        enemy.state = State::Idle;
        //println!("{:?}", blackboard.typesInRoom);
        //println!("{}", blackboard.enemyQuantity);
        //println!("{}", blackboard.playerHealth);
        //println!("{:?}", blackboard.playerpos);
        //enemy.signal_attack();
        //let enemy_attack = enemy.box_es.get_attackbox(enemy.pos, enemy.dir);
    }*/
}

pub fn retreat(enemy: & mut Enemy, blackboard: &BlackBoard){
    
}

pub fn takeCover(enemy: & mut Enemy, blackboard: &BlackBoard){
    
}

pub fn chase(enemy: & mut Enemy, blackboard: &BlackBoard){
    
}

pub fn heal(enemy: & mut Enemy, blackboard: &BlackBoard){
    
}

pub fn idle(enemy: & mut Enemy, blackboard: &BlackBoard){
    if(blackboard.playerpos.x > 400.0)
    {
        enemy.state = State::Attack;
    }
}


