#![allow(dead_code,unused_imports)]

use super::window_center;

#[cfg(feature="mouse_cursor_icon")]
use super::{
    graphics::Graphics,
    image::{ImageBase,Texture}
};

#[cfg(feature="mouse_cursor_icon")]
use glium::{
    Display,
    DrawParameters,
};

use std::path::PathBuf;

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


const radius:f32=30f32;
const d_radius:f32=5f32;

/// Иконка курсора мышки.
/// 
/// Загружает картинку из папки ресурсов.
#[cfg(feature="mouse_cursor_icon")]
pub struct MouseCursorIcon{
    image_base:ImageBase,
    texture:Texture,
    radius:f32,
    visible:bool,
}

#[cfg(feature="mouse_cursor_icon")]
impl MouseCursorIcon{
    pub fn new(display:&Display,path:PathBuf)->MouseCursorIcon{
        Self{
            image_base:ImageBase::new([1f32;4],[0f32,0f32,2f32*radius,2f32*radius]),
            texture:Texture::from_path(path,display).unwrap(),
            radius:radius/2f32,
            visible:true,
        }
    }

    pub fn set_position(&mut self,position:[f32;2]){
        let x=unsafe{position[0]-window_center[0]};
        let y=unsafe{window_center[1]-position[1]};

        self.image_base.x1=x-self.radius;
        self.image_base.y1=y+self.radius;
        self.image_base.x2=x+self.radius;
        self.image_base.y2=y-self.radius;
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
    pub fn pressed(&mut self){
        self.image_base.x1+=d_radius;
        self.image_base.y1-=d_radius;
        self.image_base.x2-=d_radius;
        self.image_base.y2+=d_radius;
        self.radius-=d_radius;
    }

    /// При освобождении кнопки мыши.
    /// 
    /// On a mouse button released.
    pub fn released(&mut self){
        self.image_base.x1-=d_radius;
        self.image_base.y1+=d_radius;
        self.image_base.x2+=d_radius;
        self.image_base.y2-=d_radius;
        self.radius+=d_radius;
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        if self.visible{
            self.image_base.draw(&self.texture,draw_parameters,graphics);
        }
    }
}