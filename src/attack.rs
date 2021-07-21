use crate::boxes::*;
use crate::boxes::Box;
use crate::util::*;
use crate::entity::*;


pub struct AtkProjectile {
	pub pos: Vec2<f32>,
	pub box_es: Box,
	pub speed: f32,
    pub movement_vec: Vec2<f32>,
    pub damage: i32,
}

impl AtkProjectile {
    pub fn new(pos : Vec2<f32>, movement_vec : Vec2<f32>, enemy_kind : EnemyKind) -> AtkProjectile {
        AtkProjectile {
        	pos : pos,
        	movement_vec : movement_vec,
        	speed : getSpeed(&enemy_kind),
        	damage : getDamage(&enemy_kind),
        	box_es : getBoxes(&enemy_kind),
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
			box_es = Box::new(Vec2::new(20, 20), Vec2::new(20, 20), Vec2::new(20, 20));
		}
		EnemyKind::Speed =>{
			box_es = Box::new(Vec2::new(10, 10), Vec2::new(10, 10), Vec2::new(10, 10));
		}
		EnemyKind::Attack => {
			box_es = Box::new(Vec2::new(16, 16), Vec2::new(16, 16), Vec2::new(16, 16));
		}
	}
	return box_es;
}