use crate::blackboard::*;
use crate::attack::*;
use crate::util::*;
//use sdl2::rect::Rect;
//use std::time::{Duration, Instant};
//use crate::boxes::*;
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

    println!("Daphne");

    let mut vector = Vec2::new(blackboard.playerpos.x - enemy.pos.x, blackboard.playerpos.y - enemy.pos.y);
    let length = ((vector.x * vector.x + vector.y * vector.y) as f64).sqrt();

    // normalize vector
    vector.x /= length as f32;
    vector.y /= length as f32;

    let new_atk = AtkProjectile::new(enemy.pos, vector, &EnemyKind::Attack);
    enemy.atk_list.push(new_atk);

    // if blackboard.playerpos.y >= 400.0
    // {
    //     enemy.state = State::Chase;
    // }


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
            match rng.gen_range( 0 ..= 4 ){
                0 | 1 => {
                    let mut enemies = Enemy::new(Vec2::new(enemy.box_left_final_pos.x, enemy.box_left_final_pos.y), EnemyKind::Speed);
                    enemy.add_enemies(enemies);
                },
                2 | 3 => {
                    let mut enemies = Enemy::new(Vec2::new(enemy.box_right_final_pos.x, enemy.box_right_final_pos.y), EnemyKind::Speed);
                    enemy.add_enemies(enemies);
                },
                _ => {enemy.is_attacking = false}

        }
    }

    if blackboard.playerpos.y > 400.0
    {
        enemy.state = State::Attack;
    }

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
        enemy.state = State::Attack;
    }
}
