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

    // if player is far, chase them
    if !Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Chase;
    }

    // if there are other non health enemies in room and low on life, retreat
    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){
        enemy.state = State::Retreat;
    }

    // low on life and health enemy avail to heal
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
            if enemy.pos.y < blackboard.playerpos.y - 5.0 {
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y + 5.0 {
                enemy.movement_vec.x = -DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else {
                enemy.movement_vec.x = -1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
        Direction::Left => {
            if enemy.pos.y < blackboard.playerpos.y - 5.0{
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = -DIAGONAL_VEC;
            }
            else if enemy.pos.y > blackboard.playerpos.y + 5.0{
                enemy.movement_vec.x = DIAGONAL_VEC;
                enemy.movement_vec.y = DIAGONAL_VEC;
            }
            else{
                enemy.movement_vec.x = 1.0;
                enemy.movement_vec.y = 0.0;
            }
        }
    }

    // update movement depending on direction
    enemy.pos.x += enemy.movement_vec.x * enemy.speed;
    enemy.pos.y += enemy.movement_vec.y * enemy.speed;

    // if last enemy in room, switch to chase
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

    //println!("{}, {}", enemy.movement_vec.x, enemy.movement_vec.y);

        enemy.pos.x += enemy.movement_vec.x * enemy.speed;
        enemy.pos.y += enemy.movement_vec.y * enemy.speed;

    // if close to player, switch to attack
    if Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Attack;
    }

    // if low on life, and no health enemy, retreat
    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){ //True if there isn't a health enemy
        enemy.state = State::Retreat;
    }

    // if low on life, and health enemy avail, go heal
    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health)){ //True if there is a health enemy
        enemy.state = State::Heal;
    }
}

pub fn heal(enemy: & mut Enemy, blackboard: &BlackBoard){


    // if close to health enemy, start to heal

    //let mut heal_close = false;

    if !blackboard.health_enemy_hitbox.is_empty() && enemy.box_es.get_hitbox(enemy.pos).has_intersection(blackboard.health_enemy_hitbox[0])
        && (enemy.hp as f32) < enemy.m_hp as f32 * 0.75 {
        enemy.is_healing = true;
        enemy.take_damage(-1, HEAL_TIME);
        //println!("{}", enemy.hp);

    }
    else{
        enemy.is_healing = false; // if not close to heal enemy, change bool
    }

    // when not close to a heal enemy, but need to heal, find one and move to them
    if !enemy.is_healing && blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health){
        //enemy.update_dir(blackboard.health_enemy_tile[0]);
        enemy.pathfinding(blackboard.health_enemy_pos[0], blackboard);

        match enemy.dir {
            Direction::Up => {
                enemy.movement_vec.y = -1.0;
            }
            Direction::Down => {
                enemy.movement_vec.y = 1.0;
            }
            Direction::Right => {
                if(enemy.pos.y < blackboard.health_enemy_pos[0].y - 5.0){
                    enemy.movement_vec.x = DIAGONAL_VEC;
                    enemy.movement_vec.y = DIAGONAL_VEC;
                }
                else if(enemy.pos.y > blackboard.health_enemy_pos[0].y + 5.0){
                    enemy.movement_vec.x = DIAGONAL_VEC;
                    enemy.movement_vec.y = -DIAGONAL_VEC;
                }
                else{
                    enemy.movement_vec.x = 1.0;
                    enemy.movement_vec.y = 0.0;
                }
            }
            Direction::Left => {
                if(enemy.pos.y < blackboard.health_enemy_pos[0].y - 5.0){
                    enemy.movement_vec.x = -DIAGONAL_VEC;
                    enemy.movement_vec.y = DIAGONAL_VEC;
                }
                else if(enemy.pos.y > blackboard.health_enemy_pos[0].y + 5.0){
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

    // if no health enemies left, move to chase
    if !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health){
        enemy.is_healing = false;
        enemy.state = State::Chase;
    }

    // if healed move to chase
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
