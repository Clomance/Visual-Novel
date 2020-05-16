#![allow(non_upper_case_globals,unused_must_use,dead_code)]
// reimports
pub use glium;
pub use image;

// text::{
//      Glyphs,
//      Character,
//      TextBase
// }
pub mod text;

mod mouse_cursor;

pub mod game_texture;

mod game_window;
pub use game_window::*;

pub mod image_base;

pub mod graphics;

pub mod music;


pub type Colour=[f32;4];