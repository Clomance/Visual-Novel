#![allow(unused_imports)]

#[cfg(feature="mouse_cursor_icon")]
use crate::{
    graphics::{Graphics,Graphics2D},
    image::{ImageBase,Texture}
};

use super::{
    mouse_cursor,
    window_center
};

#[cfg(feature="mouse_cursor_icon")]
use glium::{
    Display,
    DrawParameters,
};

use std::path::Path;
use core::ops::Range;

/// Положение курсора мыши.
/// The mouse cursor position.
pub struct MouseCursor{
    position:[f32;2],
    saved_position:[f32;2],
}

impl MouseCursor{
    /// Инициирует новую позицию курсора.
    /// 
    /// Initiates new cursor position.
    pub const fn new()->MouseCursor{
        Self{
            position:[0f32;2],
            saved_position:[0f32;2],
        }
    }

    /// Сохраняет текущую позицию курсора мыши.
    /// 
    /// Saves the current mouse cursor position.
    pub fn save_position(&mut self){
        self.saved_position=self.position
    }

    /// Вычисляет перемещение от сохранённой позиции.
    /// 
    /// Calculates movement from the saved position.
    pub fn saved_shift(&self)->[f32;2]{
        [
            self.position[0]-self.saved_position[0],
            self.position[1]-self.saved_position[1]
        ]
    }

    #[inline(always)]
    pub fn x(&self)->f32{
        self.position[0]
    }

    #[inline(always)]
    pub fn y(&self)->f32{
        self.position[1]
    }

    /// Позиция курсора мыши.
    /// 
    /// The mouse cursor position.
    #[inline(always)]
    pub fn position(&self)->[f32;2]{
        self.position
    }

    /// Расстояние от курсора до центра окна.
    /// 
    /// Distance between the cursor and the center of the window.
    pub fn center_radius(&self)->[f32;2]{
        unsafe{[
            self.position[0]-window_center[0],
            self.position[1]-window_center[1]
        ]}
    }

    /// Уставливает позицию курсора мыши.
    /// 
    /// Sets the mouse cursor position.
    #[inline(always)]
    pub fn set_position(&mut self,position:[f32;2]){
        self.position=position;
    }
}

#[cfg(feature="mouse_cursor_icon")]
const radius:f32=30f32;
#[cfg(feature="mouse_cursor_icon")]
const d_radius:f32=5f32;

/// Иконка курсора мышки.
/// 
/// Загружает картинку из папки ресурсов.
#[cfg(feature="mouse_cursor_icon")]
pub struct MouseCursorIcon{
    image_base:ImageBase,
    texture:Texture,
    visible:bool,
}

#[cfg(feature="mouse_cursor_icon")]
impl MouseCursorIcon{
    pub fn new<P:AsRef<Path>>(path:P,range:Range<usize>,display:&Display,graphics:&mut Graphics2D)->MouseCursorIcon{
        let image_base=ImageBase::new([1f32;4],
            unsafe{[
                window_center[0]-radius/2f32,
                window_center[1]-radius/2f32,
                radius,
                radius
            ]}
        );

        graphics.bind_image(range,image_base.clone()).unwrap();

        Self{
            image_base,
            texture:Texture::from_path(path,display).unwrap(),
            visible:true,
        }
    }

    pub fn update(&self,graphics:&mut Graphics2D){
        graphics.rewrite_range_image(0,self.image_base.clone()).unwrap();
    }

    #[inline(always)]
    pub fn set_visible(&mut self,visible:bool){
        self.visible=visible
    }

    #[inline(always)]
    pub fn switch_visibility(&mut self){
        self.visible=!self.visible
    }

    /// При нажатии кнопки мыши.
    /// 
    /// On a mouse button pressed.
    pub fn pressed(&mut self,graphics:&mut Graphics2D){
        self.image_base.x1+=d_radius;
        self.image_base.y1-=d_radius;
        self.image_base.x2-=d_radius;
        self.image_base.y2+=d_radius;
        graphics.rewrite_range_image(0,self.image_base.clone()).unwrap();
    }

    /// При освобождении кнопки мыши.
    /// 
    /// On a mouse button released.
    pub fn released(&mut self,graphics:&mut Graphics2D){
        self.image_base.x1-=d_radius;
        self.image_base.y1+=d_radius;
        self.image_base.x2+=d_radius;
        self.image_base.y2-=d_radius;
        graphics.rewrite_range_image(0,self.image_base.clone()).unwrap();
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        if self.visible{
            let shift=unsafe{mouse_cursor.center_radius()};
            graphics.draw_shift_range_image(0,&self.texture,[1f32;4],shift,draw_parameters).unwrap();
        }
    }
}