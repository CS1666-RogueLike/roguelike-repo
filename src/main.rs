extern crate roguelike;

mod menu;
use crate::menu::*;

mod game;
use crate::game::*;

mod player;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
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
const TITLE: &str = "CS 1666 - RogueLike";
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
    // TEXTURES
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
        'gameloop: loop {

            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
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

            if keystate.contains(&Keycode::W) { print!("W"); mov_y -= 1; }
            if keystate.contains(&Keycode::S) { print!("S"); mov_y += 1; }
            if keystate.contains(&Keycode::A) { print!("A"); mov_x -= 1; }
            if keystate.contains(&Keycode::D) { print!("D"); mov_x += 1; }

            self.game.player.update_pos(mov_x, mov_y);

            println!("");

            // Draw game state
            self.draw();
        }


        // Out of game loop, return Ok
        Ok(())
    }
}

impl Manager {
    fn draw(& mut self) {
            self.core.wincan.set_draw_color(Color::BLACK);
            self.core.wincan.clear();

            self.core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
            self.core.wincan.fill_rect(Rect::new(self.game.player.get_pos_x(),
                                                self.game.player.get_pos_y(),
                                                self.game.player.get_hbox_x(),
                                                self.game.player.get_hbox_y()));

            self.core.wincan.present();

    }

}

