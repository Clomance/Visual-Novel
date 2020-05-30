use super::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    graphics::{Graphics,TexturedVertex},
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

    // Массив готовых для вывода координат
    pub fn vertex_buffer(&self)->[TexturedVertex;4]{
        let (x1,y1,x2,y2)=unsafe{(
            self.x1/window_center[0]-1f32,
            1f32-self.y1/window_center[1],

            self.x2/window_center[0]-1f32,
            1f32-self.y2/window_center[1]
        )};

        [
            TexturedVertex::new([x1,y1],[0.0,1.0]),
            TexturedVertex::new([x1,y2],[0.0,0.0]),
            TexturedVertex::new([x2,y1],[1.0,1.0]),
            TexturedVertex::new([x2,y2],[1.0,0.0])
        ]
    }

    #[inline(always)] // Рисует изображение
    pub fn draw(&self,texture:&Texture,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        graphics.draw_image(self,texture,draw_parameters);
    }

    #[inline(always)] // Рисует изображение под углом
    pub fn draw_rotate(&self,texture:&Texture,angle:f32,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        graphics.draw_rotate_image(self,texture,angle,draw_parameters);
    }
}