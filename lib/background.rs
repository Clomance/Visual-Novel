use crate::*;

pub struct Background{
    rect:[f64;4],
    base:Rectangle,
}

impl Background{
    pub fn new(color:Color,rect:[f64;4])->Background{
        Self{
            rect,
            base:Rectangle::new(color)
        }
    }
}

impl Drawable for Background{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.color[3]=alpha
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.base.draw(self.rect,&context.draw_state,context.transform,graphics)
    }
}