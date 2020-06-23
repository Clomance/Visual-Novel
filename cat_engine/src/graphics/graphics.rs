use crate::Colour;

#[cfg(feature="texture_graphics")]
use crate::image::{ImageBase,Texture};
#[cfg(feature="texture_graphics")]
use super::TextureGraphics;

#[cfg(feature="simple_graphics")]
use super::{SimpleGraphics,SimpleObject};

#[cfg(feature="text_graphics")]
use crate::text::Character;
#[cfg(feature="text_graphics")]
use super::TextGraphics;

use glium::{
    // enums
    DrawError,
    // traits
    Surface,
    // structs
    Frame,
    DrawParameters,
    Display,
    index::{
        PrimitiveType, // enum
        NoIndices,
    },
};

use core::ops::Range;

/// Настройки графических основ.
/// 
/// Settings for graphic basics.
pub struct GraphicsSettings{
    #[cfg(feature="texture_graphics")]
    pub texture_vertex_buffer_size:usize,
    #[cfg(feature="simple_graphics")]
    pub simple_vertex_buffer_size:usize,
    #[cfg(feature="text_graphics")]
    pub text_vertex_buffer_size:usize,
}

impl GraphicsSettings{
    pub const fn new()->GraphicsSettings{
        Self{
            #[cfg(feature="texture_graphics")]
            texture_vertex_buffer_size:8usize,
            #[cfg(feature="simple_graphics")]
            simple_vertex_buffer_size:100usize,
            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:2000usize,
        }
    }
}

/// Графические основы.
pub struct Graphics2D{
    #[cfg(feature="texture_graphics")]
    texture:TextureGraphics,
    #[cfg(feature="simple_graphics")]
    simple:SimpleGraphics,
    #[cfg(feature="text_graphics")]
    text:TextGraphics,
}

impl Graphics2D{
    pub fn new(window:&Display,settings:GraphicsSettings,glsl:u16)->Graphics2D{
        Self{
            #[cfg(feature="texture_graphics")]
            texture:TextureGraphics::new(window,settings.texture_vertex_buffer_size,glsl),
            #[cfg(feature="simple_graphics")]
            simple:SimpleGraphics::new(window,settings.simple_vertex_buffer_size,glsl),
            #[cfg(feature="text_graphics")]
            text:TextGraphics::new(window,settings.text_vertex_buffer_size,glsl),
        }
    }

    /// Сохраняет координаты картинки в выбранной области в буфере,
    /// чтобы постоянно не загружать заново при отрисовке.
    /// Возращает номер области, если она не выходит за границы буфера.
    /// 
    /// Используется только для невращающихся изображений.
    /// 
    /// Для вывода изображения из этой области используется функция 'draw_range_image'.
    #[cfg(feature="texture_graphics")]
    pub fn bind_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        let data=image_base.vertex_buffer();
        self.texture.bind_range(range,&data)
    }

    /// Сохраняет координаты картинки в выбранной области в буфере,
    /// чтобы постоянно не загружать заново при отрисовке.
    /// Возращает номер области, если она не выходит за границы буфера.
    /// 
    /// Используется только для вращающихся изображений.
    /// 
    /// Для вывода изображения из этой области используется функция 'draw_rotate_range_image'.
    #[cfg(feature="texture_graphics")]
    pub fn bind_rotating_image(&mut self,range:Range<usize>,image_base:ImageBase)->Option<usize>{
        let data=image_base.rotation_vertex_buffer();
        self.texture.bind_range(range,&data)
    }

    #[cfg(feature="simple_graphics")]
    pub fn bind_simple<'a,O:SimpleObject<'a>>(&mut self,range:Range<usize>,object:&O)->Option<usize>{
        let data=object.vertex_buffer();
        self.simple.bind_range(range,&data)
    }

    #[cfg(feature="texture_graphics")]
    pub fn rewrite_range_image(&mut self,range:usize,image_base:ImageBase)->Option<()>{
        let data=image_base.rotation_vertex_buffer();
        self.texture.rewrite_range(range,&data)
    }

    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn unbind_texture(&mut self,index:usize){
        self.texture.unbind(index)
    }

    #[inline(always)]
    #[cfg(feature="simple_graphics")]
    pub fn unbind_simple(&mut self,index:usize){
        self.simple.unbind(index)
    }

    #[cfg(feature="texture_graphics")]
    pub fn draw_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_range(
            index,
            texture,
            colour_filter,
            indices,
            draw_parameters,
            frame
        )
    }

    #[cfg(feature="texture_graphics")]
    pub fn draw_shift_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_shift_range(
            index,
            texture,
            colour_filter,
            shift,
            indices,
            draw_parameters,
            frame
        )
    }

    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_range_image(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        angle:f32,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        self.texture.draw_rotate_range(
            index,
            texture,
            colour_filter,
            angle,
            indices,
            draw_parameters,
            frame
        )
    }

    #[cfg(feature="simple_graphics")]
    pub fn draw_range_simple<'a,O:SimpleObject<'a>>(
        &self,
        index:usize,
        object:&O,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let colour=object.colour();
        let draw_type=object.indices();

        self.simple.draw_range(
            index,
            colour,
            draw_type,
            draw_parameters,
            frame
        )
    }
}

