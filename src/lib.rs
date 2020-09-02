#[cfg(feature = "tilemap")]
pub mod tilemap;
#[cfg(feature = "tilemap_json")]
pub mod tilemap;
#[cfg(feature = "tilemap_xml")]
pub mod tilemap;

pub mod scenes;
#[cfg(feature = "ron_file")]
pub mod config;
#[cfg(feature = "animation")]
pub mod animation;
pub mod input_action;
pub mod gui;
pub mod utils;
#[cfg(feature = "sound")]
pub mod sound;

// experimental pack
//mod experimental;

// custom types
pub type TetraVec2 = tetra::math::Vec2<f32>;

