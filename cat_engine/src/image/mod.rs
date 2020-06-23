//! # Основы работы с изображениями. Image basics. `feature = "texture_graphics"`, `default-features`.

use super::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    graphics::{Graphics,TexturedVertex},
};

mod texture;
pub use texture::{Texture,TextureCreationError};

pub use image;

use glium::draw_parameters::DrawParameters;

/// Основа для изображений (текстур). Image (texture) base.
/// 
/// Прямоугольник с точками: (x1, y1), (x1, y2), (x2, y1), (x2, y2).
/// 
/// Цветовой фильтр - [red, green, blue, alpha].
/// Цвет = цвет * фильтр.
/// 
/// Изменённая система координат - начало в центре экрана, ось Y инвертирована.
/// 
/// 
/// Rectagle with points: (x1, y1), (x1, y2), (x2, y1), (x2, y2).
/// 
/// Colour filter - [red, green, blue, alpha].
/// Colour = colour * filter.
/// 
/// Coordinate system is changed: origin is at the center of the screen, Y axe is reversed.
#[derive(Clone)]
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
        let x=unsafe{rect[0]-window_center[0]};
        let y=unsafe{window_center[1]-rect[1]};
        Self{
            x1:x,
            y1:y,
            x2:x+rect[2],
            y2:y-rect[3],
            colour_filter,
        }
    }

    /// Сдвигает координаты.
    ///
    /// Shifts coordinates.
    pub fn shift(&mut self,[dx,dy]:[f32;2]){
        self.x1+=dx;
        self.y1-=dy;
        self.x2+=dx;
        self.y2-=dy;
    }

    /// Массив координат
    /// для невращающихся изображений.
    /// 
    /// Returns vertex array for static images.
    pub (crate) fn vertex_buffer(&self)->[TexturedVertex;4]{
        let (x1,y1,x2,y2)=unsafe{(
            self.x1/window_center[0],
            self.y1/window_center[1],

            self.x2/window_center[0],
            self.y2/window_center[1]
        )};

        [
            TexturedVertex::new([x1,y1],[0.0,1.0]),
            TexturedVertex::new([x1,y2],[0.0,0.0]),
            TexturedVertex::new([x2,y1],[1.0,1.0]),
            TexturedVertex::new([x2,y2],[1.0,0.0])
        ]
    }

    /// Массив координат
    /// для вращающихся изображений.
    /// 
    /// Returns vertex array for rotating images.
    pub (crate) fn rotation_vertex_buffer(&self)->[TexturedVertex;4]{
        let (x1,y1,x2,y2)=(
            self.x1,
            self.y1,
            self.x2,
            self.y2
        );

        [
            TexturedVertex::new([x1,y1],[0.0,1.0]),
            TexturedVertex::new([x1,y2],[0.0,0.0]),
            TexturedVertex::new([x2,y1],[1.0,1.0]),
            TexturedVertex::new([x2,y2],[1.0,0.0])
        ]
    }

    #[inline(always)] /// Рисует изображение.
    pub fn draw(&self,texture:&Texture,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        graphics.draw_image(self,texture,draw_parameters);
    }

    #[inline(always)] /// Рисует изображение под углом.
    pub fn draw_rotate(&self,texture:&Texture,angle:f32,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        graphics.draw_rotate_image(self,texture,angle,draw_parameters);
    }
}