/// Простой интерфейс для связи кадра и графических функций
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

    /// Возвращает ссылку на кадр.
    /// 
    /// Return reference to the frame.
    #[inline(always)]
    pub fn frame(&mut self)->&mut Frame{
        self.frame
    }

    #[inline(always)]
    pub fn clear_colour(&mut self,colour:[f32;4]){
        self.frame.clear_color(colour[0],colour[1],colour[2],colour[3]);
    }

    /// Рисует простой объект.
    /// 
    /// Draws a simple object.
    #[inline(always)]
    #[cfg(feature="simple_graphics")]
    pub fn draw_simple<'a,O:SimpleObject<'a>>(
        &mut self,
        object:&O,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw(object,draw_parameters,self.frame)
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws a shifted simple object.
    #[inline(always)] 
    #[cfg(feature="simple_graphics")]
    pub fn draw_shift_simple<'a,O:SimpleObject<'a>>(
        &mut self,
        object:&O,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.simple.draw_shift(object,shift,draw_parameters,self.frame)
    }

    /// Рисует один символ.
    /// 
    /// Draws one character.
    #[inline(always)]
    #[cfg(feature="text_graphics")]
    pub fn draw_character(
        &mut self,
        colour:Colour,
        character:&Character,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.text.draw_character(character,colour,draw_parameters,self.frame)
    }

    /// Рисует изображение на основе `ImageBase`.
    /// 
    /// Draws image based on `ImageBase`.
    #[inline(always)] 
    #[cfg(feature="texture_graphics")]
    pub fn draw_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.texture.draw_image(image_base,texture,draw_parameters,self.frame)
    }

    /// Рисует изображение на основе `ImageBase` c поворотом в 'angle' градусов.
    /// 
    /// Draws image based on `ImageBase` rotated `angle` degrees.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_image(
        &mut self,
        image_base:&ImageBase,
        texture:&Texture,
        angle:f32,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.texture.draw_rotate_image(image_base,texture,angle,self.frame,draw_parameters)
    }
}

/// Функции для работы с областями.
/// 
/// Function to work with ranges.
impl<'graphics,'frame> Graphics<'graphics,'frame>{
    /// Рисует изображение на основе данных из области.
    /// 
    /// Draws image based on data from a range.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.draw_range_image(
            index,
            texture,
            colour_filter,
            draw_parameters,
            self.frame,
        )
    }

    /// Рисует сдвинутое изображение на основе данных из области.
    /// 
    /// Draws shifted image based on data from a range.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_shift_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        shift:[f32;2],
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.draw_shift_range_image(
            index,
            texture,
            colour_filter,
            shift,
            draw_parameters,
            &mut self.frame
        )
    }

    /// Рисует изображение с поворотом в 'angle' градусов на основе
    /// данных из области.
    /// 
    /// Draws image based on data from a range rotated `angle` degrees.
    #[inline(always)]
    #[cfg(feature="texture_graphics")]
    pub fn draw_rotate_range_image(
        &mut self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        angle:f32,
        draw_parameters:&mut DrawParameters
    )->Result<(),DrawError>{
        self.graphics.draw_rotate_range_image(
            index,
            texture,
            colour_filter,
            angle,
            draw_parameters,
            &mut self.frame
        )
    }
}