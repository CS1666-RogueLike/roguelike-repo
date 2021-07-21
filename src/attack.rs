use crate::boxes::*;
use crate::boxes::Box;
use crate::util::*;
use crate::entity::*;

#[derive(Clone)]
pub struct AtkProjectile {
	pub pos: Vec2<f32>,
	pub box_es: Box,
	pub speed: f32,
    pub movement_vec: Vec2<f32>,
    pub damage: i32,
}

impl AtkProjectile {
    pub fn new(pos : Vec2<f32>, movement_vec : Vec2<f32>, enemy_kind : &EnemyKind) -> AtkProjectile {
        AtkProjectile {
        	pos : pos, //Where the attack is
        	movement_vec : movement_vec, //What direction its moving
        	speed : getSpeed(enemy_kind), //How fast it is
        	damage : getDamage(enemy_kind), //How much it will damage the player when collided with
        	box_es : getBoxes(enemy_kind), //The hitbox of it
        }
    }
}

fn getSpeed(enemy_kind : & EnemyKind) -> f32 {
	let mut speed;
    match enemy_kind {
		EnemyKind::Health => {
			speed = 2.0;
		}
		EnemyKind::Speed =>{
			speed = 4.0;
		}
		EnemyKind::Attack => {
			speed = 3.0;
		}
	}
	return speed;
}

fn getDamage(enemy_kind : & EnemyKind) -> i32 {
	let mut damage;
    match enemy_kind {
		EnemyKind::Health => {
			damage = 2;
		}
		EnemyKind::Speed =>{
			damage = 1;
		}
		EnemyKind::Attack => {
			damage = 3;
		}
	}
	return damage;
}

fn getBoxes(enemy_kind : & EnemyKind) -> Box {
	let mut box_es;
    match enemy_kind {
		EnemyKind::Health => {
			box_es = Box::new(Vec2::new(30, 30), Vec2::new(30, 30), Vec2::new(30, 30));
		}
		EnemyKind::Speed =>{
			box_es = Box::new(Vec2::new(10, 10), Vec2::new(10, 10), Vec2::new(10, 10));
		}
		EnemyKind::Attack => {
			box_es = Box::new(Vec2::new(20, 20), Vec2::new(20, 20), Vec2::new(20, 20));
		}
	}
	return box_es;
}