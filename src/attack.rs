
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
    pub kind: EnemyKind,
}

impl AtkProjectile {
    pub fn new(pos : Vec2<f32>, movement_vec : Vec2<f32>, enemy_kind : &EnemyKind) -> AtkProjectile {
        AtkProjectile {
        	pos : pos, //Where the attack is
        	movement_vec : movement_vec, //What direction its moving
        	speed : get_speed(enemy_kind), //How fast it is
        	damage : get_damage(enemy_kind), //How much it will damage the player when collided with
        	box_es : get_boxes(enemy_kind), //The hitbox of it
            kind : *enemy_kind, //What kind of attack it is
        }
    }
}
 //Set speed of attack projectile based on what kind of enemy it is
fn get_speed(enemy_kind : & EnemyKind) -> f32 {
	let speed;
    match enemy_kind {
		EnemyKind::Health => {
			speed = HEALTH_ENEMY_ATK_PROJSPEED;
		}
		EnemyKind::Speed =>{
			speed = SPEED_ENEMY_ATK_PROJSPEED;
		}
		EnemyKind::Attack => {
			speed = ATTACK_ENEMY_ATK_PROJSPEED;
		}
		_ => {speed = 0.0;}
	}
	return speed;
}

 //Set damage of attack projectile based on what kind of enemy it is
fn get_damage(enemy_kind : & EnemyKind) -> i32 {
	let damage;
    match enemy_kind {
		EnemyKind::Health => {
			damage = HEALTH_ENEMY_ATK_DMG;
		}
		EnemyKind::Speed =>{
			damage = SPEED_ENEMY_ATK_DMG;
		}
		EnemyKind::Attack => {
			damage = ATTACK_ENEMY_ATK_DMG;
		}
		_ => {damage = 0;}
	}
	return damage;
}

 //Set hitbox and walkbox of attack projectile based on what kind of enemy it is
fn get_boxes(enemy_kind : & EnemyKind) -> Box {
	let box_es;
    match enemy_kind {
		EnemyKind::Health => {
			box_es = Box::new(
                Vec2::new(HEALTH_ENEMY_ATK_WIDTH, HEALTH_ENEMY_ATK_HEIGHT), //hitbox (deals damage)
                Vec2::new(HEALTH_ENEMY_ATK_WIDTH, HEALTH_ENEMY_ATK_HEIGHT), //walkbox (interacts with enviornment)
                Vec2::new(0, 0) //NOT FOR USE WITH ANYTHING OTHER THAN PLAYER
            );
		}
		EnemyKind::Speed =>{
			box_es = Box::new(
                Vec2::new(SPEED_ENEMY_ATK_WIDTH, SPEED_ENEMY_ATK_HEIGHT),
                Vec2::new(SPEED_ENEMY_ATK_WIDTH, SPEED_ENEMY_ATK_HEIGHT),
                Vec2::new(0, 0)
            );
		}
		EnemyKind::Attack => {
			box_es = Box::new(
                Vec2::new(ATTACK_ENEMY_ATK_WIDTH, ATTACK_ENEMY_ATK_HEIGHT),
                Vec2::new(ATTACK_ENEMY_ATK_WIDTH, ATTACK_ENEMY_ATK_HEIGHT),
                Vec2::new(0, 0)
            );
		}
		_ => {
			box_es = Box::new(
                Vec2::new(0, 0), //hitbox (deals damage)
                Vec2::new(0, 0), //walkbox (interacts with enviornment)
                Vec2::new(0, 0) //NOT FOR USE WITH ANYTHING OTHER THAN PLAYER
            );
		}
	}
	return box_es;
}

