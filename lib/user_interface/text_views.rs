use super::*;

const line_margin:f64=20f64; // Расстояние между строками

// Изменяемый зависимый текстовой блок с одной линией текста
pub struct TextViewLineDependent{
    base:TextBase,
    line:String,
    rect:[f64;4],
    align:Align,
}

impl TextViewLineDependent{
    pub fn new<S:ToString>(settings:TextViewSettings<S>,glyphs:&mut GlyphCache)->TextViewLineDependent{
        let line=settings.text.to_string();

        let mut line_len=0f64;
        for ch in line.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            line_len+=character.advance_width();
        }

        // Выравнивание
        let (x,y)=settings.align.text_position(settings.rect,[line_len,settings.font_size as f64]);

        Self{
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x,y]),
            line:line,
            rect:settings.rect,
            align:settings.align
        }
    }

    pub fn set_text<S:ToString>(&mut self,text:S,glyphs:&mut GlyphCache){
        self.line=text.to_string();
        let mut line_len=0f64;
        for ch in self.line.chars(){
            let character=glyphs.character(self.base.font_size,ch).unwrap();
            line_len+=character.advance_width();
        }

        let x=match self.align.x{
            AlignX::Right=>self.rect[0]+self.rect[2]-line_len,
            AlignX::Center=>self.rect[0]+(self.rect[2]-line_len)/2f64,
            AlignX::Left=>self.rect[0],
        };

        self.base.set_x(x);
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        self.base.shift(dx,dy)
    }

    pub fn draw(&mut self,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache){
        let (x,y)=(self.base.image.rect[0],self.base.image.rect[1]); // Сохранение начального положения
        // Перебор символов
        for ch in self.line.chars(){
            let character=glyphs.character(self.base.font_size,ch).unwrap();

            { // Установка положения и размер символа
                self.base.image.rect[0]+=character.left();
                self.base.image.rect[1]-=character.top();
                self.base.image.rect[2]=character.atlas_size[0];
                self.base.image.rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                self.base.image.src_rect[0]=character.atlas_offset[0];
                self.base.image.src_rect[1]=character.atlas_offset[1];
                self.base.image.src_rect[2]=character.atlas_size[0];
                self.base.image.src_rect[3]=character.atlas_size[1];
            }

            self.base.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
            self.base.image.rect[0]+=character.advance_width()-character.left();
            self.base.image.rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        self.base.image.rect[0]=x;
        self.base.image.rect[1]=y;
    }

    // Частичный вывод текста (Может пригодиться)
    // fn draw_part(&mut self,chars:usize,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache)->bool{
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
    //         self.base.image.rect[0]+=character.advance_width()-character.left();
    //         self.base.image.rect[1]+=character.advance_height()+character.top();
    //     }
    //     // Возвращение в начальное положение
    //     self.base.image.rect[0]=x;
    //     self.base.image.rect[1]=y;

    //     whole_text
    // }

    pub fn draw_smooth(&mut self,alpha:f32,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache){
        self.set_alpha_channel(alpha);
        self.draw(c,g,glyphs)
    }
}

pub struct TextViewLinedDependent{
    base:TextBase,
    lines:Vec<(f64,String)>,
    rect:[f64;4],
    align:Align,
}

