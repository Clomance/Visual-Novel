use crate::{
    // types
    Colour,
    // structs
    image::{ImageBase,Texture},
    text::Character,
};

use super::{
    SimpleGraphics,
    SimpleObject,
    TextureGraphics,
    TextGraphics,
};

use glium::{
    Frame,
    DrawParameters,
    Surface,
    Display,
};

pub struct Graphics2D{
    texture:TextureGraphics,
    simple:SimpleGraphics,
    text:TextGraphics,
}

impl Graphics2D{
    pub fn new(window:&Display,glsl:u16)->Graphics2D{
        Self{
            texture:TextureGraphics::new(window,glsl),
            simple:SimpleGraphics::new(window,glsl),
            text:TextGraphics::new(window,glsl),
        }
    }

    #[inline(always)] // Рисует один символ
    pub fn draw_character(
        &self,
        character:&Character,
        colour:Colour,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    ){
        self.text.draw_character(character,colour,draw_parameters,frame);
    }

    #[inline(always)] // Рисует простой объект
    pub fn draw_simple<O:SimpleObject>(
        &self,
        object:&O,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
        self.simple.draw(object,draw_parameters,frame);
    }
}

// Простой интерфейс для связи кадра и графических функций
pub struct GameGraphics<'graphics,'frame>{
    graphics:&'graphics Graphics2D,
    frame:&'frame mut Frame,
}

impl<'graphics,'frame> GameGraphics<'graphics,'frame>{
    #[inline(always)]
    pub fn new(graphics:&'graphics Graphics2D,frame:&'frame mut Frame)->GameGraphics<'graphics,'frame>{
        Self{
            graphics,
            frame
        }
    }

    #[inline(always)]
    pub fn frame(&mut self)->&mut Frame{
        self.frame
    }

    #[inline(always)]
    pub fn clear_colour(&mut self,colour:[f32;4]){
        self.frame.clear_color(colour[0],colour[1],colour[2],colour[3]);
    }

    #[inline(always)]
    fn clear_stencil(&mut self,value: u8){
        self.frame.clear_stencil(value as i32);
    }

    #[inline(always)] // Рисует простой объект
    pub fn draw_simple<O:SimpleObject>(&mut self,object:&O,draw_parameters:&mut DrawParameters){
        self.graphics.draw_simple(object,draw_parameters,self.frame)
    }

    #[inline(always)] // Рисует один символ
    pub fn draw_character(&mut self,colour:Colour,character:&Character,draw_parameters:&DrawParameters){
        self.graphics.text.draw_character(character,colour,draw_parameters,self.frame);
    }

    #[inline(always)] // Рисует тектстуру на основе image_base
    pub fn draw_texture(&mut self,image_base:&ImageBase,texture:&Texture,draw_parameters:&DrawParameters){
        self.graphics.texture.draw_texture(image_base,texture,self.frame,draw_parameters)
    }

    #[inline(always)] // Рисует тектстуру на основе image_base c поворотом в 'angle' градусов
    pub fn draw_rotate_texture(&mut self,image_base:&ImageBase,texture:&Texture,angle:f32,draw_parameters:&mut DrawParameters){
        self.graphics.texture.draw_rotate_texture(image_base,texture,angle,self.frame,draw_parameters)
    }
}