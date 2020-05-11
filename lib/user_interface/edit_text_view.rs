use super::*;

// Изменяемый текстовый блок (возможность вписывать и удалять символы)
pub struct EditTextView<'a>{
    background:Rectangle,
    x1:f32,
    y1:f32,
    x2:f32,
    y2:f32,
    width:f32,
    height:f32,
    base:TextBase,
    line:String,
    capacity:usize,
    align:Align,
    glyphs:Glyphs<'a>,
}

impl<'a> EditTextView<'a>{
    pub fn new<S:Into<String>>(settings:EditTextViewSettings<S>,glyphs:Glyphs<'a>)->EditTextView<'a>{
        // Создание заднего фона
        let rect=settings.rect;
        let mut background=Rectangle::new(settings.background_color);
        if let Some(color)=settings.border_color{
            let border=graphics::rectangle::Border{
                color,
                radius:2f64,
            };
            background=background.border(border);
        }
        
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
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            width:rect[2],
            height:rect[3],
            base:TextBase::new(settings.text_color,settings.font_size).position([x,y]),
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

        self.x1<x && self.x2>x && self.y1<y && self.y2>y
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
        self.background.color[3]=alpha;
        self.background.border.as_mut().unwrap().color[3]=alpha;
    }

    fn draw(&mut self,context:&Context,graphics:&mut GameGraphics){
        let rect=[self.x1 as f64,self.y1 as f64,self.width as f64,self.height as f64];
        self.background.draw(rect,&context.draw_state,context.transform,graphics);
        self.base.draw(&self.line,context,graphics,&mut self.glyphs)
    }
}


pub struct EditTextViewSettings<S:Into<String>>{
    text:S,
    capacity:usize,
    font_size:f32,
    text_color:Color,
    align:Align,
    rect:[f32;4], // [x1,y1,width,height] - сюда вписывается текст
    background_color:Color,
    border_color:Option<Color>,
}

impl<S:Into<String>> EditTextViewSettings<S>{
    pub fn new(text:S,rect:[f32;4])->EditTextViewSettings<S>{
        Self{
            text,
            capacity:20usize,
            font_size:20f32,
            text_color:Black,
            align:Align::center(),
            rect,
            background_color:White,
            border_color:Some(Black)
        }
    }

    pub fn background_color(mut self,color:Color)->EditTextViewSettings<S>{
        self.background_color=color;
        self
    }

    pub fn border_color(mut self,color:Option<Color>)->EditTextViewSettings<S>{
        self.border_color=color;
        self
    }
} 