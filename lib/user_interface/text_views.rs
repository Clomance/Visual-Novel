use super::{
    Align,
    AlignX,
    AlignY,
    Black,
};

use engine::{
    // types
    Colour,
    // structs
    text::{TextBase,Glyphs},
    graphics::GameGraphics,
    glium::DrawParameters,
};


const line_margin:f32=20f32; // Расстояние между строками

// Изменяемый зависимый текстовой блок с одной линией текста
pub struct TextViewLineDependent{
    base:TextViewStaticLineDependent,
    rect:[f32;4],
    align:Align,
}

impl TextViewLineDependent{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,glyphs:&Glyphs)->TextViewLineDependent{
        Self{
            rect:settings.rect,
            align:settings.align.clone(),
            base:TextViewStaticLineDependent::new(settings,glyphs),
        }
    }

    pub fn font_size(&self)->f32{
        self.base.font_size()
    }

    pub fn set_text<S:Into<String>>(&mut self,text:S,glyphs:&Glyphs){
        self.base.line=text.into();

        let mut line_len=0f32;
        for ch in self.base.line.chars(){
            let character=glyphs.character(ch,self.font_size());
            line_len+=character.width();
        }

        let x=match self.align.x{
            AlignX::Right=>self.rect[0]+self.rect[2]-line_len,
            AlignX::Center=>self.rect[0]+(self.rect[2]-line_len)/2f32,
            AlignX::Left=>self.rect[0],
        };

        self.base.base.set_x(x);
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    pub fn shift(&mut self,dx:f32,dy:f32){
        self.base.shift(dx,dy)
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,g:&mut GameGraphics,glyphs:&Glyphs){
        self.base.base.draw(&self.base.line,draw_parameters,g,glyphs);
    }

    // Частичный вывод текста (Может пригодиться)
    // fn draw_part(&mut self,chars:usize,c:&Context,g:&mut GameGraphics,glyphs:&mut Glyphs)->bool{
    //     let (x,y)=(self.base.image.rect[0],self.base.image.rect[1]); // Сохранение начального положения

    //     let mut chars_passed=0; // Символов выведенно
    //     let mut whole_text=true;

    //     // Перебор символов
    //     for ch in self.line.chars(){
    //         if chars_passed==chars{
    //             whole_text=false;
    //             break
    //         }
    //         chars_passed+=1;
    //         let character=glyphs.character(self.base.font_size,ch).unwrap();

    //         { // Установка положения и размер символа
    //             self.base.image.rect[0]+=character.left();
    //             self.base.image.rect[1]-=character.top();
    //             self.base.image.rect[2]=character.atlas_size[0];
    //             self.base.image.rect[3]=character.atlas_size[1];
    //         }

    //         { // Обрезка символа
    //             self.base.image.src_rect[0]=character.atlas_offset[0];
    //             self.base.image.src_rect[1]=character.atlas_offset[1];
    //             self.base.image.src_rect[2]=character.atlas_size[0];
    //             self.base.image.src_rect[3]=character.atlas_size[1];
    //         }

    //         self.base.image.draw(character.texture,&c.draw_state,c.transform,g);

    //         // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
    //         self.base.image.rect[0]+=character.width() as f64-character.left();
    //         self.base.image.rect[1]+=character.advance_height()+character.top();
    //     }
    //     // Возвращение в начальное положение
    //     self.base.image.rect[0]=x;
    //     self.base.image.rect[1]=y;

    //     whole_text
    // }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,g:&mut GameGraphics,glyphs:&Glyphs){
        self.set_alpha_channel(alpha);
        self.draw(draw_parameters,g,glyphs)
    }
}

pub struct TextViewLinedDependent{
    base:TextViewStaticLinedDependent,
    rect:[f32;4],
    align:Align,
}

