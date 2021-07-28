use crate::util::*;
use crate::game::*;
use crate::entity::*;
use crate::boxes::*;


pub struct BlackBoard
{
    pub playerpos: Vec2<f32>,
    pub player_frame_tile: Vec2<i32>,
    pub player_box: Box,
    pub player_health: i32,
    pub enemy_quantity: i32,
    //pub bomb:
    pub types_in_room: Vec<EnemyKind>,
    //pub playerGemStatus:
    
}

impl BlackBoard{
    pub fn new() -> BlackBoard{ 
        BlackBoard{
            playerpos : Vec2::new(0.0, 0.0),
            player_frame_tile : Vec2::new(0, 0),
            player_box : Box::new(Vec2::new(0,0), Vec2::new(0,0), Vec2::new(0,0)),
            player_health: -1,
            enemy_quantity: -1,
            types_in_room: Vec::<EnemyKind>::new(),
        }
    }
    
    pub fn update(&mut self, game: &Game){
        self.playerpos = game.player.pos;
        self.player_frame_tile = game.player.current_frame_tile;
        self.player_box = game.player.box_es;
        self.player_health = game.player.hp;
        self.enemy_quantity = BlackBoard::get_enemy_quantity(game);
        self.types_in_room = BlackBoard::get_types_in_room(game);
    }
    
    pub fn get_types_in_room(game: &Game) -> Vec<EnemyKind> {
        let mut v = vec![];
        for enemy in game.current_room().enemies.iter() {
            v.push(enemy.kind);
        }

        v.dedup_by(|a, b| Enemy::type_eq(*a, *b));
        
        return v;
        
    }
    
    pub fn get_enemy_quantity(game: &Game) -> i32 {
        let mut qty = 0;
        
        for enemy in game.current_room().enemies.iter()
        {
            if !enemy.death {
                qty += 1;
            }
        }
        
        return qty;
    }
}
