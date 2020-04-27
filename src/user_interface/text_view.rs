use crate::*;

// Изменяемый текстовый блок (возможность вписывать и удалять символы)
pub struct EditTextView<'a>{
    text:String,
    base:TextBase,
    glyphs:GlyphCache<'a>,
    max_line_length:f64,
    text_length:f64,
    rect:[f64;4],
    background:Rectangle,
}

impl<'a> EditTextView<'a>{
    pub fn new(settings:EditTextViewSettings,mut glyphs:GlyphCache<'a>)->EditTextView<'a>{

        let rect=settings.base.rect;
        let border=graphics::rectangle::Border{
            color:settings.border_color,
            radius:2f64,
        };

        let mut text_len=0f64;
        for ch in settings.base.text.chars(){
            let character=glyphs.character(settings.base.font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        let (x,y)=settings.base.align.position(settings.base.rect,[text_len,settings.base.font_size as f64]);

        Self{
            text:settings.base.text,
            base:TextBase::new_color(settings.base.text_color,settings.base.font_size)
                    .position([x,y]),
            glyphs:glyphs,
            max_line_length:rect[2]-20f64,
            text_length:text_len,
            rect:rect,
            background:Rectangle::new(settings.background_color).border(border),
        }
    }

    pub fn clicked(&self)->bool{
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        self.rect[0]<x && self.rect[0]+self.rect[2]>x && self.rect[1]<y && self.rect[1]+self.rect[3]>y
    }

    pub fn get_text(&self)->String{
        self.text.clone()
    }

    // Добавление символа с выравниванием
    pub fn push_char(&mut self,ch:char){

        let character=self.glyphs.character(self.base.font_size,ch).unwrap(); // Поиск нужной буквы

        let mut len=self.text_length; // Исходная длина текста
        let dlen=character.advance_width(); // Ширина буквы
        len+=dlen; // Длина текста с вписанной буквой

        if len<self.max_line_length{
            self.text.push(ch);
            self.text_length+=dlen;
            self.base.image.rectangle.as_mut().unwrap()[0]-=dlen/2f64; // Сдвиг
        }
    }

    // Удаление последнего символа с выравниванием
    pub fn pop_char(&mut self){
        if let Some(ch)=self.text.pop(){
            let character=self.glyphs.character(self.base.font_size,ch).unwrap(); // Поиск нужной буквы
            let len=character.advance_width(); // Ширина буквы

            self.base.image.rectangle.as_mut().unwrap()[0]+=len/2f64; // Сдвиг
            self.text_length-=len; // Уменьшение длины строки
        }
    }
}

impl<'a> Drawable for EditTextView<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
        self.background.color[3]=alpha;
        self.background.border.as_mut().unwrap().color[3]=alpha;
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.background.draw(self.rect,&context.draw_state,context.transform,graphics);
        self.base.draw(&self.text,context,graphics,&mut self.glyphs)
    }
}

// Текстовый блок
pub struct TextView<'a,T:Text>{
    base:TextViewDependent<T>,
    glyphs:GlyphCache<'a>,
}

impl<'a> TextView<'a,TextLine>{
    pub fn new(settings:TextViewSettings,mut glyphs:GlyphCache<'a>)->TextView<'a,TextLine>{
        Self{
            base:TextViewDependent::new(settings,&mut glyphs),
            glyphs:glyphs
        }
    }

    pub fn get_text(&self)->String{
        self.base.text.get_text()
    }

    pub fn set_text_raw(&mut self,text:String){
        self.base.text.set_text(text);
    }
}

impl<'a,T:Text> Drawable for TextView<'a,T>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.base.draw(context,graphics,&mut self.glyphs);
    }
}

// Зависимый от шрифта текстовый блок
pub struct TextViewDependent<T:Text>{
    base:TextBase,
    text:T,
}

// Функции для одной строки текста
impl TextViewDependent<TextLine>{
    pub fn new(settings:TextViewSettings,glyphs:&mut GlyphCache)->TextViewDependent<TextLine>{
        // Длина текста
        let mut text_len=0f64;
        for ch in settings.text.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        // Выравнивание
        let (x,y)=settings.align.position(settings.rect,[text_len,settings.font_size as f64]);

        Self{
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x,y]),
            text:TextLine::new(settings.text),
        }
    }

    pub fn set_text_raw(&mut self,text:String){
        self.text.set_text(text)
    }
}

// Общие функции
impl<T:Text> TextViewDependent<T>{
    pub fn shift(&mut self,dx:f64,dy:f64){
        self.base.shift(dx,dy)
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
    }

    pub fn draw(&mut self,context:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        self.text.draw(&mut self.base,context,g,glyphs)
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
    pub text_color:Color,
    pub align:Align,
}

impl TextViewSettings{
    pub fn new()->TextViewSettings{
        Self{
            rect:[0f64;4],
            text:String::new(),
            font_size:20,
            text_color:Black,
            align:Align::center()
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

    pub fn align_x(mut self,align:AlignX)->TextViewSettings{
        self.align.x=align;
        self
    }

    pub fn align_y(mut self,align:AlignY)->TextViewSettings{
        self.align.y=align;
        self
    }
}