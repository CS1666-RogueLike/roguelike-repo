extern crate roguelike;

mod menu;
use crate::menu::*;

mod game;
use crate::game::*;

mod player;

mod util;
use crate::util::*;

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
    core: SDLCore,
    menu: MenuState,
    game: Game,
}

impl Demo for Manager {
    // Initialize manager struct.
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, VSYNC, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let menu = MenuState::GameActive;
        let game = Game::new();
        Ok(Manager{core, menu, game})
    }

    fn run(&mut self) -> Result<(), String> {
        
        // Used so that holding the esc key down won't flash between states every frame


        'gameloop: loop {

            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'gameloop,
                    _ => {},
                }
            }

            let keystate: HashSet<Keycode> = self.core.event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            let mut mov_x = 0;
            let mut mov_y = 0;

            use menu::MenuState::*;

            match self.menu {

                GameActive => {
                    // Movement
                    if keystate.contains(&Keycode::W) { mov_y -= 1; }
                    if keystate.contains(&Keycode::S) { mov_y += 1; }
                    if keystate.contains(&Keycode::A) { mov_x -= 1; }
                    if keystate.contains(&Keycode::D) { mov_x += 1; }

                    self.game.player.update_pos(mov_x, mov_y);
                }

                GamePaused => {}

                MainMenu => {}
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
            match self.menu {
                GameActive => {
                    // Draw black screen
                    self.core.wincan.set_draw_color(Color::BLACK);
                    self.core.wincan.clear();
                    
                    // Draw background
                    let texture_creator = self.core.wincan.texture_creator();
                    let bg = texture_creator.load_texture("assets/test_image.png")?;
                    self.core.wincan.copy(&bg, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT));
        
                    // Draw player hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    self.core.wincan.fill_rect(Rect::new(self.game.player.get_pos_x() - (self.game.player.get_hbox_x()/2) as i32,
                                                        self.game.player.get_pos_y() - (self.game.player.get_hbox_y()/2) as i32,
                                                        self.game.player.get_hbox_x(),
                                                        self.game.player.get_hbox_y()));
        
                    // Draw null at center of player hitbox
                    self.core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
                    self.core.wincan.draw_line(
                        Point::new(self.game.player.get_pos_x() + 4, self.game.player.get_pos_y()),
                        Point::new(self.game.player.get_pos_x() - 4, self.game.player.get_pos_y()),
                    );
                    self.core.wincan.draw_line(
                        Point::new(self.game.player.get_pos_x(), self.game.player.get_pos_y() + 4),
                        Point::new(self.game.player.get_pos_x(), self.game.player.get_pos_y() - 4),
                    );
                }

                GamePaused => {
                    println!("GAME IS PAUSED, PRESS ESC TO RESUME");
                    self.core.wincan.set_draw_color(Color::RGBA(100, 0, 0, 255));
                    self.core.wincan.clear();
                }

                _ => (),
            }

            self.core.wincan.present();

            Ok(())

    }

}

