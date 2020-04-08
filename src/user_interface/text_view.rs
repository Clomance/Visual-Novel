use crate::*;

pub struct EditTextView<'a>{
    base:TextView<'a>,
    max_line_length:f64,
    rect:[f64;4],
    background:[Rectangle;2], // Первый - Заполненный прямоугольник, второй - его обводка
}

impl<'a> EditTextView<'a>{
    pub fn new(settings:EditTextViewSettings,glyphs:GlyphCache<'a>)->EditTextView<'a>{
        let rect=settings.base.rect;
        Self{
            base:TextView::new(settings.base,glyphs),
            max_line_length:rect[2]-20f64,
            rect:rect,
            background:[
                Rectangle::new(settings.background_color),
                Rectangle::new_border(settings.border_color,2f64),
            ],
        }
    }

    pub fn get_text(&self)->String{
        self.base.get_text()
    }

    pub fn push_text(&mut self,text:&str){
        let glyphs=&mut self.base.glyphs;

        let mut len=self.base.base.text_length; // Длина
        let mut dlen=0f64;

        for ch in text.chars(){
            let character=glyphs.character(self.base.base.base.font_size,ch).unwrap(); // Поиск нужной буквы
            len+=character.advance_width(); // Ширина буквы

            if len>=self.max_line_length{
                break
            }

            dlen+=character.advance_width();
            self.base.base.text.push(ch);
        }
        self.base.base.text_length+=dlen;
        self.base.base.base.image.rectangle.as_mut().unwrap()[0]-=dlen/2f64; // Сдвиг
    }

    pub fn pop_char(&mut self){
        let glyphs=&mut self.base.glyphs;
        if let Some(ch)=self.base.base.text.pop(){
            let character=glyphs.character(self.base.base.base.font_size,ch).unwrap(); // Поиск нужной буквы
            let len=character.advance_width(); // Ширина буквы

            self.base.base.base.image.rectangle.as_mut().unwrap()[0]+=len/2f64; // Сдвиг
            self.base.base.text_length-=len; // Уменьшение длины строки
        }
    }
}

impl<'a> Drawable for EditTextView<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
        self.background[0].color[3]=alpha;
        self.background[1].color[3]=alpha;
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.background[1].draw(self.rect,&context.draw_state,context.transform,graphics);
        self.background[0].draw(self.rect,&context.draw_state,context.transform,graphics);
        self.base.draw(context,graphics)
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
}

impl<'a> Drawable for TextView<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.base.draw(context,graphics,&mut self.glyphs);
    }
}

// Зависимый от шрифта текстовый блок
pub struct TextViewDependent{
    base:TextBase,
    text:String,
    text_length:f64,
}

impl TextViewDependent{
    pub fn new(settings:TextViewSettings,glyphs:&mut GlyphCache)->TextViewDependent{
        // Длина текста
        let mut text_len=0f64;
        for ch in settings.text.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        let x1=settings.rect[0]+(settings.rect[2]-text_len)/2f64;
        let y1=settings.rect[1]+(settings.rect[3]+settings.font_size as f64)/2f64;

        Self{
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x1,y1]),
            text_length:text_len,
            text:settings.text
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.color[3]=alpha;
    }

    pub fn draw(&mut self,context:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        self.base.draw(&self.text,context,g,glyphs);
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

    pub fn border_color(mut self,color:Color)->EditTextViewSettings{
        self.border_color=color;
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