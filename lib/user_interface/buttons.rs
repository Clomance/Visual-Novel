use crate::{
    colours::{Black,Light_blue},
};

use super::{
    TextView,
    TextViewSettings,
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
    index:usize,
    //click_area:[f32;4],
}

impl Button{
    pub fn new<S:Into<String>>(settings:ButtonSettings<S>,graphics:&mut Graphics2D)->Button{
        let text_view_settings=TextViewSettings::new(settings.text,settings.rect)
            .font_size(settings.font_size)
            .font(settings.font)
            .text_colour(settings.text_colour);

        let rect=Rectangle::new(settings.rect,settings.background_colour);

        Self{
            text:TextView::new(text_view_settings,graphics),
            index:graphics.add_simple_object(&rect).unwrap(),
            //click_area:settings.rect
        }
    }

    pub fn draw(&self,graphics:&mut Graphics){
        graphics.draw_simple_object(self.index).unwrap();
        self.text.draw(graphics);
    }

    pub fn draw_shift(&self,shift:[f32;2],graphics:&mut Graphics){
        graphics.draw_shift_simple_object(self.index,shift).unwrap();
        self.text.draw_shift(shift,graphics);
    }
}

/// Настройки для построения кнопок
pub struct ButtonSettings<S:Into<String>>{
    rect:[f32;4],
    background_colour:Colour,
    text:S,
    font_size:f32,
    font:usize,
    text_colour:Colour
}

impl<S:Into<String>> ButtonSettings<S>{
    pub fn new(text:S,rect:[f32;4])->ButtonSettings<S>{
        Self{
            rect,
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