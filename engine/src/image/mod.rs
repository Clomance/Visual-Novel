use super::{
    // types
    Colour,
    // structs
    graphics::GameGraphics,
};

mod texture;
pub use texture::Texture;

pub use image;
use glium::draw_parameters::DrawParameters;

pub struct ImageBase{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour_filter:Colour,
}

impl ImageBase{
    // rect - [x,y,width,height]
    pub fn new(colour_filter:Colour,rect:[f32;4])->ImageBase{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            colour_filter,
        }
    }

    pub fn draw(&self,texture:&Texture,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_texture(self,texture,draw_parameters);
    }

    pub fn draw_rotate(&self,texture:&Texture,angle:f32,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_rotate_texture(self,texture,angle,draw_parameters);
    }
}