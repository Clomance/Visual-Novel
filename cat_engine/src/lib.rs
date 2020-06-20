#![allow(non_upper_case_globals,unused_must_use)]

/*!
 * # Графический движок с поддержкой аудио.
 * Graphics engine with audio support.
 * 
*/

pub use glium; // reimports

// text::{
//      Glyphs,
//      Character,
//      TextBase
// }
/// Основы работы с текстом. Text basics. `feature = "text_graphics"`, `default-features`.
#[cfg(feature="text_graphics")]
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

/// Основы работы с изображениями. Image basics. `feature = "texture_graphics"`, `default-features`.
#[cfg(feature="texture_graphics")]
pub mod image;

/// Графические основы. Graphic basics.
pub mod graphics;

/// Простая аудио система. Simple audio system - `feature = "audio"`.
#[cfg(feature="audio")]
pub mod audio;

/// RGBA - [f32; 4]
pub type Colour=[f32;4];

/// Возвращает прямоугольник размера окна.
/// Returns a window sized rectangle.
/// [0, 0, width, height]
pub fn window_rect()->[f32;4]{
    unsafe{[
        0f32,
        0f32,
        window_width,
        window_height,
    ]}
}