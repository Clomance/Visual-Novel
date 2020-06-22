#![allow(non_upper_case_globals,unused_must_use)]

/*!
 * # 2D графический движок с поддержкой аудио. 2D graphics engine with audio support.
 * 
 * Использует OpenGL 2.0 и выше.
 * 
 * Текст рисуется поточечно. Так что важно указать правильный размер буфера.
 * 
 * 
 * 
 * Uses OpenGL 2.0 and above.
 * 
 * Text is drawn pointwise. It's important to set corrent size of the text graphics buffer.
*/

pub use glium; // reimports

// text::{
//      Glyphs,
//      Character,
//      TextBase
// }
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

#[cfg(feature="texture_graphics")]
pub mod image;

pub mod graphics;

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