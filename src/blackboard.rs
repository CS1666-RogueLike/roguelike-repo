use crate::util::*;
use crate::game::*;
use crate::entity::*;
use crate::boxes::*;


pub struct BlackBoard
{
    pub playerpos: Vec2<f32>,
    pub playerFrameTile: Vec2<i32>,
    pub playerBox: Box,
    pub playerHealth: i32,
    pub enemyQuantity: i32,
    //pub bomb:
    pub typesInRoom: Vec<EnemyKind>,
    //pub playerGemStatus:
    
}

impl BlackBoard{
    pub fn new() -> BlackBoard{ 
        BlackBoard{
            playerpos : Vec2::new(0.0, 0.0),
            playerFrameTile : Vec2::new(0, 0),
            playerBox : Box::new(Vec2::new(0,0), Vec2::new(0,0), Vec2::new(0,0)),
            playerHealth: -1,
            enemyQuantity: -1,
            typesInRoom: Vec::<EnemyKind>::new(),
        }
    }
    
    pub fn update(&mut self, game: &Game){
        self.playerpos = game.player.pos;
        self.playerFrameTile = game.player.current_frame_tile;
        self.playerBox = game.player.box_es;
        self.playerHealth = game.player.hp;
        self.enemyQuantity = BlackBoard::getEnemyQuantity(game);
        self.typesInRoom = BlackBoard::getTypesInRoom(game);
    }
    
    pub fn getTypesInRoom(game: &Game) -> Vec<EnemyKind> {
        let mut v = vec![];
        for enemy in game.current_room().enemies.iter() {
            v.push(enemy.kind);
        }

        v.dedup_by(|a, b| Enemy::type_eq(*a, *b));
        
        return v;
        
    }
    
    pub fn getEnemyQuantity(game: &Game) -> i32 {
        let mut qty = 0;
        
        for enemy in game.current_room().enemies.iter()
        {
            if(!enemy.death) {
                qty += 1;
            }
        }
        
        return qty;
    }
}
