extern crate sdl_rust;

use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL01 Hello World";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const TIMEOUT: u64 = 7500;

pub struct SDL01 {
	core: SDLCore,
}

impl Demo for SDL01 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL01{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		let ms = texture_creator.load_texture("Dan.png")?;
		let tux = texture_creator.load_texture("connor.png")?;

		self.core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
		self.core.wincan.clear();

		self.core.wincan.copy(&ms, None, None)?;
		self.core.wincan.present();

		// Note SDL has a timer subsystem, but the Rust SDL bindings recommend
		// the use of std::thread::sleep and std::time instead
		thread::sleep(Duration::from_millis(TIMEOUT));

		self.core.wincan.copy(&tux, None, None)?;
		self.core.wincan.present();

		thread::sleep(Duration::from_millis(TIMEOUT));

		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL01::init);
}
