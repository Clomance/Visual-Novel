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
    index::{
        PrimitiveType, // enum
        NoIndices,
    },
};

use core::ops::Range;

pub struct Graphics2D{
    texture:TextureGraphics,
    simple:SimpleGraphics,
    text:TextGraphics,
}

impl Graphics2D{
    pub fn new(window:&Display,glsl:u16)->Graphics2D{
        Self{
            texture:TextureGraphics::new(window,8,glsl),
            simple:SimpleGraphics::new(window,glsl),
            text:TextGraphics::new(window,glsl),
        }
    }

    // Сохраняет координаты картинки в выбранной области в буфере,
    // чтобы постоянно не загружать из заново при отрисовке
    // Для вывода изображения из этой области используется функция 'draw_range_image'
    // Возращает номер области, если она не выходит за границы буфера
    #[inline(always)]
    pub fn bind_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        self.texture.bind_range_image(range,image_base)
    }

    pub fn draw_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_range(
            index,
            texture,
            colour_filter,
            indices,
            draw_parameters,
            frame
        );
    }

    pub fn draw_move_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        movement:[f32;2],
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_move_range(
            index,
            texture,
            colour_filter,
            movement,
            indices,
            draw_parameters,
            frame
        );
    }
}

// Простой интерфейс для связи кадра и графических функций
pub struct Graphics<'graphics,'frame>{
    graphics:&'graphics Graphics2D,
    frame:&'frame mut Frame,
}

impl<'graphics,'frame> Graphics<'graphics,'frame>{
    #[inline(always)]
    pub fn new(graphics:&'graphics Graphics2D,frame:&'frame mut Frame)->Graphics<'graphics,'frame>{
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

    #[inline(always)] // Рисует простой объект
    pub fn draw_simple<O:SimpleObject>(&mut self,object:&O,draw_parameters:&mut DrawParameters){
        self.graphics.simple.draw(object,draw_parameters,self.frame)
    }

    #[inline(always)] // Рисует один символ
    pub fn draw_character(&mut self,colour:Colour,character:&Character,draw_parameters:&mut DrawParameters){
        self.graphics.text.draw_character(character,colour,draw_parameters,self.frame);
    }

    #[inline(always)] // Рисует изображение на основе image_base
    pub fn draw_image(&mut self,image_base:&ImageBase,texture:&Texture,draw_parameters:&mut DrawParameters){
        self.graphics.texture.draw_image(image_base,texture,draw_parameters,self.frame)
    }

    #[inline(always)] // Рисует изображение на сохранённой основе
    pub fn draw_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        draw_parameters:&mut DrawParameters
    ){
        self.graphics.draw_range_image(
            index,
            texture,
            colour_filter,
            draw_parameters,
            self.frame,
        )
    }

    #[inline(always)] // Рисует изображение на основе image_base c поворотом в 'angle' градусов
    pub fn draw_rotate_image(&mut self,image_base:&ImageBase,texture:&Texture,angle:f32,draw_parameters:&mut DrawParameters){
        self.graphics.texture.draw_rotate_image(image_base,texture,angle,self.frame,draw_parameters)
    }

    #[inline(always)]
    pub fn draw_move_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        movement:[f32;2],
        draw_parameters:&mut DrawParameters
    ){
        self.graphics.draw_move_range_image(
            index,
            texture,
            colour_filter,
            movement,
            draw_parameters,
            &mut self.frame
        );
    }
}

