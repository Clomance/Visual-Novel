#![allow(non_upper_case_globals,unused_must_use)]

//! # 2D графический движок с поддержкой аудио. 2D graphics engine with audio support.
//! 
//! Использует OpenGL 2.0 и выше.
//! 
//! Текст рисуется поточечно. Так что важно указать правильный размер буфера.
//! 
//! 
//! 
//! Uses OpenGL 2.0 and above.
//! 
//! Text is drawn pointwise. It's important to set corrent size of the text graphics buffer.
//! 
//! ```
//! // Default settings
//! let mut window=Window::new(|_,_|{}).unwrap();
//! 
//! while let Some(event)=window.next_event(){
//!     match event{
//!         WindowEvent::Exit=>break,
//!         _=>{}
//!     }
//! }
//! ```

pub use glium; // reimports

#[cfg(feature="audio")]
pub mod audio;

#[cfg(feature="text_graphics")]
pub mod text;

#[cfg(feature="texture_graphics")]
pub mod image;

pub mod graphics;

mod mouse_cursor;

mod window;
pub use window::*;




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