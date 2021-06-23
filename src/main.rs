extern crate roguelike;

mod menu;
use crate::menu::*;

mod game;
use crate::game::*;

mod player;

mod util;
use crate::util::*;

mod tile;

mod map;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use roguelike::SDLCore;
use roguelike::Demo;

use std::cmp::min;
use std::collections::HashSet;

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
    menu: MenuState, // Enum that controls the control flow via the menu.
    game: Game, // Struct holding all game related data.
}

impl Demo for Manager {
    // Initialize manager struct.
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, VSYNC, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let menu = MenuState::MainMenu;
        let game = Game::new();
        Ok(Manager{core, menu, game})
    }

    fn run(&mut self) -> Result<(), String> {
        
        // Print controls to terminal.
        println!("");
        println!("");
        println!(" --------------- CONTROLS --------------- ");
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
        println!("\tEscape\tPause game (while in game, not menus)");
        println!("");

        // Hacky solution for pause menu
        let mut esc_prev = false;
        let mut esc_curr = false;

        'gameloop: loop {

            // Check for press of close window button.
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'gameloop,
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

                    // Player position
                    self.game.player.update_pos(mov_vec);
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
            self.draw();
        }


        // Out of game loop, return Ok
        Ok(())
    }
}

impl Manager {
    fn draw(& mut self) -> Result<(), String> {
            use menu::MenuState::*;
            let texture_creator = self.core.wincan.texture_creator();
            match self.menu {


                MainMenu => {
                    let main_menu = texture_creator.load_texture("assets/main_menu.png")?;
                    self.core.wincan.copy(&main_menu, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT));
}

                GameActive => {
                    // Load textures
                    let bg = texture_creator.load_texture("assets/test_image.png")?;
                    let slime = texture_creator.load_texture("assets/slime_sprite.png")?;

                    let slime_up = texture_creator.load_texture("assets/slime_up.png")?;
                    let slime_down = texture_creator.load_texture("assets/slime_down.png")?;
                    let slime_left = texture_creator.load_texture("assets/slime_left.png")?;
                    let slime_right = texture_creator.load_texture("assets/slime_right.png")?;

                    let bricks = texture_creator.load_texture("assets/ground_tile.png")?;
                    let rock = texture_creator.load_texture("assets/rock.png")?;

                    // Draw black screen
                    self.core.wincan.set_draw_color(Color::BLACK);
                    self.core.wincan.clear();

                    self.core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT));
        
                    for x in 0..15 {
                        for y in 0..9 {
                            if x == 7 && y == 4 {
                                self.core.wincan.copy(&rock, None, Rect::new(LEFT_WALL + x as i32 * 64, TOP_WALL + y as i32 * 64, 64, 64));
                            }
                            else {
                                self.core.wincan.copy(&bricks, None, Rect::new(LEFT_WALL + x as i32 * 64, TOP_WALL + y as i32 * 64, 64, 64));
                            }
                        }
                    }

                    match self.game.player.get_dir() {
                        Direction::Up => {
                            self.core.wincan.copy(&slime_up, None,
                                Rect::new( 
                                    self.game.player.get_pos_x() - 35 + 4,
                                    self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                    64, 64)
                                );
                        }
                        Direction::Down => {
                            self.core.wincan.copy(&slime_down, None,
                                Rect::new( 
                                    self.game.player.get_pos_x() - 35,
                                    self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                    64, 64)
                                );
                        }
                        Direction::Left => {
                            self.core.wincan.copy(&slime_left, None,
                                Rect::new( 
                                    self.game.player.get_pos_x() - 35 + 4,
                                    self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                    64, 64)
                                );
                        }
                        Direction::Right => {
                            self.core.wincan.copy(&slime_right, None,
                                Rect::new( 
                                    self.game.player.get_pos_x() - 35,
                                    self.game.player.get_pos_y() - 64 + (self.game.player.get_walkbox().height()/2) as i32,
                                    64, 64)
                                );
                        }

                    }



                    // CHANGE THIS VALUE TO STOP DRAWING DEBUG STUFF
                    let debug = true;



                    if debug {
                    // Draw player collision hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    self.core.wincan.draw_rect(self.game.player.get_walkbox_world());
                    /*
                    self.core.wincan.draw_rect(Rect::new(self.game.player.get_pos_x() - (self.game.player.get_walkbox_x()/2) as i32,
                                                        self.game.player.get_pos_y() - (self.game.player.get_walkbox_y()/2) as i32,
                                                        self.game.player.get_walkbox_x(),
                                                        self.game.player.get_walkbox_y())
                                               );
                                               */


                    // Draw player damage hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(128, 128, 255, 255));
                    self.core.wincan.draw_rect(Rect::new(self.game.player.get_pos_x() - (self.game.player.get_hitbox_x()/2) as i32,
                                                        self.game.player.get_pos_y() - (self.game.player.get_hitbox_y()) as i32 + (self.game.player.get_walkbox().height()/2) as i32,
                                                        self.game.player.get_hitbox_x(),
                                                        self.game.player.get_hitbox_y())
                                               );
        
                    // Draw null at center of player hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 255, 255));
                    self.core.wincan.draw_line(
                        Point::new(self.game.player.get_pos_x() + 4, self.game.player.get_pos_y()),
                        Point::new(self.game.player.get_pos_x() - 4, self.game.player.get_pos_y()),
                    );
                    self.core.wincan.draw_line(
                        Point::new(self.game.player.get_pos_x(), self.game.player.get_pos_y() + 4),
                        Point::new(self.game.player.get_pos_x(), self.game.player.get_pos_y() - 4),
                    );

                    
                    // Draw rock hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    self.core.wincan.draw_rect(Rect::new(174 * 4, 82 * 4, 64, 64));

                    }

                }

                GamePaused => {
                    self.core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                    self.core.wincan.clear();

                    let pause_menu = texture_creator.load_texture("assets/pause_menu.png")?;
                    self.core.wincan.copy(&pause_menu, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT));
                }

                _ => (),
            }

            // Tell SDL to draw everything on screen.
            self.core.wincan.present();

            Ok(())

    }

}

