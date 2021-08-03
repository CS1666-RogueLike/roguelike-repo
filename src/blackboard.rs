use crate::util::*;
use crate::game::*;
use crate::entity::*;
use crate::boxes::*;
use crate::tile::*;
use crate::room::*;
use crate::player::*;
use sdl2::rect::Rect;

use rand::Rng;


pub struct BlackBoard
{
    pub playerpos: Vec2<f32>,
    pub player_frame_tile: Vec2<i32>,
    pub player_box: Box,
    pub player_health: i32,
    pub player_max_health: i32,
    pub player_speed: f32,
    pub player_attack: i32,
    pub player_charged: bool,
    pub enemy_quantity: i32,
    pub health_enemy_pos: Vec<Vec2<f32>>,
    pub health_enemy_tile: Vec<Vec2<i32>>,
    pub health_enemy_hitbox: Vec<Rect>,
    pub types_in_room: Vec<EnemyKind>,
    pub yellow_chicken: bool,

    pub boss_kind: EnemyKind,

    pub boss_fight: bool,


    pub cr_tiles: Vec<Vec<std::boxed::Box<dyn Tile>>>,

}

impl BlackBoard{
    pub fn new() -> BlackBoard{
        BlackBoard{
            playerpos : Vec2::new(0.0, 0.0),
            player_frame_tile : Vec2::new(0, 0),
            player_box : Box::new(Vec2::new(0,0), Vec2::new(0,0), Vec2::new(0,0)),
            player_health: -1,
            player_max_health: -1,
            player_speed: -1.0,
            player_attack: -1,
            player_charged: false,
            enemy_quantity: -1,
            //offset: 0,
            health_enemy_pos:Vec::new(),
            health_enemy_tile:Vec::new(),
            health_enemy_hitbox:Vec::<Rect>::new(),
            types_in_room: Vec::<EnemyKind>::new(),
            yellow_chicken: false,

            boss_kind: EnemyKind::Final,

            boss_fight: false,
            //Not updated normally, updated only when the room changes
            cr_tiles : Vec::new(),
        }
    }

    pub fn update(&mut self, game: &Game){
        self.playerpos = game.player.pos;
        self.player_frame_tile = game.player.current_frame_tile;
        self.player_box = game.player.box_es;
        self.player_health = game.player.hp;
        self.player_max_health = game.player.m_hp;
        self.player_speed = game.player.speed;
        self.player_attack = game.player.attack;
        self.player_charged = game.player.is_charging;
        self.enemy_quantity = BlackBoard::get_enemy_quantity(game);
        self.health_enemy_pos = BlackBoard::get_health_enemy_pos(game);
        self.health_enemy_tile = BlackBoard::get_health_enemy_tile(&self.health_enemy_pos);
        self.health_enemy_hitbox = BlackBoard::get_health_enemy_hitbox(game);
        self.types_in_room = BlackBoard::get_types_in_room(game);
        self.yellow_chicken = BlackBoard::active_yellow_retreat(game);
        self.boss_fight = self.check_boss_fight();
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

    pub fn active_yellow_retreat(game: &Game) -> bool {
        for enemy in game.current_room().enemies.iter() {
            if !enemy.death && enemy.kind==EnemyKind::Attack && enemy.state == State::Retreat {
                return true;
            }
        }

        return false;

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
        //println!("{}", qty);
        return qty;
    }

    pub fn is_walkable(&self, tile: Vec2<i32>)-> bool {
        match self.cr_tiles[tile.y as usize][tile.x as usize].walkability(){
            Walkability::Wall | Walkability::Rock | Walkability::Pit => {false}
            _ => {true}

        }
    }

    pub fn set_boss_type(&mut self){
    let speed_powerups = ((self.player_speed - PLAYER_SPEED)/20.0) as i32;
    let atk_powerups = self.player_attack - P_DEFAULT_ATK;
    let health_powerups= (self.player_max_health - P_MAX_HP)/2;
    let mut high_kind = EnemyKind::Final;

    if speed_powerups > atk_powerups && speed_powerups > health_powerups{
        high_kind = EnemyKind::Speed;
    }
    else if atk_powerups > speed_powerups && atk_powerups > health_powerups{
        high_kind = EnemyKind::Health;
    }
    else if health_powerups > atk_powerups && health_powerups > speed_powerups{
        high_kind = EnemyKind::Attack;
    }
    else if health_powerups == atk_powerups && atk_powerups == speed_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 2 ){
            0=>{
                high_kind = EnemyKind::Speed;
            }
            1=>{
                high_kind = EnemyKind::Health;
            }
            2=>{
                high_kind = EnemyKind::Attack;
            }
            _ => {}
        }
    }
    else if health_powerups == atk_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 1 ){
            0=>{
                high_kind = EnemyKind::Attack;
            }
            1=>{
                high_kind = EnemyKind::Health;
            }
            _ => {}
        }
    }
    else if health_powerups == speed_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 1 ){
            0=>{
                high_kind = EnemyKind::Speed;
            }
            1=>{
                high_kind = EnemyKind::Attack;
            }
            _ => {}
        }
    }
    else if atk_powerups == speed_powerups{
        let mut rng = rand::thread_rng();
        match rng.gen_range( 0 ..= 1 ){
            0=>{
                high_kind = EnemyKind::Health;
            }
            1=>{
                high_kind = EnemyKind::Speed;
            }
            _ => {}
        }
    }
    else{
        println!{"Scrappy Doo, i guess"};
    }
    self.boss_kind = high_kind;

}

    pub fn check_boss_fight(&self) -> bool{
        if self.types_in_room.iter().any(|&i| i==EnemyKind::Final){
            return true;
        }
        false
    }
}
