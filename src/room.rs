use crate::util::*;
use crate::tile::*;
use crate::entity::*;
use rand::Rng;

pub const ROOM_WIDTH: i32 = 17;
pub const ROOM_HEIGHT: i32 = 11;


pub struct Room {
    pub exists: bool,
    pub visited: bool,
    pub tiles: Vec<Vec<Box<dyn Tile>>>,
    pub enemies: Vec<Enemy>,
    pub gemCount: i32,
}

/*
 *  IMPORTANT NOTE ABOUT TRAITS
 *
 *  For a structure like a vec! to work properly, it needs to know the exact size of the objects
 *  it's containing. However, traits are simply interfaces implemented by types, and thus objects
 *  can be multiple sizes. Because their size is therefore unknown, they can't be stored in
 *  containers alone. Instead, we have to allocate them to the heap using a Box (a type that is of
 *  a consistent size), then act on things through the box.
 */

impl Room {
    // Returns a room that the developer sets every tile of manually.
    pub fn non_room() -> Room {
        Room { exists: false, visited: false, tiles: Vec::new(), enemies: Vec::new(), gemCount: 0 }

    }
    pub fn new_test_room(blueprint: [[char; 17]; 11]) -> Room {

        // ----------------------- READ THIS!!!!!!!!!!!!!!!!! -----------------------
        // Manually defining the room array is needed, but the syntax to do that manually would be a mess.
        // Instead, we can define an array of the same size of just characters, with each character
        // representing a type of tile. Then, we translate that tile from this easy to view setup
        // to an actual room array filled with tiles.

        // KEY:
        // _ -> Ground (to make looking at it easier)
        // W -> Wall
        // R -> Rock
        // P -> Pit
        // D -> Door
        // S -> Spike

        // BLUEPRINT ARRAY SHOULD BE DEFINED IN FLOOR.RS AND PASSED IN

        // Vec that contains actual Tile trait implementing structs
        let mut tiles: Vec<Vec<Box<dyn Tile>>> = Vec::new();

        for y in 0..ROOM_HEIGHT {
            // Add a row to our struct
            tiles.push(Vec::new());
            for x in 0..ROOM_WIDTH {
                match blueprint[y as usize][x as usize] {

                    // These have to be in boxes because the compiler does know how big the
                    // implementations of the trait are. Because a box is essentially a pointer,
                    // it's of a size the compiler knows about. Thus, we give it a pointer to an
                    // implementation of the type, and the compiler is satisfied.
                    '_' => tiles[y as usize].push(Box::new(Ground { gem: Gem::None })),
                    'r' => tiles[y as usize].push(Box::new(Ground { gem: Gem::Red })),
                    'y' => tiles[y as usize].push(Box::new(Ground { gem: Gem::Yellow })),
                    'b' => tiles[y as usize].push(Box::new(Ground { gem: Gem::Blue })),
                    'Q' => tiles[y as usize].push(Box::new(Bomb {has_bomb: true})),
                    'W' => tiles[y as usize].push(Box::new(Wall {})),
                    'R' => tiles[y as usize].push(Box::new(Rock {})),
                    'P' => tiles[y as usize].push(Box::new(Pit {})),
                    'S' => tiles[y as usize].push(Box::new(Spike { gem: Gem::None })),
                    // TODO: Add code for proper handling of direction
                    'D' => tiles[y as usize].push(Box::new(Door { lock: LockState::Unlocked, position: Direction::Up })),

                    'K' => tiles[y as usize].push(Box::new(Key { has_key: true, })),
                    'T' => tiles[y as usize].push(Box::new(Trapdoor { lock: LockState::Locked })),

                    _ => panic!("NO MATCH FOR TILE TYPE"), // NOTE THAT THIS IS DIFFERENT FROM '_' WHICH CHECKS FOR THE UNDERSCORE CHARACTER
                    // This needs to panic if an unrecogized character is found,
                    // otherwise the rooms won't be the right size and a bunch
                    // of crazy buggy stuff could happen.
                }
            }
        }


        /*
        // Debug print to make sure structure was built properly
        println!(" -------- FROM ACTUAL TRAIT STORING ARRAY -------");
        for row in &tiles {
            for t in row {
                match t.walkability() {
                    Walkability::Floor => print!(" _ "),
                    Walkability::Wall => print!(" W "),
                    Walkability::Rock => print!(" R "),

                    _ => panic!("NO MATCH FOR TILE TYPE"),
                    // This needs to panic, otherwise the rooms won't be the right size and a bunch
                    // of crazy buggy stuff could happen.

                }
            }
            println!();
        }
        */

        // Return room struct.
        Room {
            exists: true,
            visited: false,
            tiles: tiles,
            enemies: Vec::new(),
            gemCount: 0,
        }
    }

