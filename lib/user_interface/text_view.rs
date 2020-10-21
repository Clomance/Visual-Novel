use crate::Drawable;

use super::{
    Align,
    AlignX,
    AlignY,
    Black,
    GeneralSettings,
};

use cat_engine::{
    Colour,
    text::{
        TextBase,
        RawGlyphCache,
    },
    graphics::{
        Graphics2D,
        DrawType,
        ObjectType,
    },
};

#[derive(Clone)]
pub struct TextView{
    index:usize,
    draw_type:DrawType,
}

impl TextView{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,graphics:&mut Graphics2D)->TextView{
        // Создаем строку текста
        let line=settings.text.into();

        let font=graphics.get_glyph_cache(settings.font);

        let scale=RawGlyphCache::scale_for_height(font, settings.font_size);

        let text_size=font.text_size(&line,scale);

        // Выравнивание
        let (x,y)=settings.align.text_position(settings.rect,text_size);

        let text_base=TextBase::new([x,y],scale,settings.text_colour);

        Self{
            index:graphics.add_text_object(line,&text_base,settings.font).unwrap(),
            draw_type:settings.general.draw_type
        }
    }
}

impl Drawable for TextView{
    fn index(&self)->usize{
        self.index
    }

    fn object_type(&self)->ObjectType{
        ObjectType::Text
    }

    fn draw_type(&self)->DrawType{
        self.draw_type.clone()
    }
}

#[derive(Clone)] // Настройки текстового поля
pub struct TextViewSettings<S:Into<String>>{
    general:GeneralSettings,
    rect:[f32;4], // [x1,y1,width,height] - сюда вписывается текст
    text:S,
    font_size:f32,
    font:usize,
    text_colour:Colour,
    align:Align,
}

impl<S:Into<String>> TextViewSettings<S>{
    pub fn new(text:S,rect:[f32;4])->TextViewSettings<S>{
        Self{
            general:GeneralSettings::new(),
            rect,
            text,
            font_size:20f32,
            font:0usize,
            text_colour:Black,
            align:Align::center()
        }
    }

    pub fn draw_type(mut self,draw_type:DrawType)->TextViewSettings<S>{
        self.general.draw_type=draw_type;
        self
    }

    pub fn font_size(mut self,size:f32)->TextViewSettings<S>{
        self.font_size=size;
        self
    }

    pub fn font(mut self,font:usize)->TextViewSettings<S>{
        self.font=font;
        self
    }

    pub fn text_colour(mut self,colour:Colour)->TextViewSettings<S>{
        self.text_colour=colour;
        self
    }

    pub fn align_x(mut self,align:AlignX)->TextViewSettings<S>{
        self.align.x=align;
        self
    }

    pub fn align_y(mut self,align:AlignY)->TextViewSettings<S>{
        self.align.y=align;
        self
    }
}