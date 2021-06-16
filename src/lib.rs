extern crate sdl2;

use sdl2::rect::Rect;

pub struct SDLCore {
	sdl_cxt: sdl2::Sdl,
	pub wincan: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
	pub cam: Rect,
}

impl SDLCore {
	pub fn init(title: &str, vsync: bool,
		    width: u32, height: u32) -> Result<SDLCore, String> {

		// Calls to SDL2 library for initializing window
		let sdl_cxt = sdl2::init()?;
		let video_subsys = sdl_cxt.video()?;

		let window = video_subsys.window(title, width, height)
			.build()
			.map_err(|e| e.to_string())?;

		let wincan = window.into_canvas().accelerated();

		// Determine whether SDL should lock to vsync or not.
		let wincan = match vsync {
			true => wincan.present_vsync(),
			false => wincan,
		};
		
		let wincan = wincan.build()
			.map_err(|e| e.to_string())?;

		// Incoming inputs from keyboard, window, etc.
		let event_pump = sdl_cxt.event_pump()?;

		// Defines the size of the window.
		let cam = Rect::new(0, 0, width, height);
		
		// Return new core struct.
		Ok(SDLCore{sdl_cxt, wincan, event_pump, cam,})
	}
}

pub trait Demo {
	fn init() -> Result<Self, String> where Self: Sized;
	fn run(&mut self) -> Result<(), String>;
}

pub fn runner<F, D>(desc: &str, initter: F)
	where
		F: Fn() -> Result<D, String>,
		D: Demo,
{
	println!("\nRunning {}:", desc);
	print!("\tInitting...");
	match initter() {
		Err(e) => println!("\n\t\tFailed to init: {}", e),
		Ok(mut d) => {
			println!("DONE");

			print!("\tRunning...");
			match d.run() {
				Err(e) => println!("\n\t\tEncountered error while running: {}", e),
				Ok(_) => println!("DONE\nExiting cleanly"),
			};
		},
	};
}
