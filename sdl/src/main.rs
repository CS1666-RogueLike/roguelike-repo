extern crate sdl;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use sdl::SDLCore;
use sdl::Demo;

use std::cmp::min;

const TITLE: &str = "CS 1666 - RogueLike";

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct SDLInfo {
    core: SDLCore,
}

impl Demo for SDLInfo {
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(SDLInfo{core})
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
            self.core.wincan.copy(&pics[current_index], None, Rect::new(0, 0, CAM_W, CAM_H))?;

            self.core.wincan.present();
        }

        // Out of game loop, return Ok
        Ok(())
    }
}

fn main() {
    sdl::runner(TITLE, SDLInfo::init);
}