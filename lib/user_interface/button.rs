use crate::{
    Drawable,
    Clickable,
    GeneralSettings,
};

use super::{
    Black,
    Light_blue,
    TextViewSettings,
    TextView,
};

use cat_engine::{
    // types
    Colour,
    // structs
    graphics::{
        DrawType,
        ObjectType,
        Graphics2D
    },
    shapes::Rectangle,
};

pub struct Button{
    pub text:TextView,
    index:usize,
    draw_type:DrawType,
    click_area:[f32;4],
}

impl Button{
    pub fn new<S:Into<String>>(settings:ButtonSettings<S>,graphics:&mut Graphics2D)->Button{

        let text_view_settings=TextViewSettings::new(settings.text,settings.rect)
            .draw_type(settings.general.draw_type.clone())
            .font_size(settings.font_size)
            .font(settings.font)
            .text_colour(settings.text_colour);

        let rect=Rectangle::new(settings.rect,settings.background_colour);

        Self{
            text:TextView::new(text_view_settings,graphics),
            index:graphics.add_simple_object(&rect).unwrap(),
            draw_type:settings.general.draw_type,
            click_area:settings.rect
        }
    }
}

impl Clickable for Button{
    fn area(&self)->[f32;4]{
        self.click_area
    }
}

impl Drawable for Button{
    fn index(&self)->usize{
        self.index
    }

    fn draw_type(&self)->DrawType{
        self.draw_type.clone()
    }

    fn object_type(&self)->ObjectType{
        ObjectType::Simple
    }
}

/// Настройки для построения кнопок
/// 
/// Settings for building buttons
pub struct ButtonSettings<S:Into<String>>{
    general:GeneralSettings,
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
            general:GeneralSettings::new(),
            rect,
            background_colour:Light_blue,
            text,
            font_size:20f32,
            font:0usize,
            text_colour:Black,
        }
    }

    pub fn draw_type(mut self,draw_type:DrawType)->ButtonSettings<S>{
        self.general.draw_type=draw_type;
        self
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