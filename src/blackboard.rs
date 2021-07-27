use crate::util::*;
use crate::game::*;
use crate::entity::*;


pub struct BlackBoard
{
    pub playerpos: Vec2<f32>,
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
            playerHealth: -1,
            enemyQuantity: -1,
            typesInRoom: Vec::<EnemyKind>::new(),
        }
    }
    
    pub fn update(&mut self, game: &Game){
        self.playerpos = game.player.pos;
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