impl TextViewLinedDependent{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,glyphs:&Glyphs)->TextViewLinedDependent{
        Self{
            rect:settings.rect,
            align:settings.align.clone(),
            base:TextViewStaticLinedDependent::new(settings,glyphs)
        }
    }

    pub fn set_text<S:Into<String>>(&mut self,text:S,glyphs:&Glyphs){
        self.base.lines.clear(); // Удаление старого текста

        let font_size=self.base.base.font_size;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста
        
        let line_length=self.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f32; // Длина строки текста
        let mut word_len=0f32; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=glyphs.character(' ',self.base.base.font_size).width();
        let nl_whitespace_width=glyphs.character('\n',self.base.base.font_size).width();

        let text=text.into();

        for (c,ch) in text.char_indices(){

            let character=glyphs.character(ch,self.base.base.font_size);

            let char_width=character.width();
            line_len+=char_width;
            word_len+=char_width;

            if ch.is_whitespace(){
                word_len=0f32;
                last_whitespace=c;
            }

            if line_len>=line_length || ch=='\n'{
                if ch=='\n'{
                    line_len-=word_len+nl_whitespace_width;
                }
                else{
                    line_len-=word_len+whitespace_width;
                }

                if line_start==last_whitespace{
                    break // Если слово больше, чем длина строки
                }

                let line=text[line_start..last_whitespace].to_string();

                let pos=match self.align.x{
                    AlignX::Right=>line_length-line_len,
                    AlignX::Center=>(line_length-line_len)/2f32,
                    AlignX::Left=>0f32,
                };
                self.base.lines.push((pos,line));

                last_whitespace+=1;
                line_start=last_whitespace;
                
                line_len=word_len;

                height+=dline;
            }
        }

        let line=text[line_start..].to_string();
        let pos=match self.align.x{
            AlignX::Right=>line_length-line_len,
            AlignX::Center=>(line_length-line_len)/2f32,
            AlignX::Left=>0f32,
        };
        self.base.lines.push((pos,line));

        let x=self.rect[0];
        let y=self.rect[1]+match self.align.y{
            AlignY::Up=>font_size,
            AlignY::Center=>(self.rect[3]-height+font_size)/2f32,
            AlignY::Down=>self.rect[3]-height,
        };

        self.base.base.set_position([x,y]);
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics,glyphs:&Glyphs){
       self.base.draw(draw_parameters,graphics,glyphs)
    }

    // Вывод части текста
    pub fn draw_part(&mut self,chars:usize,draw_parameters:&DrawParameters,g:&mut GameGraphics,glyphs:&Glyphs)->bool{
        self.base.draw_part(chars,draw_parameters,g,glyphs)
    }
}

// Неизменяемый зависимый текстовый блок с одной линией текста
// Зависим от шрифта
pub struct TextViewStaticLineDependent{
    base:TextBase,
    line:String,
}

impl TextViewStaticLineDependent{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,glyphs:&Glyphs)->TextViewStaticLineDependent{
        let line=settings.text.into();

        let font=glyphs.glyph_height(settings.font_size);

        let mut line_len=0f32;
        for ch in line.chars(){
            let character=glyphs.character(ch,settings.font_size);
            line_len+=character.width();
        }

        // Выравнивание
        let (x,y)=settings.align.text_position(settings.rect,[line_len,font]);

        Self{
            base:TextBase::new(settings.text_colour,settings.font_size).position([x,y]),
            line:line
        }
    }

    pub fn font_size(&self)->f32{
        self.base.font_size
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    pub fn shift(&mut self,dx:f32,dy:f32){
        self.base.shift(dx,dy)
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,g:&mut GameGraphics,glyphs:&Glyphs){
        self.base.draw(&self.line,draw_parameters,g,glyphs);
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,g:&mut GameGraphics,glyphs:&Glyphs){
        self.set_alpha_channel(alpha);
        self.draw(draw_parameters,g,glyphs)
    }
}

// Неизменяемый зависимый текстовый блок с множеством линий текста
// Зависим от шрифта
pub struct TextViewStaticLinedDependent{
    base:TextBase,
    lines:Vec<(f32,String)>,
}

impl TextViewStaticLinedDependent{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,glyphs:&Glyphs)->TextViewStaticLinedDependent{
        let mut lines=Vec::new();