impl TextViewLinedDependent{
    pub fn new<S:ToString>(settings:TextViewSettings<S>,glyphs:&mut GlyphCache)->TextViewLinedDependent{
        let mut lines=Vec::new();

        let font_size=settings.font_size as f64;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста
        
        let line_length=settings.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f64; // Длина строки текста
        let mut word_len=0f64; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=glyphs.character(settings.font_size,' ').unwrap().advance_width();
        let nl_whitespace_width=glyphs.character(settings.font_size,'\n').unwrap().advance_width();

        let text=settings.text.to_string();

        for (c,ch) in text.char_indices(){

            let character=glyphs.character(settings.font_size,ch).unwrap();

            let char_width=character.advance_width();
            line_len+=char_width;
            word_len+=char_width;

            if ch.is_whitespace(){
                word_len=0f64;
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
                    AlignX::Center=>(line_length-line_len)/2f64,
                    AlignX::Left=>0f64,
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
            AlignX::Center=>(line_length-line_len)/2f64,
            AlignX::Left=>0f64,
        };
        lines.push((pos,line));

        let x=settings.rect[0];
        let y=settings.rect[1]+match settings.align.y{
            AlignY::Up=>font_size,
            AlignY::Center=>(settings.rect[3]-height+font_size)/2f64,
            AlignY::Down=>settings.rect[3]-height,
        };

        Self{
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x,y]),
            lines,
            rect:settings.rect,
            align:settings.align
        }
    }

    pub fn set_text<S:ToString>(&mut self,text:S,glyphs:&mut GlyphCache){
        self.lines.clear(); // Удаление старого текста

        let font_size=self.base.font_size as f64;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста
        
        let line_length=self.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f64; // Длина строки текста
        let mut word_len=0f64; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=glyphs.character(self.base.font_size,' ').unwrap().advance_width();
        let nl_whitespace_width=glyphs.character(self.base.font_size,'\n').unwrap().advance_width();

        let text=text.to_string();

        for (c,ch) in text.char_indices(){

            let character=glyphs.character(self.base.font_size,ch).unwrap();

            let char_width=character.advance_width();
            line_len+=char_width;
            word_len+=char_width;

            if ch.is_whitespace(){
                word_len=0f64;
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
                    AlignX::Center=>(line_length-line_len)/2f64,
                    AlignX::Left=>0f64,
                };
                self.lines.push((pos,line));

                last_whitespace+=1;
                line_start=last_whitespace;
                
                line_len=word_len;

                height+=dline;
            }
        }

        let line=text[line_start..].to_string();
        let pos=match self.align.x{
            AlignX::Right=>line_length-line_len,
            AlignX::Center=>(line_length-line_len)/2f64,
            AlignX::Left=>0f64,
        };
        self.lines.push((pos,line));

        let x=self.rect[0];
        let y=self.rect[1]+match self.align.y{
            AlignY::Up=>font_size,
            AlignY::Center=>(self.rect[3]-height+font_size)/2f64,
            AlignY::Down=>self.rect[3]-height,
        };

        self.base.set_position([x,y]);
    }

    pub fn draw(&mut self,context:&Context,graphics:&mut GameGraphics,glyphs:&mut GlyphCache){
        let (x,y)=(self.base.image.rect[0],self.base.image.rect[1]); // Сохранение начального положения

        let dy=self.base.font_size as f64+line_margin;

        // Перебор строк
        for line in &self.lines{
            let dx=line.0; // Выравнивание строки
            self.base.image.rect[0]+=dx; // Сдвиг строки

            for ch in line.1.chars(){
                let character=glyphs.character(self.base.font_size,ch).unwrap();

                { // Установка положения и размер символа
                    self.base.image.rect[0]+=character.left();
                    self.base.image.rect[1]-=character.top();
                    self.base.image.rect[2]=character.atlas_size[0];
                    self.base.image.rect[3]=character.atlas_size[1];
                }

                { // Обрезка символа
                    self.base.image.src_rect[0]=character.atlas_offset[0];
                    self.base.image.src_rect[1]=character.atlas_offset[1];
                    self.base.image.src_rect[2]=character.atlas_size[0];
                    self.base.image.src_rect[3]=character.atlas_size[1];
                }

                self.base.image.draw(character.texture,&context.draw_state,context.transform,graphics);

                // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
                self.base.image.rect[0]+=character.advance_width()-character.left();
                self.base.image.rect[1]+=character.advance_height()+character.top();
            }

            // Переход на новую строку
            self.base.image.rect[0]=x;
            self.base.image.rect[1]+=dy;
        }
        // Возвращение в начальное положение
        self.base.image.rect[0]=x;
        self.base.image.rect[1]=y;
    }

    pub fn draw_part(&mut self,chars:usize,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache)->bool{
        // Сохранение начального положения
        let (x,y)=(self.base.image.rect[0],self.base.image.rect[1]); // Сохранение начального положения

        let dy=self.base.font_size as f64+line_margin;

        let mut chars_passed=0; // Символов выведенно

        let mut whole_text=true;

        // Перебор строк
        'lines:for line in &self.lines{
            let dx=line.0; // Выравнивание строки
            self.base.image.rect[0]+=dx; // Сдвиг строки

            for ch in line.1.chars(){
                if chars_passed==chars{
                    whole_text=false;
                    break 'lines
                }
                chars_passed+=1;

                let character=glyphs.character(self.base.font_size,ch).unwrap();

                { // Установка положения и размер символа
                    self.base.image.rect[0]+=character.left();
                    self.base.image.rect[1]-=character.top();
                    self.base.image.rect[2]=character.atlas_size[0];
                    self.base.image.rect[3]=character.atlas_size[1];
                }

                { // Обрезка символа
                    self.base.image.src_rect[0]=character.atlas_offset[0];
                    self.base.image.src_rect[1]=character.atlas_offset[1];
                    self.base.image.src_rect[2]=character.atlas_size[0];
                    self.base.image.src_rect[3]=character.atlas_size[1];
                }

                // Вывод символа
                self.base.image.draw(character.texture,&c.draw_state,c.transform,g);

                // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
                self.base.image.rect[0]+=character.advance_width()-character.left();
                self.base.image.rect[1]+=character.advance_height()+character.top();
            }

            // Переход на новую строку
            self.base.image.rect[0]=x;
            self.base.image.rect[1]+=dy;
        }
        // Возвращение в начальное положение
        self.base.image.rect[0]=x;
        self.base.image.rect[1]=y;

        whole_text
    }
}

