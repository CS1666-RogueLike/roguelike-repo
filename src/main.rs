extern crate roguelike;
//use sdl2::audio::{AudioCallback, AudioSpecDesired};

mod menu;
use crate::menu::*;

mod game;
use crate::game::*;

mod player;
mod entity;
mod attack;

mod blackboard;
use crate::blackboard::*;

mod yellowenemy;
mod redenemy;
mod blueenemy;

mod finalenemy;
//use crate::finalenemy::*;

mod util;
use crate::util::*;

mod collision;
//use crate::collision::*;
mod walkover;
mod draw;

mod map;
mod floor;
mod procgen;
mod room;
mod tile;
mod boxes;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use roguelike::SDLCore;
use roguelike::Demo;
use entity::Health;
use player::PowerUp;

use entity::*;


//use std::cmp::min;
use std::collections::HashSet;
//use std::collections::VecDeque;
use std::time::Instant;


use std::time::Duration;

// TODO: Move all sdl code to a separate file, keep the main.rs file simple



fn main() {
    // Initializes SDL and hands off control.
    roguelike::runner(TITLE, Manager::init);
}

// Manager struct responsible for working with SDL, menu system, initializing game, etc.
pub struct Manager {
    core: SDLCore, // SDL data for drawing.
    debug: bool,
    menu: MenuState, // Enum that controls the control flow via the menu.
    game: Game, // Struct holding all game related data.
    blackboard: BlackBoard, //Struct for holding game data that the enemy needs to access
    prev_frame: Instant,
    cur_frame: Instant,
    time_scale: f32,
}

impl Demo for Manager {
    // Initialize manager struct.
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, VSYNC, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let debug = false;
        let menu = MenuState::MainMenu;
        let blackboard = BlackBoard::new();
        let mut game = Game::new();
        game.changed_floors = false;
        game.transition_start = Instant::now();
        let prev_frame = Instant::now();
        let cur_frame = Instant::now();
        let time_scale = 1.0;

