extern crate roguelike;

mod menu;
use crate::menu::*;

mod game;
use crate::game::*;

mod player;
mod entity;

mod util;
use crate::util::*;

mod map;
mod floor;
mod room;
mod tile;
use crate::tile::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use roguelike::SDLCore;
use roguelike::Demo;
use entity::Health;
use player::PowerUp;
use entity::EnemyKind;

//use std::cmp::min;
use std::collections::HashSet;

use std::time::Duration;
use crate::menu::MenuState::GameOver;

// TODO: Move all sdl code to a separate file, keep the main.rs file simple

// Constants to clean up SDLCore initiation
const TITLE: &str = "Roguelike";
const VSYNC: bool = true;
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

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
}

impl Demo for Manager {
    // Initialize manager struct.
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, VSYNC, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let debug = false;
        let menu = MenuState::MainMenu;
        let game = Game::new();

        Ok(Manager{core, debug, menu, game})
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
        println!("\tH\t\tTest health power up");
        println!("\tJ\t\tTest speed power up");
        println!("\tK\t\tTest attack power up");
        println!("");

        println!("Health is: {}", self.game.player.health());
        println!("Max Health is: {}", self.game.player.max_hp());

        // Hacky solution for pause menu
        let mut esc_prev = false;
        let mut esc_curr = false;



        //println!("DOES THE ROOM EXIST? {}", self.game.current_room().exists);

        'gameloop: loop {