// Неизменяемый зависимый текстовый блок с одной линией текста
// Зависим от шрифта
pub struct TextViewStaticLineDependent{
    base:TextBase,
    line:String,
}

impl TextViewStaticLineDependent{
    pub fn new<S:ToString>(settings:TextViewSettings<S>,glyphs:&mut GlyphCache)->TextViewStaticLineDependent{
        let line=settings.text.to_string();

        let mut line_len=0f64;
        for ch in line.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            line_len+=character.advance_width();
        }

        // Выравнивание
        let (x,y)=settings.align.text_position(settings.rect,[line_len,settings.font_size as f64]);

        Self{
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x,y]),
            line:line
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        self.base.shift(dx,dy)
    }

    pub fn draw(&mut self,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache){
        let (x,y)=(self.base.image.rect[0],self.base.image.rect[1]); // Сохранение начального положения

        // Перебор символов
        for ch in self.line.chars(){
            let character=glyphs.character(self.base.font_size,ch).unwrap();

            { // Установка положения и размер символа
                self.base.image.rect[0]+=character.left();
                self.base.image.rect[1]-=character.top();
                self.base.image.rect[2]=character.atlas_size[0];
                self.base.image.rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                self.base.image.src_rect[0]=character.atlas_offset[0];
                self.base.image.src_rect[1]=character.atlas_offset[1];
                self.base.image.src_rect[2]=character.atlas_size[0];
                self.base.image.src_rect[3]=character.atlas_size[1];
            }

            self.base.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
            self.base.image.rect[0]+=character.advance_width()-character.left();
            self.base.image.rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        self.base.image.rect[0]=x;
        self.base.image.rect[1]=y;
    }

    pub fn draw_smooth(&mut self,alpha:f32,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache){
        self.set_alpha_channel(alpha);
        self.draw(c,g,glyphs)
    }
}

// Неизменяемый зависимый текстовый блок с множеством линий текста
// Зависим от шрифта
pub struct TextViewStaticLinedDependent{
    base:TextBase,
    lines:Vec<(f64,String)>,
}

