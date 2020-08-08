use tetra::graphics::Color;
use tetra::math::Vec2;
use crate::TetraVec2;
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	// Game configuration
	pub titel: String,
	pub clear_color: Color,
	// Window settings
	pub window_width: i32,
	pub window_height: i32,
	pub window_scale: i32,
	pub maximized: bool,
	pub fullscreen: bool,
	pub resizable: bool,
	// Tetra settings
	pub show_mouse: bool,
	pub vsync: bool,
	pub quit_on_escape: bool,
	// Game Music
	pub master_volume: f32,
}

impl Config {

	pub fn version(&self) -> String{
		env!("CARGO_PKG_VERSION").to_owned()
	}

	pub fn half_window(&self) -> TetraVec2 {
		Vec2::new((self.window_width / 2) as f32, (self.window_height / 2) as f32)
	}
}

pub fn load_config(path: &str) -> Config{
	match ron::from_str(path){
		Ok(config) => config,
		Err(error) => {
			println!("Failed to load config: {}", error);
			std::process::exit(1);
		}
	}
}