        Ok(Manager{core, debug, menu, game, blackboard, prev_frame, cur_frame, time_scale })
    }

    fn run(&mut self) -> Result<(), String> {

        // Print controls to terminal.
        println!("");
        println!("");
        println!(" --------------- CONTROLS --------------- ");
        println!("");
        println!("\t1\t\tTurn OFF debug graphics");
        println!("\t2\t\tTurn ON debug graphics");
        println!("\t3\t\tLock doors");
        println!("\t4\t\tUnlock doors");
        println!("");
        println!("\tW\t\tMove Up");
        println!("\tS\t\tMove Down");
        println!("\tA\t\tMove Left");
        println!("\tD\t\tMove Right");
        println!("");
        println!("\tUp Arrow\tLook Up");
        println!("\tDown Arrow\tLook Down");
        println!("\tLeft Arrow\tLook Left");
        println!("\tRight Arrow\tLook Right");
        println!("");
        println!("\tSpace\t\tShort-range attack (cardinal directions only)");
        println!("\tEscape\t\tPause game (while in game, not menus)");
        println!("");
        println!("\tQ\t\tUse Bomb");
        println!("\tH\t\tTest health power up");
        println!("\tJ\t\tTest speed power up");
        println!("\tK\t\tTest attack power up");
        println!("");

        println!("Health is: {}", self.game.player.health());
        println!("Max Health is: {}", self.game.player.max_hp());


        // Hacky solution for pause menu
        let mut esc_prev = false;
        let mut esc_curr = false;

        //sdl2::mixer::open_audio(Some(44100),)?;
        // let music = sdl2::mixer::Music::from_file("assets/Blob Style.mp3")?;
        // music.play(1);
        //Duration::from_millis(100000);

        //play::play("assets/Blob Style.mp3").unwrap();

        //println!("DOES THE ROOM EXIST? {}", self.game.current_room().exists);

        'gameloop: loop {

            // Record time at current frame + set prev frame time
            // This is used for frame independent movement
            self.prev_frame = self.cur_frame;
            self.cur_frame = Instant::now();

            // One frame at 60fps is 16.6 ms
            // This gives us a scale value to adjust movements
            self.time_scale = (self.cur_frame - self.prev_frame).as_micros() as f32 * 0.001 / 16.6;
            //println!("{}", dif);

            // Pass time scale to enemies in current room
            self.game.current_room_mut().update_enemies(self.time_scale);

            // Check for press of close window button.
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'gameloop,
                    Event::KeyUp {keycode: Some(Keycode::H), repeat: false, ..} =>
                        { self.game.player.plus_power_health();
                            //println!("PowerupHealth is {}", self.game.player.power_up_vec[0]);
                            //println!("Max Health is: {}", self.game.player.max_hp());
                        },
                    Event::KeyUp {keycode: Some(Keycode::J), repeat: false, ..} =>
                        { self.game.player.plus_power_speed();
                            //println!("PowerupSpeed is {}", self.game.player.power_up_vec[1]);
                        },
                    Event::KeyUp {keycode: Some(Keycode::K), repeat: false, ..} =>
                        { self.game.player.plus_power_attack();
                            //println!("PowerupAttack is {}", self.game.player.power_up_vec[2]);
                        },
                    _ => {},
                }
            }

            // Create a hashmap for easy key lookup.
            let keystate: HashSet<Keycode> = self.core.event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            // Namespace enums for readability.
            use menu::MenuState::*;

            // Movement vector to determine movement direction.
            let mut mov_vec = Vec2::new(0.0, 0.0);

            // Filter inputs by menu state.
            match self.menu {

                Victory => {
                    //println!("Press Escape to return to main menu.");
                    if keystate.contains(&Keycode::Escape) {
                        self.menu = MainMenu;
                    }

                }

                MainMenu => {
                    if keystate.contains(&Keycode::Space) {
                        self.menu = GameActive;
                        self.game = Game::new(); // Initialize a new game
                        // This makes it so exiting to menu and staring the game again works
                        // properly
                        //println!("Switch");
                    }
                }

                GameActive => {

                    match self.game.game_state {

                        // Handles move from main menu to actual gameplay by displaying "Floor 1"
                        GameState::InitialFloorTrans => {
                            if self.game.transition_start.elapsed().as_millis() > 2500 {
                                self.blackboard.update_room(& self.game);
                                self.game.game_state = GameState::Gameplay;
                            }
                        }

                        // Sliding transitions between rooms
                        GameState::BetweenRooms => {
                            //sleep(Duration::new(0, 500_000_000)); // 500 mil is half second
                            if self.game.transition_start.elapsed().as_millis() > 400 {
                                self.blackboard.update_room(& self.game);
                                self.game.game_state = GameState::Gameplay;
                            }

                        }

                        // Black bar transitions between rooms + floor number
                        GameState::BetweenFloors => {
                            if self.game.transition_start.elapsed().as_millis() > 3000 {
                                self.blackboard.update_room(& self.game);
                                self.game.game_state = GameState::Gameplay;
                            }

                            // Change the room at the appropriate time
                            // If we change it right away we'll see it before the transition is over
                            if self.game.changed_floors == false && self.game.transition_start.elapsed().as_millis() > 500 {
                                if self.game.cf == 3 {
                                    self.menu = MenuState::Victory;
                                } else {
                                    //Tell the game which type of final boss to generate
                                    if self.game.cf == 2 {
                                        self.blackboard.set_boss_type();
                                    }
                                    // Next floor
                                    self.game.cf += 1;

                                    // Reset current room
                                    self.game.cr.x = START_X;
                                    self.game.cr.y = START_Y;

                                    // Center player in room
                                    self.game.player.pos.x = (LEFT_WALL + 8 * 64) as f32 + 32.0;
                                    self.game.player.pos.y = (TOP_WALL + 5 * 64) as f32 + 40.0;

                                    // Used for drawing code
                                    self.game.changed_floors = true;
                                }
                            }

                        }

                        GameState::Gameplay => {
                            // Pause Code
                            esc_prev = esc_curr;
                            if keystate.contains(&Keycode::Escape) && esc_prev == false {
                                esc_curr = true;
                                self.menu = GamePaused;
                            }
                            else if keystate.contains(&Keycode::Escape) && esc_prev == true {
                                esc_curr = true;
                            }
                            else {
                                esc_curr = false;
                            }

                            // Debug on/off
                            if keystate.contains(&Keycode::Num1) { self.debug = false; }
                            if keystate.contains(&Keycode::Num2) { self.debug = true; }

                            // Lock doors
                            if keystate.contains(&Keycode::Num3) {
                                self.game.current_room_mut().tiles[5][0].lock();
                                self.game.current_room_mut().tiles[5][16].lock();
                                self.game.current_room_mut().tiles[0][8].lock();
                                self.game.current_room_mut().tiles[10][8].lock();
                            }
                            if keystate.contains(&Keycode::Num4) {
                                self.game.current_room_mut().tiles[5][0].unlock();
                                self.game.current_room_mut().tiles[5][16].unlock();
                                self.game.current_room_mut().tiles[0][8].unlock();
                                self.game.current_room_mut().tiles[10][8].unlock();
                            }


                            // -------------------------------------- GAMEPLAY CODE -------------------------
                            // Movement
                            if keystate.contains(&Keycode::W) { mov_vec.y -= 1.0; }
                            if keystate.contains(&Keycode::S) { mov_vec.y += 1.0; }
                            if keystate.contains(&Keycode::A) { mov_vec.x -= 1.0; }
                            if keystate.contains(&Keycode::D) { mov_vec.x += 1.0; }

                            // Attacks

                            // These stay to update direction even when player can't attack
                            if keystate.contains(&Keycode::Up)    { if !self.game.player.is_charging && !self.game.player.is_attacking {self.game.player.set_dir(Direction::Up);} }
                            if keystate.contains(&Keycode::Down)  { if !self.game.player.is_charging && !self.game.player.is_attacking {self.game.player.set_dir(Direction::Down);}  }
                            if keystate.contains(&Keycode::Left)  { if !self.game.player.is_charging && !self.game.player.is_attacking {self.game.player.set_dir(Direction::Left);}  }
                            if keystate.contains(&Keycode::Right) { if !self.game.player.is_charging && !self.game.player.is_attacking {self.game.player.set_dir(Direction::Right);} }

                            if keystate.contains(&Keycode::Up) && matches!(self.menu, MenuState::GameActive) &&
                                self.game.player.last_attack_time.unwrap().elapsed() >= Duration::from_millis(800) && !self.game.player.is_attacking && !self.game.player.is_charging {
                                self.game.player.set_dir(Direction::Up);
                                if keystate.contains(&Keycode::LShift){
                                    //self.game.player.signal_charge();
                                    self.game.player.signal_attack();
                                }
                                else{
                                    self.game.player.signal_attack();
                                }
                            }
                            if keystate.contains(&Keycode::Down) && matches!(self.menu, MenuState::GameActive) &&
                                self.game.player.last_attack_time.unwrap().elapsed() >= Duration::from_millis(800) && !self.game.player.is_attacking && !self.game.player.is_charging {
                                self.game.player.set_dir(Direction::Down);
                                if keystate.contains(&Keycode::LShift){
                                    //maybe we add charge for up and down?
                                    //self.game.player.signal_charge();
                                    self.game.player.signal_attack();
                                }
                                else{
                                    self.game.player.signal_attack();
                                }
                            }
                            if keystate.contains(&Keycode::Left) && matches!(self.menu, MenuState::GameActive) &&
                                self.game.player.last_attack_time.unwrap().elapsed() >= Duration::from_millis(800) && !self.game.player.is_attacking && !self.game.player.is_charging{
                                self.game.player.set_dir(Direction::Left);
                                if keystate.contains(&Keycode::LShift){
                                    self.game.player.signal_charge();
                                    self.game.player.signal_attack();
                                }
                                else{
                                    self.game.player.signal_attack();
                                }
                            }
                            if keystate.contains(&Keycode::Right) && matches!(self.menu, MenuState::GameActive) &&
                                self.game.player.last_attack_time.unwrap().elapsed() >= Duration::from_millis(800) && !self.game.player.is_attacking && !self.game.player.is_charging {
                                self.game.player.set_dir(Direction::Right);
                                if keystate.contains(&Keycode::LShift){
                                    self.game.player.signal_charge();
                                    self.game.player.signal_attack();
                                }
                                else{
                                    self.game.player.signal_attack();
                                }

                            }



                            // Attack without changing direction
                            if keystate.contains(&Keycode::Space) && matches!(self.menu, MenuState::GameActive) &&
                                self.game.player.last_attack_time.unwrap().elapsed() >= Duration::from_millis(800) && !self.game.player.is_attacking {
                                self.game.player.signal_attack();
                            }

                            if keystate.contains(&Keycode::Q) && matches!(self.menu, MenuState::GameActive) &&
                                self.game.init_time.elapsed() >= Duration::from_secs(1) && self.game.player.has_bomb {
                                self.game.player.use_bomb();
                            }

                            // add enemies to the room
                            if keystate.contains(&Keycode::Z) && matches!(self.menu, MenuState::GameActive) {
                                //let mut enemies = Enemy::new(Vec2::new((LEFT_WALL + 6 * 64) as f32 + 32.0, (TOP_WALL + 7 * 64) as f32 + 40.0), EnemyKind::Speed);
                                //enemies.push(Enemy::new(Vec2::new((LEFT_WALL + 6 * 64) as f32 + 32.0, (TOP_WALL + 7 * 64) as f32 + 40.0), EnemyKind::Speed));
                                // rooms[START_Y as usize][START_X as usize].add_enemies(enemies);
                                //self.game.current_room_mut().additional_enemies(enemies);
                                self.blackboard.set_boss_type();
                            }

                            self.blackboard.update(& self.game);


                            // Move player
                            self.game.player.update_pos(mov_vec);
                            //Update enemy
                            let mut enemy_to_push = Enemy::new(Vec2::new(0.0, 0.0), EnemyKind::Speed);
                            let mut enemy_to_push2 = Enemy::new(Vec2::new(0.0, 0.0), EnemyKind::Speed);
                            let mut push_enemy = false;
                            let mut boss_dead = false;
                            let mut v = vec![];

                            for enemy in self.game.current_room_mut().enemies.iter_mut() {
                                if !enemy.death{
                                    enemy.update(& self.blackboard);
                                    v.push(enemy.clone());
                                    let enemy_walkbox = enemy.box_es.get_walkbox(enemy.pos);
                                    for enemy_walk in v.iter(){
                                        let enemy_walk_walkbox = enemy_walk.box_es.get_walkbox(enemy_walk.pos);
                                        // to stop overlap of enemies (Doesn't look good whatever we tried)
                                        // if (enemy.kind == enemy_walk.kind) && (enemy.pos != enemy_walk.pos) && (enemy_walkbox.has_intersection(enemy_walk_walkbox)){
                                        //     println!("inter_rect");
                                        //     collision::enemy_collision(enemy, &(enemy_walk.current_frame_tile.x), &(enemy_walk.current_frame_tile.y));
                                        //     //enemy.state = State::Idle;
                                        // }
                                        if (enemy_walkbox != enemy_walk_walkbox) && enemy.kind == EnemyKind::Health && enemy_walk.is_healing && enemy_walkbox.has_intersection(enemy_walk_walkbox) {
                                            // if red enemy is intersection with another enemy that is 'healing'
                                            enemy.is_healing = true;
                                            enemy.state = State::Heal;
                                        }
                                    }


                                    if enemy.kind == EnemyKind::Final && enemy.is_attacking{ //Final Boss Check
                                        //println!("Before pops");

                                        match enemy.final_enemies_to_spawn.pop() {
                                            Some( new_enemy ) => enemy_to_push = new_enemy,
                                            None => {},
                                        };

                                        //println!("First pop");

                                        match enemy.final_enemies_to_spawn.pop() {
                                            Some( new_enemy ) => enemy_to_push2 = new_enemy,
                                            None => {},
                                        };

                                        //enemy_to_push2 = enemy.final_enemies_to_spawn.pop().unwrap();
                                        //println!("Second pop");
                                        push_enemy = true;
                                        //self.game.current_room_mut().additional_enemies(enemy.final_enemies_to_spawn.pop().unwrap());
                                        enemy.is_attacking = false;
                                    }
                                }
                                if enemy.death && enemy.kind == EnemyKind::Final{
                                    boss_dead = true;
                                }
                            }
                            //FINAL BOSS ONLY
                            if push_enemy {
                                self.game.current_room_mut().additional_enemies(enemy_to_push);
                                self.game.current_room_mut().additional_enemies(enemy_to_push2);
                            }
                            if boss_dead {
                                self.game.changed_floors = false;
                                self.game.transition_start = Instant::now();
                                self.game.game_state = GameState::BetweenFloors;
                            }

                            // Apply collision
                            self.collide();
                            // // debugging healing and damage to a PLAYER
                            // if keystate.contains(&Keycode::H) { self.game.player.heal(2);
                            //     println!("Health is: {}", self.game.player.health());
                            // }  // heal
                            // if keystate.contains(&Keycode::H) {
                            //    self.game.player.plus_power_health();
                            //    println!("PowerupHealth is {}", self.game.player.power_up_vec[0]);
                            // }  // powerup
                            // if keystate.contains(&Keycode::B) { self.game.player.damage(1);
                            //     println!("Health is: {}", self.game.player.health());
                            // }  //damage

                            // Set prev frame tile
                            self.game.player.prev_frame_tile = self.game.player.current_frame_tile;
                            // Update current fream tile
                            self.game.player.current_frame_tile = Vec2::new(
                                (self.game.player.get_pos_x() - LEFT_WALL) / TILE_WIDTH,
                                (self.game.player.get_pos_y() - TOP_WALL) / TILE_WIDTH
                            );
                            //println!("{}, {}", self.game.player.current_frame_tile.x, self.game.player.current_frame_tile.y);

                            self.walkover();

                        }


                    }
                    // --------------------------------- GAMEPLAY CODE END -------------------------
                }

                GameOver => {
                    if keystate.contains(&Keycode::Space) {
                        self.game = Game::new();
                        self.menu = GameActive;
                    }
                }

                GamePaused => {
                    // Unpause Code
                    esc_prev = esc_curr;
                    if keystate.contains(&Keycode::Escape) && esc_prev == false {
                        esc_curr = true;
                        self.menu = GameActive;
                    }
                    else if keystate.contains(&Keycode::Escape) && esc_prev == true {
                        esc_curr = true;
                    }
                    else {
                        esc_curr = false;
                    }

                    // MM
                    if keystate.contains(&Keycode::X) { self.menu = MainMenu }
                }

            }

            // Draw game state
            self.draw()?;
        }


        // Out of game loop, return Ok
        Ok(())
    }
}

impl Manager {
    // fn draw_init(& mut self) {

    // }

    fn collide(& mut self) {
        collision::base(&mut self.game, &mut self.core, &mut self.menu, & self.blackboard);
    }

    fn walkover(& mut self) {
        walkover::base(&mut self.game, &mut self.menu);
    }

    // fn draw_enemies<'r>(&mut self, textures: Vec<Texture>) -> Result<(), String> {


    //     Ok(())
    // }

    // Draw entire game state on screen.
    fn draw(& mut self) -> Result<(), String> {
        draw::base(&mut self.game, &mut self.core, &mut self.menu, & self.debug, & self.blackboard)
    }
}
