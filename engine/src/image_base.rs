use super::{
    // types
    Colour,
    // structs
    game_texture::Texture,
    game_graphics::GameGraphics,
};

use glium::draw_parameters::DrawParameters;

pub struct ImageBase{
    pub rect:[f32;4],
    pub colour:Colour,
}

impl ImageBase{
    // rect - [x,y,width,height]
    pub fn new(colour:Colour,rect:[f32;4])->ImageBase{
        Self{
            rect,
            colour,
        }
    }

    pub fn draw(&self,texture:&Texture,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_texture(self,texture,draw_parameters);
    }
}