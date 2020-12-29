use crate::{
    Align,
    AlignX,
    colours::{
        Black,
        White,
    },
};

use super::{
    TextView,
    TextViewSettings,
    GeneralSettings,
};

use cat_engine::{
    // types
    Colour,
    // structs
    graphics::{Graphics,Graphics2D},
    shapes::Rectangle,
    text::RawGlyphCache,
};

// Изменяемый текстовый блок (возможность вписывать и удалять символы)
pub struct EditTextView{
    text_view:TextView,
    background:usize,
    click_area:[f32;4],
    capacity:usize,
    align:Align,
}

impl EditTextView{
    pub fn new<S:Into<String>>(settings:EditTextViewSettings<S>,graphics:&mut Graphics2D)->EditTextView{
        // Создание заднего фона
        let rect=settings.general.layout;

        let text_view_settings=TextViewSettings::new(settings.text,settings.general)
                .text_colour(settings.text_colour)
                .font(settings.font)
                .font_size(settings.font_size);

        let background=Rectangle::new(rect,settings.background_colour);
                //.border(2f32,settings.border_colour);

        let background=graphics.add_simple_object(&background).unwrap();

        let click_area=[
            rect[0],
            rect[1],
            rect[0]+rect[2],
            rect[1]+rect[3],
        ];

        Self{
            text_view:TextView::new(text_view_settings,graphics),
            background,
            click_area,
            capacity:settings.capacity,
            align:settings.align,
        }
    }

    pub fn in_area(&self,x:f32,y:f32)->bool{
        let [x1,y1,x2,y2]=self.click_area;

        x1<x && x2>x && y1<y && y2>y
    }

    pub fn text<'a>(&self,graphics:&'a mut Graphics2D)->&'a mut String{
        graphics.get_text_object_text(self.text_view.index())
    }

    /// Добавление символа с выравниванием.
    pub fn push_char<'a>(&mut self,ch:char,graphics:&'a mut Graphics2D){
        let font=*graphics.get_text_object_font(self.text_view.index());

        let scale=*graphics.get_text_object_scale(self.text_view.index());

        if graphics.get_text_object_text(self.text_view.index()).len()<self.capacity{
            graphics.get_text_object_text(self.text_view.index()).push(ch);
            
            let character_width=graphics.get_font(font).text_width(&ch.to_string(),scale);
            
            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f32,
                AlignX::Left=>0f32,
            };

            graphics.get_text_object_position(self.text_view.index())[0]-=dx; // Сдвиг по X
        }
    }

    /// Удаление последнего символа с выравниванием.
    pub fn pop_char<'a>(&mut self,graphics:&'a mut Graphics2D){
        if let Some(ch)=graphics.get_text_object_text(self.text_view.index()).pop(){
            let font=*graphics.get_text_object_font(self.text_view.index());

            let scale=*graphics.get_text_object_scale(self.text_view.index());

            let character_width=graphics.get_font(font).text_width(&ch.to_string(),scale);

            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f32,
                AlignX::Left=>0f32,
            };

            graphics.get_text_object_position(self.text_view.index())[0]+=dx; // Сдвиг по X
        }
    }

    pub fn draw(&self,graphics:&mut Graphics){
        graphics.draw_simple_object(self.background).unwrap();
        self.text_view.draw(graphics)
    }
}

pub struct EditTextViewSettings<S:Into<String>>{
    general:GeneralSettings,
    text:S,
    capacity:usize,
    font_size:f32,
    font:usize,
    text_colour:Colour,
    align:Align,
    background_colour:Colour,
    border_colour:Colour,
}

impl<S:Into<String>> EditTextViewSettings<S>{
    pub fn new(text:S,rect:[f32;4])->EditTextViewSettings<S>{
        Self{
            general:GeneralSettings::new(rect),
            text,
            capacity:20usize,
            font_size:20f32,
            font:0usize,
            text_colour:Black,
            align:Align::center(),
            background_colour:White,
            border_colour:Black
        }
    }

    pub fn background_colour(mut self,colour:Colour)->EditTextViewSettings<S>{
        self.background_colour=colour;
        self
    }

    pub fn border_colour(mut self,colour:Colour)->EditTextViewSettings<S>{
        self.border_colour=colour;
        self
    }

    pub fn align(mut self,align:Align)->EditTextViewSettings<S>{
        self.align=align;
        self
    }
}