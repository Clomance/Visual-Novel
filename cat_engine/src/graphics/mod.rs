//! # Графические основы. Graphic basics.
//! 
//! Графический движок разделен на три части:
//! 1. Простая графика - одноцветные объекты состоящие из `Point2D`.
//! 2. Текстуры (изображения)
//! 3. Текст
//! 
//! Graphics engine divided into three parts:
//! 1. Simple graphics - plain objects composed of `Point2D`.
//! 2. Textures (images)
//! 3. Text

mod graphics;
pub (crate) use graphics::Graphics2D;
pub use graphics::{Graphics,GraphicsSettings};


#[cfg(feature="simple_graphics")]
mod simple_graphics;
#[cfg(feature="simple_graphics")]
pub (crate) use simple_graphics::SimpleGraphics;
#[cfg(feature="simple_graphics")]
pub use simple_graphics::{
    SimpleObject,
    Point2D,
};

#[cfg(feature="simple_graphics")]
mod graphic_basics;
#[cfg(feature="simple_graphics")]
pub use graphic_basics::*;


#[cfg(feature="texture_graphics")]
mod texture_graphics;
#[cfg(feature="texture_graphics")]
pub (crate) use texture_graphics::{TextureGraphics,TexturedVertex};

#[cfg(feature="text_graphics")]
mod text_graphics;
#[cfg(feature="text_graphics")]
pub (crate) use text_graphics::TextGraphics;