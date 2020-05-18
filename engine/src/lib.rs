#![allow(non_upper_case_globals,unused_must_use,dead_code)]
// reimports
pub use glium;

// text::{
//      Glyphs,
//      Character,
//      TextBase
// }
pub mod text;

mod mouse_cursor;

mod window;
pub use window::*;

pub mod image;

pub mod graphics;

pub mod music;

pub type Colour=[f32;4];