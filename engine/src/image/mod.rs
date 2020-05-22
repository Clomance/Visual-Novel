use super::{
    // types
    Colour,
    // structs
    graphics::GameGraphics,
};

mod texture;
pub use texture::Texture;

pub use image;
use glium::draw_parameters::DrawParameters;

// Основа для изображений (текстур)
// Прямоугольник с координатами: (x1,y1), (x1,y2), (x2,y1), (x2,y2)
// Цветовой фильтр - [red, green, blue, alpha]
// Цвет = цвет * фильтр
pub struct ImageBase{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour_filter:Colour,
}

impl ImageBase{
    // rect - [x,y,width,height]
    pub fn new(colour_filter:Colour,rect:[f32;4])->ImageBase{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            colour_filter,
        }
    }

    #[inline(always)] // Рисует изображение
    pub fn draw(&self,texture:&Texture,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_texture(self,texture,draw_parameters);
    }

    #[inline(always)] // Рисует изображение под углом
    pub fn draw_rotate(&self,texture:&Texture,angle:f32,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_rotate_texture(self,texture,angle,draw_parameters);
    }
}