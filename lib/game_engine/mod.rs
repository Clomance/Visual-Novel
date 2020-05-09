pub mod text;

pub mod game_graphics;
pub use game_graphics::*;

pub mod draw_state;

pub mod mouse_cursor;
pub use mouse_cursor::*;

pub mod game_texture;
pub use game_texture::*;

pub mod game_window;
pub use game_window::*;

const open_gl:shader_version::OpenGL=shader_version::OpenGL::V3_2;