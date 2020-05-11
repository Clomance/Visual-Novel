use crate::GameGraphics;
use graphics::Context;

pub trait Drawable{
    fn set_alpha_channel(&mut self,alpha:f32);

    fn draw(&mut self,context:&Context,graphics:&mut GameGraphics);

    fn draw_smooth(&mut self,alpha:f32,context:&Context,graphics:&mut GameGraphics){
        self.set_alpha_channel(alpha);
        self.draw(context,graphics);
    }
}