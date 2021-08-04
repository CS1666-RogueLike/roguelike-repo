use crate::blackboard::*;
//use crate::attack::*;
use crate::util::*;
//use sdl2::rect::Rect;
//use std::time::{Duration, Instant};
//use crate::boxes::*;
use crate::entity::*;
use crate::attack::*;

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

    if enemy.is_ranged{
        enemy.signal_shot();
        if enemy.is_shooting{
            let mut vector = Vec2::new(blackboard.playerpos.x - enemy.pos.x, blackboard.playerpos.y - enemy.pos.y);
            let length = ((vector.x * vector.x + vector.y * vector.y) as f64).sqrt();

            // normalize vector
            vector.x /= length as f32;
            vector.y /= length as f32;
            let new_atk = AtkProjectile::new(enemy.pos, vector, &enemy.kind);
            enemy.atk_list.push(new_atk);

            enemy.is_shooting = false;
            }

            if enemy.state_timer.elapsed().as_millis() % 6000 <= 2000 {
            enemy.state = State::Attack;
        }
    } else {
        enemy.signal_attack();
    }


    // if player is far, chase them
    if !Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Chase;
    }

    // if there are other non health enemies in room and low on life, retreat
    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health))&&
    !blackboard.boss_fight{
        enemy.state = State::Retreat;
    }

    // we don't need a heal state


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
    // TODO
    enemy.pos.x += enemy.movement_vec.x * enemy.speed * enemy.time_scale;
    enemy.pos.y += enemy.movement_vec.y * enemy.speed * enemy.time_scale;

    // if last enemy in room, switch to chase
    if blackboard.enemy_quantity == 1
    {
        enemy.state = State::Chase;
    }
}

pub fn take_cover(_enemy: & mut Enemy, _blackboard: &BlackBoard){

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

    // TODO
        enemy.pos.x += enemy.movement_vec.x * enemy.speed * enemy.time_scale;
        enemy.pos.y += enemy.movement_vec.y * enemy.speed * enemy.time_scale;

    // if close to player, switch to attack
    if Enemy::player_close(enemy, blackboard)
    {
        enemy.state = State::Attack;
    }

    // if low on life, and no health enemy, retreat
    if (enemy.hp as f32) <= enemy.m_hp as f32/3.0 &&
    (blackboard.enemy_quantity > 1 &&
    !blackboard.types_in_room.iter().any(|&i| i==EnemyKind::Health))&&
    !blackboard.boss_fight{ //True if there isn't a health enemy
        enemy.state = State::Retreat;
    }

    // don't need heal for red enemy

}

pub fn heal(enemy: & mut Enemy, _blackboard: &BlackBoard){
    //println!("Im healing somebody else");
    if !enemy.is_healing {
        enemy.state = State::Chase;
    }

    enemy.is_healing = false;

}

pub fn idle(enemy: & mut Enemy, blackboard: &BlackBoard){
    if Enemy::distance_to_player(enemy, blackboard) > 0.0
    {
        enemy.state = State::Chase;
    }
}
