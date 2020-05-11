use crate::{
    // statics
    mouse_cursor,
    window_width,
    window_height,
    //
    colors::*,
    Drawable,
    GameGraphics,
    text::{Glyphs,TextBase},
    Align,
    AlignX,
    AlignY,
};

use graphics::{
    ellipse::Border,
    ellipse::Ellipse,
    line::Line,
    types::Color,
    rectangle::Rectangle,
    Context,
};

mod text_views;
pub use text_views::*;

mod edit_text_view;
pub use edit_text_view::*;

mod button;
pub use button::*;

mod menu;
pub use menu::{
    Menu,
    MenuSettings,
};

mod slider;
pub use slider::*;

mod wallpaper;
pub use wallpaper::*;