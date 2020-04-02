use crate::*;


pub struct EditTextView<'a>{
    base:TextView<'a>,
    rect:[f64;4],
    background:[Rectangle;2], // Первый - Заполненный прямоугольник, второй - его обводка
}

impl<'a> EditTextView<'a>{
    pub fn new(settings:EditTextViewSettings,glyphs:GlyphCache<'a>)->EditTextView<'a>{
        let rect=settings.base.rect;
        Self{
            base:TextView::new(settings.base,glyphs),
            rect:rect,
            background:[
                Rectangle::new(settings.background_color),
                Rectangle::new_border(settings.border_color,1f64),
            ],
        }
    }

    pub fn get_text(&self)->String{
        self.base.get_text()
    }

    pub fn push_text(&mut self,text:&str){
        self.base.push_text(text)
    }

    pub fn pop_text(&mut self){
        self.base.pop_text()
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        self.background[0].draw(self.rect,draw_state,transform,g);
        self.background[1].draw(self.rect,draw_state,transform,g);
        self.base.draw(draw_state,transform,g)
    }
}



// Текстовый блок
pub struct TextView<'a>{
    base:TextViewDependent,
    glyphs:GlyphCache<'a>,
}

impl<'a> TextView<'a>{
    pub fn new(settings:TextViewSettings,mut glyphs:GlyphCache<'a>)->TextView<'a>{
        Self{
            base:TextViewDependent::new(settings,&mut glyphs),
            glyphs:glyphs
        }
    }

    pub fn get_text(&self)->String{
        self.base.text.clone()
    }

    pub fn push_text(&mut self,text:&str){
        let mut len=0f64;
        for ch in text.chars(){
            let character=self.glyphs.character(self.base.base.font_size,ch).unwrap(); // Поиск нужной буквы
            len+=character.advance_width(); // Ширина буквы
        }
        self.base.text.push_str(text);
        self.base.x1-=len/2f64; // Сдвиг
    }

    pub fn pop_text(&mut self){
        if let Some(ch)=self.base.text.pop(){
            let character=self.glyphs.character(self.base.base.font_size,ch).unwrap(); // Поиск нужной буквы
            let len=character.advance_width(); // Ширина буквы
            self.base.x1+=len/2f64; // Сдвиг
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        self.base.draw(draw_state,transform,g,&mut self.glyphs);
    }
}

// Зависимый от шрифта текстовый блок
pub struct TextViewDependent{
    pub x1:f64,
    y1:f64,
    base:Text,
    pub text:String,
}

impl TextViewDependent{
    pub fn new(settings:TextViewSettings,glyphs:&mut GlyphCache)->TextViewDependent{
        let mut text_len=0f64;
        for ch in settings.text.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        let x1=settings.rect[0]+(settings.rect[2]-text_len)/2f64;
        let y1=settings.rect[1]+(settings.rect[3]+settings.font_size as f64)/2f64;

        Self{
            x1:x1,
            y1:y1,
            base:Text::new_color(settings.text_color,settings.font_size),
            text:settings.text
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.color[3]=alpha;
    }

    // pub fn get_text_color_mut(&mut self)->&mut Color{
    //     &mut self.text_base.color
    // }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let x=self.x1;
        let y=self.y1;
        self.base.draw(&self.text,glyphs,draw_state,transform.trans(x,y),g);
    }
}

pub struct EditTextViewSettings{
    pub base:TextViewSettings,
    pub background_color:Color,
    pub border_color:Color,
}

impl EditTextViewSettings{
    pub fn new()->EditTextViewSettings{
        Self{
            base:TextViewSettings::new(),
            background_color:White,
            border_color:Black
        }
    }

    pub fn rect(mut self,rect:[f64;4])->EditTextViewSettings{
        self.base.rect=rect;
        self
    }

    pub fn text(mut self,text:String)->EditTextViewSettings{
        self.base.text=text;
        self
    }

    pub fn background_color(mut self,color:Color)->EditTextViewSettings{
        self.background_color=color;
        self
    }
}

#[derive(Clone)] // Настройки текстового поля
pub struct TextViewSettings{
    pub rect:[f64;4], // [x1,y1,width,height] - сюда вписывается текст
    pub text:String,
    pub font_size:u32,
    pub text_color:Color
}

impl TextViewSettings{
    pub fn new()->TextViewSettings{
        Self{
            rect:[0f64;4],
            text:String::new(),
            font_size:20,
            text_color:Black,
        }
    }

    pub fn rect(mut self,rect:[f64;4])->TextViewSettings{
        self.rect=rect;
        self
    }

    pub fn text(mut self,text:String)->TextViewSettings{
        self.text=text;
        self
    }

    pub fn font_size(mut self,size:u32)->TextViewSettings{
        self.font_size=size;
        self
    }

    pub fn text_color(mut self,color:Color)->TextViewSettings{
        self.text_color=color;
        self
    }
}