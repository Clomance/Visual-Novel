#![allow(non_upper_case_globals,unused_must_use)]
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
// Window,

// WindowEvent,
// MouseButton,
// KeyboardButton

// mouse_cursor,
// window_width,
// window_height
// window_center
pub use window::*;

// image::{
//      image,
//
//      Texture,
//      ImageBase
// }
pub mod image;

pub mod graphics;

pub mod music;

pub type Colour=[f32;4];

// Прямоугольник размера окна
// [x, y, width, height]
pub fn window_rect()->[f32;4]{
    unsafe{[
        0f32,
        0f32,
        window_width,
        window_height,
    ]}
}