use super::*;

// Изменяемый текстовый блок (возможность вписывать и удалять символы)
pub struct EditTextView<'a>{
    background:Rectangle,
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    width:f64,
    height:f64,
    base:TextBase,
    line:String,
    capacity:usize,
    align:Align,
    glyphs:GlyphCache<'a>,
}

impl<'a> EditTextView<'a>{
    pub fn new<S:ToString>(settings:EditTextViewSettings<S>,mut glyphs:GlyphCache<'a>)->EditTextView<'a>{
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
        
        let line=settings.text.to_string();
        // Вычисление длины строки текста
        let mut text_len=0f64;
        for ch in line.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        // Выравнивание текста
        let (x,y)=settings.align.text_position(settings.rect,[text_len,settings.font_size as f64]);

        Self{
            background,
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            width:rect[2],
            height:rect[3],
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x,y]),
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
            let character_width=self.glyphs.character(self.base.font_size,ch).unwrap().advance_width(); // Поиск нужной буквы
            
            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f64,
                AlignX::Left=>0f64,
            };
            self.base.image.rect[0]-=dx; // Сдвиг
        }
    }

    // Удаление последнего символа с выравниванием
    pub fn pop_char(&mut self){
        if let Some(ch)=self.line.pop(){
            let character=self.glyphs.character(self.base.font_size,ch).unwrap(); // Поиск нужной буквы
            let character_width=character.advance_width(); // Ширина буквы

            let dx=match self.align.x{
                AlignX::Right=>character_width,
                AlignX::Center=>character_width/2f64,
                AlignX::Left=>0f64,
            };
            self.base.image.rect[0]+=dx; // Сдвиг
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
        let rect=[self.x1,self.y1,self.width,self.height];
        self.background.draw(rect,&context.draw_state,context.transform,graphics);
        self.base.draw(&self.line,context,graphics,&mut self.glyphs)
    }
}


pub struct EditTextViewSettings<S:ToString>{
    text:S,
    capacity:usize,
    font_size:u32,
    text_color:Color,
    align:Align,
    rect:[f64;4], // [x1,y1,width,height] - сюда вписывается текст
    background_color:Color,
    border_color:Option<Color>,
}

impl<S:ToString> EditTextViewSettings<S>{
    pub fn new(text:S,rect:[f64;4])->EditTextViewSettings<S>{
        Self{
            text,
            capacity:20usize,
            font_size:20u32,
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