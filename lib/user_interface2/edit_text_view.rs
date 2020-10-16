use super::{
    Align,
    AlignX,
    Black,
    Drawable,
    White
};

use cat_engine::{
    // statics
    mouse_cursor,
    // types
    Colour,
    // structs
    graphics::Graphics,
    shapes::RectangleWithBorder,
    text::{
        char_width,
        text_size,
        TextBase,
        rusttype::Font
    },
    glium::DrawParameters,
};

// Изменяемый текстовый блок (возможность вписывать и удалять символы)
pub struct EditTextView<'a>{
    background:RectangleWithBorder,
    base:TextBase,
    line:String,
    capacity:usize,
    align:Align,
    font:&'a Font<'static>,
}

impl<'a> EditTextView<'a>{
    pub fn new<S:Into<String>>(settings:EditTextViewSettings<S>,font:&'a Font<'static>)->EditTextView<'a>{
        // Создание заднего фона
        let rect=settings.rect;
        let mut background=RectangleWithBorder::new(rect,settings.background_colour);
        background=background.border(2f32,settings.border_colour);

        let line=settings.text.into();
        let size=text_size(&line,settings.font_size,font);

        // Выравнивание текста
        let (x,y)=settings.align.text_position(settings.rect,size);

        Self{
            background,
            base:TextBase::new([x,y],settings.font_size,settings.text_colour),
            line,
            capacity:settings.capacity,
            align:settings.align,
            font:font,
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
            let character_width=char_width(ch,self.base.font_size,self.font);
            
            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f32,
                AlignX::Left=>0f32,
            };

            self.base.position[0]-=dx; // Сдвиг по X
        }
    }

    // Удаление последнего символа с выравниванием
    pub fn pop_char(&mut self){
        if let Some(ch)=self.line.pop(){
            let character_width=char_width(ch,self.base.font_size,self.font);

            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f32,
                AlignX::Left=>0f32,
            };

            self.base.position[0]+=dx; // Сдвиг по X
        }
    }
}

impl<'a> Drawable for EditTextView<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.colour[3]=alpha;
        self.background.rect.colour[3]=alpha;
        self.background.border_colour[3]=alpha;
    }

    fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.background.draw(draw_parameters,graphics);
        self.base.draw_str(
            &self.line,
            &self.font,
            draw_parameters,
            graphics
        ).unwrap();
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