use crate::*;

pub trait Drawable{
    fn set_alpha_channel(&mut self,alpha:f32);

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics);

    fn draw_smooth(&mut self,alpha:f32,context:&Context,graphics:&mut GlGraphics){
        self.set_alpha_channel(alpha);
        self.draw(context,graphics);
    }
}