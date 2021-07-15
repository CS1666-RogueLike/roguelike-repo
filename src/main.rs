extern crate roguelike;

mod menu;
use crate::menu::*;

mod game;
use crate::game::*;

mod player;
mod entity;

mod util;
use crate::util::*;

mod collision;
mod walkover;
mod draw;

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
                        println!("Switch");
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
        collision::base(&mut self.game, &mut self.core, &mut self.menu);
    }

    fn walkover(& mut self) {
        walkover::base(&mut self.game, &mut self.menu);
    }

    // fn draw_enemies<'r>(&mut self, textures: Vec<Texture>) -> Result<(), String> {


    //     Ok(())
    // }

    // Draw entire game state on screen.
    fn draw(& mut self) -> Result<(), String> {
        draw::base(&mut self.game, &mut self.core, &mut self.menu, & self.debug)
    }
}