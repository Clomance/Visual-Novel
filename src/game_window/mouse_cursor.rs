use crate::*;

const common_color:Color=[0.2,0.3,0.9,0.8];
const pressed_color:Color=[0.1,0.2,0.8,0.8];


const common_radius:f64=16f64;
const pressed_radius:f64=12f64;

const d_radius:f64=common_radius-pressed_radius;

const common_diametr:f64=common_radius*2f64;

pub struct MouseCursor{
    radius:f64,
    position:[f64;2],
    saved_position:[f64;2],
    rect:[f64;4],
    cursor:Ellipse
}

impl MouseCursor{
    pub const fn new()->MouseCursor{
        Self{
            radius:common_radius,
            position:[0f64;2],
            saved_position:[0f64;2],
            rect:[0f64,0f64,common_diametr,common_diametr],
            cursor:Ellipse{
                color:common_color,
                border:None,
                resolution:360, // Количесво углов
            }
        }
    }

    #[inline]
    pub fn position(&self)->[f64;2]{
        self.position
    }

    // Расстояние от курсора до центра экрана
    pub fn center_radius(&self)->[f64;2]{
        unsafe{[
            self.position[0]-window_center[0],
            self.position[1]-window_center[1]
        ]}
    }

    #[inline]
    pub fn save_position(&mut self){
        self.saved_position=self.position;
    }

    // Сдвиг с сохранённого места
    pub fn saved_movement(&self)->(f64,f64){
        (
            self.position[0]-self.saved_position[0],
            self.position[1]-self.saved_position[1]
        )
    }

    pub fn set_position(&mut self,position:[f64;2]){
        self.position=position;
        self.rect[0]=position[0]-self.radius;
        self.rect[1]=position[1]-self.radius;
    }

    // При нажатии левой кнопки мыши
    pub fn pressed(&mut self){
        self.cursor.color=pressed_color;
        self.radius=pressed_radius;

        self.rect[0]+=d_radius;
        self.rect[1]+=d_radius;
        self.rect[2]-=d_radius*2f64;
        self.rect[3]-=d_radius*2f64;
    }

    // При освобождении левой кнопки мыши
    pub fn released(&mut self){
        self.cursor.color=common_color;
        self.radius=common_radius;

        self.rect[0]-=d_radius;
        self.rect[1]-=d_radius;
        self.rect[2]+=d_radius*2f64;
        self.rect[3]+=d_radius*2f64;
    }

    pub fn draw(&self,context:&Context,graphics:&mut GlGraphics){
        self.cursor.draw(self.rect,&context.draw_state,context.transform,graphics)
    }
}