    pub fn tile_at(&mut self, x: i32, y: i32) -> &mut Box<dyn Tile> {
        &mut self.tiles[ ((y - TOP_WALL) / 64) as usize ][ ((x - LEFT_WALL) / 64) as usize ]
    }

    pub fn tile_at_vec2(&mut self, pos: Vec2<i32>) -> &mut Box<dyn Tile> {
        self.tile_at( pos.x, pos.y )
    }

    pub fn add_enemies(&mut self, enemies: Vec<Enemy>) {
        self.enemies = enemies;
    }

    pub fn increment_gem(&mut self){
        self.gemCount += 1
    }

    pub fn additional_enemies(&mut self, enemy: Enemy) {
        self.enemies.push(enemy);
    }

    // Repositions enemies to be a certain distance from player
    // Avoids player taking immediate damage they can't prevent
    pub fn reposition_enemies(&mut self, player_pos: Vec2<f32>) {

        //println!("Repositioning enemies...");

        let mut rng = rand::thread_rng();

        for mut enemy in &mut self.enemies {

            loop {
                // Get distance between player and enemy
                let mut dist = ((enemy.pos.x - player_pos.x).powf(2.0) + (enemy.pos.y - player_pos.y).powf(2.0)).sqrt();

                // See if tile enemy is on is walkable
                let mut valid_tile = self.tiles
                    [((enemy.pos.y as i32 - TOP_WALL) / 64) as usize]
                    [((enemy.pos.x as i32 - LEFT_WALL) / 64) as usize]
                    .walkability() == Walkability::Floor;

                // Reroll enemy position if too close or not walkable
                if dist < 250.0 || !valid_tile {
                    enemy.pos.x = rng.gen_range(LEFT_WALL..RIGHT_WALL) as f32;
                    enemy.pos.y = rng.gen_range(TOP_WALL..BOT_WALL) as f32;
                }
                // If enemy is far enough away, go to next enemy
                else {
                    break
                }
            }
        }
    }

    // Removes enemies when the player is at low hp
    pub fn ease_enemy_difficulty(&mut self, hp: i32) {
        
        //for mut enemy in &mut self.enemies { enemy.is_ranged = false; }
        
        match hp {

            // When at 1 heart, get rid of yellow enemies to avoid player getting one shot
            2 => {
                self.enemies.retain(
                    |enemy| {
                        enemy.kind == EnemyKind::Health || enemy.kind == EnemyKind::Speed
                    }
                );
            }

            // When at half a heart, only spawn 1 enemy red enemy.
            1 => {
                self.enemies.clear();
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(LEFT_WALL..RIGHT_WALL) as f32;
                let y = rng.gen_range(TOP_WALL..BOT_WALL) as f32;
                self.enemies.push(Enemy::new(Vec2 {x, y}, EnemyKind::Health));
            }

            _ => {}
        }
        // When at lower than 1.5 hearts, don't spawn ranged enemies
        if hp <= 3 {
            for mut enemy in &mut self.enemies { enemy.is_ranged = false; }
        }

    }

    // Provides enemies with time_scale so they can do frame independent movement
    pub fn update_enemies(&mut self, ts: f32) {
        for mut x in &mut self.enemies {
            x.time_scale = ts;
        }
    }
}
