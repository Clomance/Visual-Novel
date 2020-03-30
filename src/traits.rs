use crate::*;

pub trait Drawable{
    fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics);
}

pub trait Clickable{
    fn pressed(&self)->bool;
}