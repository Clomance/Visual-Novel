use crate::{
    colours::{Black,Light_blue},
};

use super::{
    TextView,
    TextViewSettings,
    GeneralSettings,
};

use cat_engine::{
    Colour,
    graphics::{
        Graphics2D,
        Graphics,
    },
    shapes::Rectangle,
};

pub struct Button{
    text:TextView,
    background:usize,
    /// [x1,y1,x2,y2]
    click_area:[f32;4],
    pressed:bool,
}

impl Button{
    pub fn new<S:Into<String>>(settings:ButtonSettings<S>,graphics:&mut Graphics2D)->Button{
        let layout=settings.general.layout;

        let rect=Rectangle::new(layout,settings.background_colour);

        let text_view_settings=TextViewSettings::new(settings.text,settings.general)
            .font_size(settings.font_size)
            .font(settings.font)
            .text_colour(settings.text_colour);

        let click_area=[
            layout[0],
            layout[1],
            layout[0]+layout[2],
            layout[1]+layout[3]
        ];

        Self{
            text:TextView::new(text_view_settings,graphics),
            background:graphics.add_simple_object(&rect).unwrap(),
            click_area,
            pressed:false
        }
    }

    pub fn background_index(&self)->usize{
        self.background
    }

    /// Проверяет находится ли точка в области кнопки.
    pub fn in_area(&self,x:f32,y:f32)->bool{
        let [x1,y1,x2,y2]=self.click_area;
        x>x1 && x<x2 && y>y1 && y<y2
    }

    /// Проверяет нажата ли кнопка.
    pub fn pressed(&mut self,x:f32,y:f32)->bool{
        self.pressed=self.in_area(x,y);
        self.pressed
    }

    /// Проверяет отпущена ли кнопка, которая была нажата.
    pub fn released(&mut self,x:f32,y:f32)->bool{
        if self.pressed{
            self.pressed=false;
            if self.in_area(x,y){
                true
            }
            else{
                false
            }
        }
        else{
            false
        }
    }

    pub fn draw(&self,graphics:&mut Graphics){
        graphics.draw_simple_object(self.background).unwrap();
        self.text.draw(graphics);
    }

    pub fn draw_shift(&self,shift:[f32;2],graphics:&mut Graphics){
        graphics.draw_shift_simple_object(self.background,shift).unwrap();
        self.text.draw_shift(shift,graphics);
    }
}

/// Настройки для построения кнопок
pub struct ButtonSettings<S:Into<String>>{
    general:GeneralSettings,
    background_colour:Colour,
    text:S,
    font_size:f32,
    font:usize,
    text_colour:Colour
}

impl<S:Into<String>> ButtonSettings<S>{
    pub fn new(text:S,rect:[f32;4])->ButtonSettings<S>{
        Self{
            general:GeneralSettings::new(rect),
            background_colour:Light_blue,
            text,
            font_size:20f32,
            font:0usize,
            text_colour:Black,
        }
    }

    pub fn background_colour(mut self,colour:Colour)->ButtonSettings<S>{
        self.background_colour=colour;
        self
    }

    pub fn font_size(mut self,size:f32)->ButtonSettings<S>{
        self.font_size=size;
        self
    }

    pub fn font(mut self,font:usize)->ButtonSettings<S>{
        self.font=font;
        self
    }
    
    pub fn text_colour(mut self,colour:Colour)->ButtonSettings<S>{
        self.text_colour=colour;
        self
    }
}