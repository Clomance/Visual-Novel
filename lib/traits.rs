use engine::graphics::GameGraphics;
use engine::glium::DrawParameters;


pub trait Drawable{
    fn set_alpha_channel(&mut self,alpha:f32);

    fn draw(&mut self,draw_parameters:&DrawParameters,graphics:&mut GameGraphics);

    fn draw_smooth(&mut self,alpha:f32,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        self.set_alpha_channel(alpha);
        self.draw(draw_parameters,graphics);
    }
}