use super::{
    Align,
    AlignX,
    Black,
    Drawable,
    White
};

use engine::{
    // statics
    mouse_cursor,
    // types
    Colour,
    // structs
    graphics::{
        RectangleWithBorder,
        GameGraphics
    },
    text::{TextBase,Glyphs},
    glium::DrawParameters,
};

// Изменяемый текстовый блок (возможность вписывать и удалять символы)
pub struct EditTextView<'a>{
    background:RectangleWithBorder,
    base:TextBase,
    line:String,
    capacity:usize,
    align:Align,
    glyphs:&'a Glyphs,
}

impl<'a> EditTextView<'a>{
    pub fn new<S:Into<String>>(settings:EditTextViewSettings<S>,glyphs:&'a Glyphs)->EditTextView<'a>{
        // Создание заднего фона
        let rect=settings.rect;
        let mut background=RectangleWithBorder::new(rect,settings.background_colour);
        background=background.border(2f32,settings.border_colour);

        let line=settings.text.into();
        // Вычисление длины строки текста
        let mut text_len=0f32;
        for ch in line.chars(){
            let character=glyphs.character(ch,settings.font_size);
            text_len+=character.width();
        }

        // Выравнивание текста
        let (x,y)=settings.align.text_position(settings.rect,[text_len,settings.font_size]);

        Self{
            background,
            base:TextBase::new(settings.text_colour,settings.font_size).position([x,y]),
            line,
            capacity:settings.capacity,
            align:settings.align,
            glyphs:glyphs,
        }
    }

    pub fn clicked(&self)->bool{
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        self.background.rect.x1<x &&
         self.background.rect.x2>x &&
          self.background.rect.y1<y &&
           self.background.rect.y2>y
    }

    pub fn text(&mut self)->&mut String{
        &mut self.line
    }

    // Добавление символа с выравниванием
    pub fn push_char(&mut self,ch:char){
        if self.line.len()<self.capacity{
            self.line.push(ch);
            let character_width=self.glyphs.character(ch,self.base.font_size).width(); // Поиск нужной буквы
            
            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f32,
                AlignX::Left=>0f32,
            };
            self.base.shift_x(-dx); // Сдвиг
        }
    }

    // Удаление последнего символа с выравниванием
    pub fn pop_char(&mut self){
        if let Some(ch)=self.line.pop(){
            let character=self.glyphs.character(ch,self.base.font_size); // Поиск нужной буквы
            let character_width=character.width(); // Ширина буквы

            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f32,
                AlignX::Left=>0f32,
            };
            self.base.shift_x(dx); // Сдвиг
        }
    }
}

impl<'a> Drawable for EditTextView<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
        self.background.rect.colour[3]=alpha;
        self.background.border_colour[3]=alpha;
    }

    fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        self.background.draw(draw_parameters,graphics);
        self.base.draw(&self.line,draw_parameters,graphics,&self.glyphs)
    }
}


pub struct EditTextViewSettings<S:Into<String>>{
    text:S,
    capacity:usize,
    font_size:f32,
    text_colour:Colour,
    align:Align,
    rect:[f32;4], // [x1,y1,width,height] - сюда вписывается текст
    background_colour:Colour,
    border_colour:Colour,
}

impl<S:Into<String>> EditTextViewSettings<S>{
    pub fn new(text:S,rect:[f32;4])->EditTextViewSettings<S>{
        Self{
            text,
            capacity:20usize,
            font_size:20f32,
            text_colour:Black,
            align:Align::center(),
            rect,
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
} 