impl TextViewStaticLinedDependent{
    pub fn new<S:ToString>(settings:TextViewSettings<S>,glyphs:&mut GlyphCache)->TextViewStaticLinedDependent{
        let mut lines=Vec::new();

        let font_size=settings.font_size as f64;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста
        
        let line_length=settings.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f64; // Длина строки текста
        let mut word_len=0f64; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=glyphs.character(settings.font_size,' ').unwrap().advance_width();
        let nl_whitespace_width=glyphs.character(settings.font_size,'\n').unwrap().advance_width();

        let text=settings.text.to_string();

        for (c,ch) in text.char_indices(){

            let character=glyphs.character(settings.font_size,ch).unwrap();

            let char_width=character.advance_width();
            line_len+=char_width;
            word_len+=char_width;

            if ch.is_whitespace(){
                word_len=0f64;
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
                    AlignX::Center=>(line_length-line_len)/2f64,
                    AlignX::Left=>0f64,
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
            AlignX::Center=>(line_length-line_len)/2f64,
            AlignX::Left=>0f64,
        };
        lines.push((pos,line));

        let x=settings.rect[0];
        let y=settings.rect[1]+match settings.align.y{
            AlignY::Up=>font_size,
            AlignY::Center=>(settings.rect[3]-height+font_size)/2f64,
            AlignY::Down=>settings.rect[3]-height,
        };

        Self{
            base:TextBase::new_color(settings.text_color,settings.font_size).position([x,y]),
            lines
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
    }

    pub fn draw(&mut self,context:&Context,graphics:&mut GameGraphics,glyphs:&mut GlyphCache){
        let (x,y)=(self.base.image.rect[0],self.base.image.rect[1]); // Сохранение начального положения

        let dy=self.base.font_size as f64+line_margin;

        // Перебор строк
        for line in &self.lines{
            let dx=line.0; // Выравнивание строки
            self.base.image.rect[0]+=dx; // Сдвиг строки

            for ch in line.1.chars(){
                let character=glyphs.character(self.base.font_size,ch).unwrap();

                { // Установка положения и размер символа
                    self.base.image.rect[0]+=character.left();
                    self.base.image.rect[1]-=character.top();
                    self.base.image.rect[2]=character.atlas_size[0];
                    self.base.image.rect[3]=character.atlas_size[1];
                }

                { // Обрезка символа
                    self.base.image.src_rect[0]=character.atlas_offset[0];
                    self.base.image.src_rect[1]=character.atlas_offset[1];
                    self.base.image.src_rect[2]=character.atlas_size[0];
                    self.base.image.src_rect[3]=character.atlas_size[1];
                }

                self.base.image.draw(character.texture,&context.draw_state,context.transform,graphics);

                // Сдвиг дальше вдоль горизонтальной линии и выравнивае по горизонтали
                self.base.image.rect[0]+=character.advance_width()-character.left();
                self.base.image.rect[1]+=character.advance_height()+character.top();
            }

            // Переход на новую строку
            self.base.image.rect[0]=x;
            self.base.image.rect[1]+=dy;
        }
        // Возвращение в начальное положение
        self.base.image.rect[0]=x;
        self.base.image.rect[1]=y;
    }
}

#[derive(Clone)] // Настройки текстового поля
pub struct TextViewSettings<S:ToString>{
    rect:[f64;4], // [x1,y1,width,height] - сюда вписывается текст
    text:S,
    font_size:u32,
    text_color:Color,
    align:Align,
}

impl<S:ToString> TextViewSettings<S>{
    pub fn new(text:S,rect:[f64;4])->TextViewSettings<S>{
        Self{
            rect:rect,
            text:text,
            font_size:20,
            text_color:Black,
            align:Align::center()
        }
    }

    pub fn font_size(mut self,size:u32)->TextViewSettings<S>{
        self.font_size=size;
        self
    }

    pub fn text_color(mut self,color:Color)->TextViewSettings<S>{
        self.text_color=color;
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