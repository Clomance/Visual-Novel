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

// Настройки графических основ
pub struct GraphicsSettings{
    pub texture_vertex_buffer_size:usize,
    pub simple_vertex_buffer_size:usize,
    pub text_vertex_buffer_size:usize,
}

impl GraphicsSettings{
    pub fn new()->GraphicsSettings{
        Self{
            texture_vertex_buffer_size:8usize,
            simple_vertex_buffer_size:100usize,
            text_vertex_buffer_size:2000usize,
        }
    }
}

pub struct Graphics2D{
    texture:TextureGraphics,
    simple:SimpleGraphics,
    text:TextGraphics,
}

impl Graphics2D{
    pub fn new(window:&Display,settings:GraphicsSettings,glsl:u16)->Graphics2D{
        Self{
            texture:TextureGraphics::new(window,settings.texture_vertex_buffer_size,glsl),
            simple:SimpleGraphics::new(window,settings.simple_vertex_buffer_size,glsl),
            text:TextGraphics::new(window,settings.text_vertex_buffer_size,glsl),
        }
    }

    // Сохраняет координаты картинки в выбранной области в буфере,
    // чтобы постоянно не загружать заново при отрисовке
    // Используется только для невращающихся изображений
    // Для вывода изображения из этой области используется функция 'draw_range_image'
    // Возращает номер области, если она не выходит за границы буфера
    pub fn bind_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        let data=image_base.vertex_buffer();
        self.texture.bind_range(range,&data)
    }

    // Сохраняет координаты картинки в выбранной области в буфере,
    // чтобы постоянно не загружать заново при отрисовке
    // Используется только для вращающихся изображений
    // Для вывода изображения из этой области используется функция 'draw_rotate_range_image'
    // Возращает номер области, если она не выходит за границы буфера
    pub fn bind_rotating_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        let data=image_base.rotation_vertex_buffer();
        self.texture.bind_range(range,&data)
    }

    #[inline(always)]
    pub fn unbind_texture(&mut self,index:usize){
        self.texture.unbind(index)
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

    pub fn draw_rotate_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        angle:f32,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_rotate_range(
            index,
            texture,
            colour_filter,
            angle,
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
    pub fn draw_simple<'a,O:SimpleObject<'a>>(&mut self,object:&O,draw_parameters:&mut DrawParameters){
        self.graphics.simple.draw(object,draw_parameters,self.frame)
    }

    #[inline(always)] // Рисует и сдвигает простой объект
    pub fn draw_move_simple<'a,O:SimpleObject<'a>>(&mut self,object:&O,movement:[f32;2],draw_parameters:&mut DrawParameters){
        self.graphics.simple.draw_move(object,movement,draw_parameters,self.frame)
    }

    #[inline(always)] // Рисует один символ
    pub fn draw_character(&mut self,colour:Colour,character:&Character,draw_parameters:&mut DrawParameters){
        self.graphics.text.draw_character(character,colour,draw_parameters,self.frame);
    }

    #[inline(always)] // Рисует изображение на основе image_base
    pub fn draw_image(&mut self,image_base:&ImageBase,texture:&Texture,draw_parameters:&mut DrawParameters){
        self.graphics.texture.draw_image(image_base,texture,draw_parameters,self.frame);
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
        self.graphics.texture.draw_rotate_image(image_base,texture,angle,self.frame,draw_parameters);
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

    #[inline(always)]
    pub fn draw_rotate_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        angle:f32,
        draw_parameters:&mut DrawParameters
    ){
        self.graphics.draw_rotate_range_image(
            index,
            texture,
            colour_filter,
            angle,
            draw_parameters,
            &mut self.frame
        );
    }
}

