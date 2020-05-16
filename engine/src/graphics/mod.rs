mod simple_graphics;
use simple_graphics::Point2D;
pub use simple_graphics::{
    SimpleGraphics,
    SimpleObject,
};

mod game_graphics;
pub use game_graphics::*;

mod graphic_basics;
pub use graphic_basics::*;

mod texture_graphics;
pub use texture_graphics::*;