        let font_size=settings.font_size;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста
        
        let line_length=settings.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f32; // Длина строки текста
        let mut word_len=0f32; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=glyphs.character(' ',settings.font_size).width();
        let nl_whitespace_width=glyphs.character('\n',settings.font_size).width();

        let text=settings.text.into();

        for (c,ch) in text.char_indices(){

            let character=glyphs.character(ch,settings.font_size);

            let char_width=character.width();
            line_len+=char_width;
            word_len+=char_width;

            if ch.is_whitespace(){
                word_len=0f32;
                last_whitespace=c;
            }

            if line_len>=line_length || ch=='\n'{
                if ch=='\n'{
                    line_len-=word_len+nl_whitespace_width;
                }
                else{
                    line_len-=word_len+whitespace_width;
                }

                if line_start==last_whitespace{
                    break // Если слово больше, чем длина строки
                }

                let line=text[line_start..last_whitespace].to_string();

                let pos=match settings.align.x{
                    AlignX::Right=>line_length-line_len,
                    AlignX::Center=>(line_length-line_len)/2f32,
                    AlignX::Left=>0f32,
                };
                lines.push((pos,line));

                last_whitespace+=1;
                line_start=last_whitespace;
                
                line_len=word_len;

                height+=dline;
            }
        }

        let line=text[line_start..].to_string();
        let pos=match settings.align.x{
            AlignX::Right=>line_length-line_len,
            AlignX::Center=>(line_length-line_len)/2f32,
            AlignX::Left=>0f32,
        };
        lines.push((pos,line));

        let x=settings.rect[0];
        let y=settings.rect[1]+match settings.align.y{
            AlignY::Up=>font_size,
            AlignY::Center=>(settings.rect[3]-height+font_size)/2f32,
            AlignY::Down=>settings.rect[3]-height,
        };

        Self{
            base:TextBase::new(settings.text_colour,settings.font_size).position([x,y]),
            lines
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics,glyphs:&Glyphs){
        let position=self.base.position; // Сохранение начальной позиции

        let dy=self.base.font_size+line_margin;
        // Перебор строк
        for line in &self.lines{
            let dx=line.0; // Выравнивание строки
            self.base.shift_x(dx);

            self.base.draw(&line.1,draw_parameters,graphics,glyphs);

            self.base.shift(-dx,dy);
        }

        self.base.set_position(position);
    }

    // Вывод части текста
    pub fn draw_part(&mut self,chars:usize,draw_parameters:&DrawParameters,g:&mut GameGraphics,glyphs:&Glyphs)->bool{
        let mut position=[self.base.position[0] as f32,self.base.position[1] as f32];

        let dy=self.base.font_size+line_margin as f32;

        let mut chars_passed=0; // Символов выведенно

        let mut whole_text=true;

        // Перебор строк
        'lines:for line in &self.lines{
            position[0]+=line.0 as f32; // Сдвиг строки

            for ch in line.1.chars(){
                if chars_passed==chars{
                    whole_text=false;
                    break 'lines
                }
                chars_passed+=1;

                let character=glyphs.character_positioned(ch,self.base.font_size,position);
                self.base.draw_character(&character,draw_parameters,g);

                // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
                position[0]+=character.width();
            }

            // Переход на новую строку
            position[0]=self.base.position[0] as f32;
            position[1]+=dy;
        }

        whole_text
    }
}

#[derive(Clone)] // Настройки текстового поля
pub struct TextViewSettings<S:Into<String>>{
    rect:[f32;4], // [x1,y1,width,height] - сюда вписывается текст
    text:S,
    font_size:f32,
    text_colour:Colour,
    align:Align,
}

impl<S:Into<String>> TextViewSettings<S>{
    pub fn new(text:S,rect:[f32;4])->TextViewSettings<S>{
        Self{
            rect:rect,
            text:text,
            font_size:20f32,
            text_colour:Black,
            align:Align::center()
        }
    }

    pub fn font_size(mut self,size:f32)->TextViewSettings<S>{
        self.font_size=size;
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