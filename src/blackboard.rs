use crate::util::*;
use crate::game::*;
use crate::entity::*;
use crate::boxes::*;
use crate::tile::*;
use crate::room::*;
use sdl2::rect::Rect;


pub struct BlackBoard
{
    pub playerpos: Vec2<f32>,
    pub player_frame_tile: Vec2<i32>,
    pub player_box: Box,
    pub player_health: i32,
    pub enemy_quantity: i32,
    pub health_enemy_pos: Vec<Vec2<f32>>,
    pub health_enemy_tile: Vec<Vec2<i32>>,
    pub health_enemy_hitbox: Vec<Rect>,
    //pub offset: i32,
    //pub bomb:
    pub types_in_room: Vec<EnemyKind>,
    //pub playerGemStatus:
    
    pub cr_tiles: Vec<Vec<std::boxed::Box<dyn Tile>>>,

}

impl BlackBoard{
    pub fn new() -> BlackBoard{
        BlackBoard{
            playerpos : Vec2::new(0.0, 0.0),
            player_frame_tile : Vec2::new(0, 0),
            player_box : Box::new(Vec2::new(0,0), Vec2::new(0,0), Vec2::new(0,0)),
            player_health: -1,
            enemy_quantity: -1,
            //offset: 0,
            health_enemy_pos:Vec::new(),
            health_enemy_tile:Vec::new(),
            health_enemy_hitbox:Vec::<Rect>::new(),
            types_in_room: Vec::<EnemyKind>::new(),
            
            //Not updated normally, updated only when the room changes
            cr_tiles : Vec::new(),
        }
    }

    pub fn update(&mut self, game: &Game){
        self.playerpos = game.player.pos;
        self.player_frame_tile = game.player.current_frame_tile;
        self.player_box = game.player.box_es;
        self.player_health = game.player.hp;
        self.enemy_quantity = BlackBoard::get_enemy_quantity(game);
        self.health_enemy_pos = BlackBoard::get_health_enemy_pos(game);
        self.health_enemy_tile = BlackBoard::get_health_enemy_tile(&self.health_enemy_pos);
        self.health_enemy_hitbox = BlackBoard::get_health_enemy_hitbox(game);
        self.types_in_room = BlackBoard::get_types_in_room(game);
        //self.offset = ofs;
    }
    
    pub fn update_room(&mut self, game: &Game){
        
        let mut tiles: Vec<Vec<std::boxed::Box<dyn Tile>>> = Vec::new();
        for y in 0..ROOM_HEIGHT {
            // Add a row to our struct
            tiles.push(Vec::new());
            for x in 0..ROOM_WIDTH {
                    match(game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles[y as usize][x as usize]).walkability(){
                        Walkability::Floor => {tiles[y as usize].push(std::boxed::Box::new(Ground { gem: Gem::None }))}
                        Walkability::Spike => {tiles[y as usize].push(std::boxed::Box::new(Spike { gem: Gem::None}))}
                        Walkability::Pit => {tiles[y as usize].push(std::boxed::Box::new(Pit {}))}
                        Walkability::Wall => {tiles[y as usize].push(std::boxed::Box::new(Wall {}))}
                        Walkability::Rock => {tiles[y as usize].push(std::boxed::Box::new(Rock {}))}
                    }
                }
            }
            
        self.cr_tiles = tiles;
            
        //let mut tiles: Vec<Vec<std::boxed::Box<dyn Tile>>> = Vec::new();
        //tiles.copy_from_slice(&game.map.floors[game.cf].rooms[game.cr.y as usize][game.cr.x as usize].tiles[0..]);
    }

    pub fn get_types_in_room(game: &Game) -> Vec<EnemyKind> {
        let mut v = vec![];
        for enemy in game.current_room().enemies.iter() {
            if !enemy.death {
                v.push(enemy.kind);
            }
        }

        v.dedup_by(|a, b| Enemy::type_eq(*a, *b));

        return v;

    }
    // i think it would be easier to just get the whole health enemy if possible
    pub fn get_health_enemy_hitbox(game: &Game) -> Vec<Rect> {
        let mut v = vec![];
        for enemy in game.current_room().enemies.iter() {
            if !enemy.death && enemy.kind==EnemyKind::Health{
                    v.push(enemy.box_es.get_hitbox(enemy.pos));
            }
        }
        return v;
    }

    pub fn get_health_enemy_pos(game: &Game) -> Vec<Vec2<f32>> {
        let mut v = vec![];
        for enemy in game.current_room().enemies.iter() {
            if !enemy.death && enemy.kind==EnemyKind::Health{
                    v.push(enemy.pos);
            }
        }
        return v;
    }

    pub fn get_health_enemy_tile(i: &Vec<Vec2<f32>>) -> Vec<Vec2<i32>> {
        let mut v = vec![];
        for pos in i.iter(){
                let mut tile = Vec2::new(
                    (pos.x as i32- LEFT_WALL) / TILE_WIDTH,
                    (pos.y as i32- TOP_WALL) / TILE_WIDTH
                );
                v.push(tile);
        }
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
    
    pub fn is_walkable(&self, tile: Vec2<i32>)-> bool {
        match self.cr_tiles[tile.y as usize][tile.x as usize].walkability(){
            Walkability::Wall | Walkability::Rock | Walkability::Pit => {false}
            _ => {true}
            
        }
    }
}
