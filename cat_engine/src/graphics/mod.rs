#![allow(unused_imports,unused_variables,dead_code)]
//! # Графические основы. Graphic basics.
//! 
//! Графический движок разделен на три части:
//! 1. Простая графика - одноцветные объекты состоящие из `Point2D`.
//! 2. Текстуры (изображения)
//! 3. Текст
//! 
//! Также есть возможность сохранять и использовать координаты объектов.
//! (пример ниже)
//! 
//! 
//! 
//! Graphics engine divided into three parts:
//! 1. Simple graphics - plain objects composed of `Point2D`.
//! 2. Textures (images)
//! 3. Text
//! 
//! Also it's possible to save and use vertexes of objects.
//! 
//! 
//! ```
//! let image_base=ImageBase::new(White,unsafe{[
//!     (window_width-400f32)/2f32,
//!     (window_height-400f32)/2f32,
//!     400f32,
//!     400f32
//! ]});
//! 
//! let range=window.graphics().bind_image(4..8usize,image_base).unwrap();
//! 
//! let logo=Texture::from_path("./resources/images/logo.png",window.display()).unwrap();
//! 
//! window.draw(|parameters,graphics|{
//!     graphics.clear_colour(White);
//!     graphics.draw_range_image(
//!         range,
//!         &logo,
//!         White,
//!         parameters
//!     );
//! });
//! 
//! window.graphics().unbind_texture(range);
//! ```

mod graphics;
pub (crate) use graphics::{Graphics2D,GraphicsSettings};
pub use graphics::{Graphics,};


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