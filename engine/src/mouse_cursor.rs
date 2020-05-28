use super::{
    // statics
    window_center,
    // structs
    graphics::{GameGraphics},
    image::{ImageBase,Texture},
    // types
    Colour,
};

use glium::{
    Display,
    DrawParameters,
};

pub struct MouseCursor{
    position:[f32;2],
    saved_position:[f32;2],
}

impl MouseCursor{
    pub const fn new()->MouseCursor{
        Self{
            position:[0f32;2],
            saved_position:[0f32;2],
        }
    }

    #[inline(always)]
    pub fn position(&self)->[f32;2]{
        self.position
    }

    // Расстояние от курсора до центра экрана
    pub fn center_radius(&self)->[f32;2]{
        unsafe{[
            self.position[0]-window_center[0],
            self.position[1]-window_center[1]
        ]}
    }

    #[inline(always)] // Сохранение текущей позиции
    pub fn save_position(&mut self){
        self.saved_position=self.position;
    }

    // Сдвиг с сохранённого места
    pub fn saved_movement(&self)->(f32,f32){
        (
            self.position[0]-self.saved_position[0],
            self.position[1]-self.saved_position[1]
        )
    }

    #[inline(always)]
    pub fn set_position(&mut self,position:[f32;2]){
        self.position=position;
    }
}


// Иконка курсора мыши
// Сделана для прямого вывода на кадр
// Требуется доработка

const radius:f32=30f32;
const d_radius:f32=5f32;

// Иконка курсора мышки
// Загружает картинку их папки ресурсов
pub struct MouseCursorIcon{
    image_base:ImageBase,
    texture:Texture,
    radius:f32,
    visible:bool,
}

impl MouseCursorIcon{
    pub fn new(display:&Display)->MouseCursorIcon{
        Self{
            image_base:ImageBase::new([1f32;4],[0f32,0f32,radius,radius]),
            texture:Texture::from_path(display,"resources/images/mouse_icon.png").unwrap(),
            radius:radius/2f32,
            visible:true,
        }
    }

    pub fn set_position(&mut self,position:[f32;2]){
        self.image_base.x1=position[0]-self.radius;
        self.image_base.y1=position[1]-self.radius;
        self.image_base.x2=position[0]+self.radius;
        self.image_base.y2=position[1]+self.radius;
    }

    pub fn set_filter(&mut self,filter:Colour){
        self.image_base.colour_filter=filter;
    }

    pub fn set_visible(&mut self,visible:bool){
        self.visible=visible
    }

    pub fn switch_visibility(&mut self){
        self.visible=!self.visible
    }

    // При нажатии кнопки мыши
    pub fn pressed(&mut self){
        self.image_base.x1+=d_radius;
        self.image_base.y1+=d_radius;
        self.image_base.x2-=d_radius;
        self.image_base.y2-=d_radius;
        self.radius-=d_radius;
    }

    // При освобождении кнопки мыши
    pub fn released(&mut self){
        self.image_base.x1-=d_radius;
        self.image_base.y1-=d_radius;
        self.image_base.x2+=d_radius;
        self.image_base.y2+=d_radius;
        self.radius+=d_radius;
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        draw_parameters.color_mask=(self.visible,self.visible,self.visible,self.visible);
        self.image_base.draw(&self.texture,draw_parameters,graphics);
    }
}