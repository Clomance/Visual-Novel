use crate::*;

use engine::{
    // types
    Colour,
    // structs
    graphics::{
        GameGraphics,
        Rectangle
    },
};

use engine::glium::DrawParameters;

pub struct Background{
    base:Rectangle,
}

impl Background{
    pub fn new(colour:Colour,rect:[f32;4])->Background{
        Self{
            base:Rectangle::new(rect,colour)
        }
    }
}

impl Drawable for Background{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.colour[3]=alpha
    }

    fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        self.base.draw(draw_parameters,graphics)
    }
}