            // Check for press of close window button.
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'gameloop,
                    Event::KeyUp {keycode: Some(Keycode::H), repeat: false, ..} =>
                        { self.game.player.plus_power_health();
                            println!("PowerupHealth is {}", self.game.player.power_up_vec[0]);
                            println!("Max Health is: {}", self.game.player.max_hp());
                        },
                    Event::KeyUp {keycode: Some(Keycode::J), repeat: false, ..} =>
                        { self.game.player.plus_power_speed();
                            println!("PowerupSpeed is {}", self.game.player.power_up_vec[1]);
                        },
                    Event::KeyUp {keycode: Some(Keycode::K), repeat: false, ..} =>
                        { self.game.player.plus_power_attack();
                            println!("PowerupAttack is {}", self.game.player.power_up_vec[2]);
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

                MainMenu => {
                    if keystate.contains(&Keycode::Space) {
                        self.menu = GameActive;
                        self.game = Game::new(); // Initialize a new game
                        // This makes it so exiting to menu and staring the game again works
                        // properly
                    }
                }

                GameActive => {

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
                    // Direction (will eventually be attacks)
                    // TODO: FIX SO THAT NEW KEY OVERRIDES OLD ONE INSTEAD OF HAVING SET PRIORITY
                    if keystate.contains(&Keycode::Up)    { self.game.player.set_dir(Direction::Up);    }
                    if keystate.contains(&Keycode::Down)  { self.game.player.set_dir(Direction::Down);  }
                    if keystate.contains(&Keycode::Left)  { self.game.player.set_dir(Direction::Left);  }
                    if keystate.contains(&Keycode::Right) { self.game.player.set_dir(Direction::Right); }
                    if keystate.contains(&Keycode::Space) && matches!(self.menu, MenuState::GameActive) &&
                    self.game.init_time.elapsed() >= Duration::from_secs(1) && !self.game.player.is_attacking {
                        self.game.player.signal_attack();
                    }

                    // Move player
                    self.game.player.update_pos(mov_vec);

                    for enemy in self.game.current_room_mut().enemies.iter_mut() {
                        enemy.update_pos();
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
                    self.game.player.current_frame_tile = Vec2::new((self.game.player.get_pos_x() - LEFT_WALL) / 64,
                                                                    (self.game.player.get_pos_y() - TOP_WALL) / 64);
                    //println!("{}, {}", self.game.player.current_frame_tile.x, self.game.player.current_frame_tile.y);

                    self.walkover();

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

        // Outermost wall collision
        self.game.player.pos.x = self.game.player.pos.x.clamp(LEFT_WALL as f32 + (self.game.player.walkbox.x/2) as f32, RIGHT_WALL as f32 - (self.game.player.walkbox.x/2) as f32);
        self.game.player.pos.y = self.game.player.pos.y.clamp(TOP_WALL as f32 + (self.game.player.walkbox.y/2) as f32, BOT_WALL as f32 - (self.game.player.walkbox.y/2) as f32);

        // TODO: Goal is to generalize hitbox data into a trait so that we can condense logic

        // Maintain enemy bounds for the room and check player collisions
        let mut enemy_list = self.game.current_room().enemies.clone();
                    
        for enemy in enemy_list.iter_mut() {
            enemy.pos.x = enemy.pos.x.clamp(LEFT_WALL as f32 + (enemy.walkbox.x * 4) as f32, RIGHT_WALL as f32 - (enemy.walkbox.x * 4) as f32);
            enemy.pos.y = enemy.pos.y.clamp(TOP_WALL as f32 + (enemy.walkbox.y * 4) as f32, BOT_WALL as f32 - (enemy.walkbox.y * 4) as f32);

            // If the test enemy is in the current room of the player...
            if !enemy.death() {
                // If the test enemy's walkbox intersects with the player walkbox...
                let wb_test = enemy.get_walkbox_world();
                let player_test = self.game.player.get_walkbox_world();

                // Attempt at collision with attackbox
                if self.game.player.is_attacking {
                    let player_attack = self.game.player.get_attackbox_world();
                    if wb_test.has_intersection(player_attack) {
                        println!("Attack collided with enemy!");
                        enemy.damage(self.game.player.attack);
                        println!("damage done was {}", self.game.player.attack);

                        //Absorb Enemy
                        if enemy.power == true {
                            // Place gem on enemy's current tile.
                            // TODO: Factor in walkability for tile that the gem drops on.
                            self.game.current_room_mut()
                                     .tile_at(enemy.get_pos_x(), enemy.get_pos_y())
                                     .place_gem(match enemy.kind {
                                        EnemyKind::Health => Gem::Red,
                                        EnemyKind::Speed => Gem::Blue,
                                        EnemyKind::Attack => Gem::Yellow,
                                     });

                            enemy.power = false;
                        }
                    }
                }

                // Then there's a collision!
                if wb_test.has_intersection(player_test) {
                    //Damage enemy also! For some reason
                    //println!("Collision");
                    //enemy.damage(1);
                    // Check to see when the player was attacked last...
                    match self.game.player.last_invincibility_time {
                        // If there is an old invincibility time for the player,
                        // see if the "invincibility window" has elapsed since then...
                        Some( time ) => {
                            if time.elapsed() >= Duration::from_millis(1750) {
                                // If so, update the invincibility time and take damage to the player.
                                self.game.player.update_invincibility_time();
                                self.game.player.damage(1);
                            }
                        },
                        None => {
                            // Otherwise, take damage as there was
                            // no previous "invincibility window" to account for
                            self.game.player.update_invincibility_time();
                            self.game.player.damage(1);
                        }
                    }

                    // If the player is dead, update to the game over menu state
                    if self.game.player.death() {
                        self.menu = MenuState::GameOver;
                    }
                }
            }
        }
            
        self.game.current_room_mut().enemies = enemy_list;

        self.core.wincan.set_draw_color(Color::RGBA(128, 0, 0, 255));
        let mut x = 0;
        let mut y = 0;
        use tile::Walkability::*;
        // This can't be done with the current room function bc it returns a reference which messes up internal stuff
        for row in &self.game.map.floors[self.game.cf].rooms[self.game.cr.y as usize][self.game.cr.x as usize].tiles {
            for t in row {
                match t.walkability() {
                    Wall | Rock | Pit => {
                        // Hacky af block collision that needs to be changed later
                        let opt = self.game.player.get_walkbox_world().intersection(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64));

                        // increment x
                        // if we do this later it messes thing up due to the continue statement in
                        // the unboxing
                        x += 1;

                        let inter_rect = match opt {
                            Some(x) => x,
                            None => continue, // If no intersection just leave function, we're done
                        };
                        let mut x_offset = inter_rect.width() as i32;
                        let mut y_offset = inter_rect.height() as i32;

                        if self.game.player.pos.x < inter_rect.x() as f32 {
                            // TO THE LEFT OF ROCK
                            y_offset = 0;
                        }
                        if self.game.player.pos.x > (inter_rect.x() + inter_rect.width() as i32) as f32 {
                            // TO THE RIGHT OF ROCK
                            x_offset *= -1;
                            y_offset = 0;
                        }
                        if self.game.player.pos.y < inter_rect.y() as f32 {
                            // ABOVE ROCK
                            x_offset = 0;
                        }
                        if self.game.player.pos.y > (inter_rect.y() + inter_rect.height() as i32) as f32 {
                            // BELOW ROCK
                            x_offset = 0;
                            y_offset *= -1;
                        }

                        self.game.player.pos.x -= x_offset as f32;
                        self.game.player.pos.y -= y_offset as f32;
                    }

                    _ => x += 1,
                }
            }

            // Prepare for next iteration of loop
            y += 1;
            x = 0;
        }
    }

    fn walkover(& mut self) {
        // Branch for tiles that should only be called once (doors, pickups
        if self.game.player.current_frame_tile != self.game.player.prev_frame_tile {
            //TODO: Find a way to make these chain calls less crazy

            // Set new room to visited
            // This is done to the now previous room to avoid having to do special logic on the first room
            self.game.map.floors[self.game.cf].rooms[self.game.cr.y as usize][self.game.cr.x as usize].visited = true;

            match self.game.map.floors[self.game.cf].rooms[self.game.cr.y as usize][self.game.cr.x as usize].tiles[self.game.player.current_frame_tile.y as usize][self.game.player.current_frame_tile.x as usize].on_walkover() {
                WalkoverAction::DoNothing => {self.game.player.speed_adjust(WalkoverAction::DoNothing);},
                WalkoverAction::ChangeRooms => {
                    //println!("Door tile walked over.");
                    if self.game.player.current_frame_tile.x == 0 { // LEFT DOOR
                        // Current room one to the right
                        self.game.cr.x -= 1;
                        // Move player position to just outside of right door in new room
                        self.game.player.pos = Vec2::new((LEFT_WALL + 15 * 64) as f32 + 63.0, (TOP_WALL + 5 * 64) as f32 + 40.0);
                    }
                    if self.game.player.current_frame_tile.x == 16 { // RIGHT DOOR
                        // Current room one to the right
                        self.game.cr.x += 1;
                        // Move player position to just outside of left door in new room
                        self.game.player.pos = Vec2::new((LEFT_WALL + 1 * 64) as f32 + 1.0, (TOP_WALL + 5 * 64) as f32 + 40.0);
                    }
                    if self.game.player.current_frame_tile.y == 0 { // TOP DOOR
                        // Current room one up
                        self.game.cr.y -= 1;
                        // Move player position to just outside of bottom door in new room
                        self.game.player.pos = Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 9 * 64) as f32 + 50.0);
                    }
                    if self.game.player.current_frame_tile.y == 10 { // BOTTOM DOOR
                        // Current room one down
                        self.game.cr.y += 1;
                        // Move player position to just outside of bottom door in new room
                        self.game.player.pos = Vec2::new((LEFT_WALL + 8 * 64) as f32 + 32.0, (TOP_WALL + 1 * 64) as f32 + 10.0);
                    }

                },

                // Gem pickups
                WalkoverAction::BuffDamage => { self.game.player.plus_power_attack(); }
                WalkoverAction::BuffHealth => { self.game.player.plus_power_health(); }
                WalkoverAction::BuffSpeed => { self.game.player.plus_power_speed(); }

                WalkoverAction::GivePlayerKey => {
                    println!("Key has been picked up!!!");
                    self.game.player.has_key = true;
                },

                WalkoverAction::Damage => {
                    println!("You've stepped on spikes!");
                    self.game.player.damage(1);
                    self.game.player.speed_adjust(WalkoverAction::Damage);
                    if self.game.player.death() {
                        self.menu = MenuState::GameOver;
                    }
                },

                WalkoverAction::GoToNextFloor => {
                    if self.game.player.has_key {
                        println!("Congratulations! You made it to the next floor!!!");
                        self.game.map.floors[self.game.cf]
                                     .rooms[self.game.cr.y as usize][self.game.cr.x as usize]
                                     .tiles[self.game.player.current_frame_tile.y as usize][self.game.player.current_frame_tile.x as usize]
                                     .unlock();

                        self.game.player.has_key = false;
                        //Debug: println!("{}", self.game.cf);
                        // Temp Check for game over
                        if self.game.cf == 1 {
                            self.menu = GameOver;
                        }
                        else {
                            self.game.cf += 1;// this should stay
                        }
                        //Debug: println!("{}", self.game.cf);
                        // THIS WILL NEED CHANGING
                        self.game.cr.x = 3;
                        self.game.cr.y = 4;
                        self.game.player.pos.x = (LEFT_WALL + 8 * 64) as f32 + 32.0;
                        self.game.player.pos.y = (TOP_WALL + 5 * 64) as f32 + 40.0;

                    }
                    else {
                        println!("You need a key to unlock this door!");
                    }

                }

            }
        }
        // TODO: else branch for continuous tiles (spike tile)
    }

    // fn draw_enemies<'r>(&mut self, textures: Vec<Texture>) -> Result<(), String> {


    //     Ok(())
    // }

    // Draw entire game state on screen.
    fn draw(& mut self) -> Result<(), String> {

        // MOVE SOMEWHERE ELSE, TEXTURES SHOULD ONLY BE INITIALIZED ONCE
        let texture_creator = self.core.wincan.texture_creator();

        // Scope enums for readability
        use menu::MenuState::*;

        // Determine what to draw depending on state of the menu.
        match self.menu {

            MainMenu => {
                let main_menu = texture_creator.load_texture("assets/main_menu.png")?;
                self.core.wincan.copy(&main_menu, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
            }

            GameActive => {
                // Load textures
                let bg = texture_creator.load_texture("assets/test_image.png")?;

                let slime_up = texture_creator.load_texture("assets/slime_up.png")?;
                let slime_down = texture_creator.load_texture("assets/slime_down.png")?;
                let slime_left = texture_creator.load_texture("assets/slime_left.png")?;
                let slime_right = texture_creator.load_texture("assets/slime_right.png")?;

                let speed_idle = texture_creator.load_texture("assets/speed_idle.png")?;
                let attack_idle = texture_creator.load_texture("assets/wizard_attack_enemy.png")?;
                let health_idle = texture_creator.load_texture("assets/health-sprite-down.png")?;

                let hp_indicator = texture_creator.load_texture("assets/hp.png")?;

                //power assets
                let p_text = texture_creator.load_texture("assets/p_text.png")?;
                let p_text_health = texture_creator.load_texture("assets/p_text_health.png")?;
                let p_text_speed = texture_creator.load_texture("assets/p_text_speed.png")?;
                let p_text_attack = texture_creator.load_texture("assets/p_text_attack.png")?;
                //let p_empty = texture_creator.load_texture("assets/p_empty.png")?;
                let p_background = texture_creator.load_texture("assets/p_background.png")?;
                let p_blue_1 = texture_creator.load_texture("assets/p_blue_1.png")?;
                let p_blue_2 = texture_creator.load_texture("assets/p_blue_2.png")?;
                let p_blue_3 = texture_creator.load_texture("assets/p_blue_3.png")?;
                let p_red_1 = texture_creator.load_texture("assets/p_red_1.png")?;
                let p_red_2 = texture_creator.load_texture("assets/p_red_2.png")?;
                let p_red_3 = texture_creator.load_texture("assets/p_red_3.png")?;
                let p_yellow_1 = texture_creator.load_texture("assets/p_yellow_1.png")?;
                let p_yellow_2 = texture_creator.load_texture("assets/p_yellow_2.png")?;
                let p_yellow_3 = texture_creator.load_texture("assets/p_yellow_3.png")?;


                let gem_red = texture_creator.load_texture("assets/gem_red.png")?;
                let gem_yellow = texture_creator.load_texture("assets/gem_yellow.png")?;
                let gem_blue = texture_creator.load_texture("assets/gem_blue.png")?;

                let bricks = texture_creator.load_texture("assets/ground_tile.png")?;
                let rock = texture_creator.load_texture("assets/rock.png")?;
                let spike = texture_creator.load_texture("assets/spike.png")?;

                let key = texture_creator.load_texture("assets/key.png")?;
                let td_locked = texture_creator.load_texture("assets/trapdoor_locked.png")?;

                let pl_heart = texture_creator.load_texture("assets/playerheart16x16.png")?;

                // Draw black screen
                self.core.wincan.set_draw_color(Color::BLACK);
                self.core.wincan.clear();

                // Draw background of game screen
                self.core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;

                let mut x = 0;
                let mut y = 0;
                for row in &self.game.current_room().tiles {
                    for t in row {
                        match t.sprite() {
                            SpriteID::Ground => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            // GEMS
                            SpriteID::GemRed => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                self.core.wincan.copy(&gem_red, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }
                            SpriteID::GemBlue => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                self.core.wincan.copy(&gem_blue, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }
                            SpriteID::GemYellow => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                self.core.wincan.copy(&gem_yellow, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            // Do nothing, we already drew the surrounding walls as one image.
                            SpriteID::Wall => (),

                            SpriteID::Rock => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                self.core.wincan.copy(&rock, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                //self.core.wincan.copy(&gem_yellow, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            SpriteID::Pit => {
                                //self.core.wincan.set_draw_color(Color::RGBA(255, 255, 0, 255));
                                //self.core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64));
                            }

                            SpriteID::DoorLocked => {
                                self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                                self.core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }
                            SpriteID::DoorUnlocked => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                //self.core.wincan.set_draw_color(Color::RGBA(0, 255, 0, 255));
                                //self.core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64));
                            }

                            SpriteID::Key => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                self.core.wincan.copy(&key, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            SpriteID::TrapdoorLocked => {
                                self.core.wincan.copy(&td_locked, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            SpriteID::TrapdoorUnlocked => {
                                self.core.wincan.set_draw_color(Color::RGBA(255, 128, 128, 255));
                                self.core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }

                            SpriteID::Spike => {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                self.core.wincan.copy(&spike, None, Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                            }
                        }
                        x += 1;
                    }
                    y += 1;
                    x = 0;
                }

                match self.game.player.get_dir() {
                    Direction::Up => {
                        self.core.wincan.copy(&slime_up, None,
                            Rect::new(
                                self.game.player.get_pos_x() - 35 + 4,
                                self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                64, 64)
                            )?;
                    }
                    Direction::Down => {
                        self.core.wincan.copy(&slime_down, None,
                            Rect::new(
                                self.game.player.get_pos_x() - 35,
                                self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                64, 64)
                            )?;
                    }
                    Direction::Left => {
                        self.core.wincan.copy(&slime_left, None,
                            Rect::new(
                                self.game.player.get_pos_x() - 35 + 4,
                                self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                64, 64)
                            )?;
                    }
                    Direction::Right => {
                        self.core.wincan.copy(&slime_right, None,
                            Rect::new(
                                self.game.player.get_pos_x() - 35,
                                self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                64, 64)
                            )?;
                    }
                }

                //self.draw_enemies(textures);

                let enemies = &mut self.game.current_room_mut().enemies;
                for enemy in enemies.iter_mut()  {
                    if !enemy.death() {
                        let tex = match &enemy.kind {
                            EnemyKind::Attack => &attack_idle,
                            EnemyKind::Health => &health_idle,
                            EnemyKind::Speed => &speed_idle
                        };
        
                        self.core.wincan.copy(&tex, None,
                            Rect::new(
                                enemy.get_pos_x() - 35 + 4,
                                enemy.get_pos_y() - 64 + (enemy.get_walkbox().height()/2) as i32,
                                64, 64)
                        )?;
                    }
                }

                // If the player was attacked, show a quick damage indicator ("-1" in red)
                if self.game.player.was_attacked() {
                    self.core.wincan.copy(&hp_indicator, None, Rect::new(self.game.player.get_pos_x() as i32, self.game.player.get_pos_y() as i32, 64, 64))?;
                }

                for i in 0 .. self.game.player.hp {
                    self.core.wincan.copy(&pl_heart, None, Rect::new(1 + (i * 63), 40, 64, 64))?;
                }

                //draw powerup dials
                self.core.wincan.copy(&p_text, None, Rect::new(80,468,64,64))?;
                self.core.wincan.copy(&p_text_health, None, Rect::new(0,532,64,64))?;
                self.core.wincan.copy(&p_text_speed, None, Rect::new(0, 596,64,64))?;
                self.core.wincan.copy(&p_text_attack, None, Rect::new(0,660,64,64))?;
                self.core.wincan.copy(&p_background, None, Rect::new(80,532,64,64))?;
                self.core.wincan.copy(&p_background, None, Rect::new(80,596,64,64))?;
                self.core.wincan.copy(&p_background, None, Rect::new(80,660,64,64))?;

                if self.game.player.power_image_health() == 1 {
                    self.core.wincan.copy(&p_red_1, None, Rect::new(80,532,64,64))?;
                }
                else if self.game.player.power_image_health() == 2 {
                    self.core.wincan.copy(&p_red_2, None, Rect::new(80,532,64,64))?;
                }
                else if self.game.player.power_image_health() == 3 {
                    self.core.wincan.copy(&p_red_3, None, Rect::new(80,532,64,64))?;
                }
                if self.game.player.power_image_speed() == 1 {
                    self.core.wincan.copy(&p_blue_1, None, Rect::new(80,596,64,64))?;
                }
                else if self.game.player.power_image_speed() == 2 {
                    self.core.wincan.copy(&p_blue_2, None, Rect::new(80,596,64,64))?;
                }
                else if self.game.player.power_image_speed() == 3 {
                    self.core.wincan.copy(&p_blue_3, None, Rect::new(80,596,64,64))?;
                }
                if self.game.player.power_image_attack() == 1 {
                    self.core.wincan.copy(&p_yellow_1, None, Rect::new(80,660,64,64))?;
                }
                else if self.game.player.power_image_attack() == 2 {
                    self.core.wincan.copy(&p_yellow_2, None, Rect::new(80,660,64,64))?;
                }
                else if self.game.player.power_image_attack() == 3 {
                    self.core.wincan.copy(&p_yellow_3, None, Rect::new(80,660,64,64))?;
                }

                // ------------------------ DRAW UI --------------------------

                // Rough key setup
                if self.game.player.has_key {
                    self.core.wincan.copy(&key, None, Rect::new(64, 200, 64, 64))?;
                }

                // Minimap (commented out first block as the block below does the same thing)
                // for x in 0..8 {
                //     for y in 0..8 {
                //         // Current room
                //         if x == self.game.cr.x && y == self.game.cr.y {
                //             self.core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
                //             self.core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                //         }
                //         // Visited rooms
                //         else if self.game.map.floors[self.game.cf].rooms[y as usize][x as usize].visited == true {
                //             self.core.wincan.set_draw_color(Color::RGBA(80, 80, 80, 255));
                //             self.core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                //         }
                //         // Unvisited rooms
                //         else if self.game.map.floors[self.game.cf].rooms[y as usize][x as usize].visited == false &&
                //             self.game.map.floors[self.game.cf].rooms[y as usize][x as usize].exists == true {
                //             self.core.wincan.set_draw_color(Color::RGBA(30, 30, 30, 255));
                //             self.core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;

                //         }
                //         // Black border for separation
                //         self.core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                //         self.core.wincan.draw_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                //     }
                // }

                // Minimap
                for x in 0..8 {
                    for y in 0..8 {
                        // Current room
                        if x == self.game.cr.x && y == self.game.cr.y {
                            self.core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
                            self.core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                        }
                        // Visited rooms
                        else if self.game.map.floors[self.game.cf].rooms[y as usize][x as usize].visited == true {
                            self.core.wincan.set_draw_color(Color::RGBA(80, 80, 80, 255));
                            self.core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                        }
                        // Unvisited rooms
                        else if self.game.map.floors[self.game.cf].rooms[y as usize][x as usize].visited == false &&
                            self.game.map.floors[self.game.cf].rooms[y as usize][x as usize].exists == true {
                            self.core.wincan.set_draw_color(Color::RGBA(30, 30, 30, 255));
                            self.core.wincan.fill_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;

                        }
                        // Black border for separation
                        self.core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                        self.core.wincan.draw_rect(Rect::new(22 + x * 20, 300 + y * 14, 20, 14))?;
                    }
                }

                if self.debug {
                    // Draw player collision hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    self.core.wincan.draw_rect(self.game.player.get_walkbox_world())?;

                    let enemies = &mut self.game.current_room_mut().enemies;
                  
                    for enemy in enemies.iter_mut() {
                        if !enemy.death() {
                            self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                            self.core.wincan.draw_rect(enemy.get_walkbox_world())?;
        
                            self.core.wincan.set_draw_color(Color::RGBA(128,128,255,255));
                            self.core.wincan.draw_rect(
                                Rect::new(
                                    enemy.get_pos_x() - (enemy.get_hitbox_x()/2) as i32,
                                    enemy.get_pos_y() - (enemy.get_hitbox_y()) as i32,
                                    enemy.get_hitbox_x(),
                                    enemy.get_hitbox_y()
                                )
                            )?;
                        }
                    }

                    // Draw player damage hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(128, 128, 255, 255));
                    self.core.wincan.draw_rect(Rect::new(self.game.player.get_pos_x() - (self.game.player.get_hitbox_x()/2) as i32,
                                                        self.game.player.get_pos_y() - (self.game.player.get_hitbox_y()) as i32 + (self.game.player.get_walkbox().height()/2) as i32,
                                                        self.game.player.get_hitbox_x(),
                                                        self.game.player.get_hitbox_y())
                                                )?;


                    // Draw null at center of player hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 255, 255));
                    self.core.wincan.draw_line(
                        Point::new(self.game.player.get_pos_x() + 4, self.game.player.get_pos_y()),
                        Point::new(self.game.player.get_pos_x() - 4, self.game.player.get_pos_y()),
                    )?;
                    self.core.wincan.draw_line(
                        Point::new(self.game.player.get_pos_x(), self.game.player.get_pos_y() + 4),
                        Point::new(self.game.player.get_pos_x(), self.game.player.get_pos_y() - 4),
                    )?;

                    // Draw collision hitboxes
                    use tile::Walkability::*;
                    self.core.wincan.set_draw_color(Color::RGBA(128, 0, 0, 255));
                    x = 0;
                    y = 0;
                    for row in &self.game.current_room().tiles {
                        for t in row {
                            match t.walkability() {

                                Wall | Rock | Pit => {
                                    self.core.wincan.draw_rect(Rect::new(LEFT_WALL + x * 64, TOP_WALL + y * 64, 64, 64))?;
                                }

                                _ => (),
                                // Dont draw anything for other tiles
                            }
                            x += 1;
                        }
                        y += 1;
                        x = 0;
                    }

                    // Draw a box over the current tile
                    self.core.wincan.set_draw_color(Color::RGBA(255, 255, 0, 255));
                    if self.game.player.current_frame_tile != self.game.player.prev_frame_tile {
                        self.core.wincan.fill_rect(Rect::new((self.game.player.get_pos_x() - LEFT_WALL) / 64 * 64 + LEFT_WALL,
                                                            (self.game.player.get_pos_y() - TOP_WALL) / 64 * 64 + TOP_WALL,
                                                            64,
                                                            65,
                        ))?;

                    }
                    else {
                        self.core.wincan.draw_rect(Rect::new((self.game.player.get_pos_x() - LEFT_WALL) / 64 * 64 + LEFT_WALL,
                                                            (self.game.player.get_pos_y() - TOP_WALL) / 64 * 64 + TOP_WALL,
                                                            64,
                                                            65,
                        ))?;
                    }
                }

                // Draw attackbox
                self.core.wincan.set_draw_color(Color::RGBA(139, 195, 74, 255));
                if self.game.player.recently_attacked() {
                    self.core.wincan.fill_rect(self.game.player.get_attackbox_world())?;
                }

            }

            GameOver => {
                let gameover = texture_creator.load_texture("assets/game_over.png")?;
                self.core.wincan.copy(&gameover, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
            }

            GamePaused => {
                self.core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                self.core.wincan.clear();

                let pause_menu = texture_creator.load_texture("assets/pause_menu.png")?;
                self.core.wincan.copy(&pause_menu, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;
            }

        }

        // Tell SDL to draw everything on screen.
        self.core.wincan.present();

        Ok(())
    }
}
