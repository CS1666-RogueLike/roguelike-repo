extern crate roguelike;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use roguelike::SDLCore;
use roguelike::Demo;

use std::cmp::min;

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
}

impl Demo for Manager {
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, VSYNC, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        Ok(Manager{core})
    }

    fn run(&mut self) -> Result<(), String> {
        let texture_creator = self.core.wincan.texture_creator();

        let dan = texture_creator.load_texture("images/Dan.png")?;
        let jagr = texture_creator.load_texture("images/Lich.jpg")?;
        let tyler = texture_creator.load_texture("images/Tyler.jpg")?;
        let connor = texture_creator.load_texture("images/connor.png")?;
        let joe = texture_creator.load_texture("images/joecavanaugh.jpg")?;
        let keyon = texture_creator.load_texture("images/keyon_h.JPG")?;
        let trae = texture_creator.load_texture("images/trae.png")?;

        let pics = vec![dan, jagr, tyler, connor, joe, keyon, trae];
        let mut current_index = 0;

        'gameloop: loop {
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                    Event::KeyDown{ keycode: Some(Keycode::Space), .. } => {
                        current_index = min(current_index + 1, pics.len() - 1);
                    }
                    _ => {},
                }
            }

            self.core.wincan.set_draw_color(Color::BLACK);
            self.core.wincan.clear();

            // Draw current picture
            self.core.wincan.copy(&pics[current_index], None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT))?;

            self.core.wincan.present();
        }

        // Out of game loop, return Ok
        Ok(())
    }
}

