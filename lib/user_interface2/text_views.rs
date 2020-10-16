use super::{
    Align,
    AlignX,
    AlignY,
    Black,
};

use cat_engine::{
    // types
    Colour,
    // structs
    text::{
        char_width,
        text_width,
        text_size,
        TextBase,
        rusttype::{Font,Point,Scale}
    },
    graphics::{Graphics,Graphics2D},
    glium::DrawParameters,
};


const line_margin:f32=20f32; // Расстояние между строками

// Изменяемый зависимый текстовой блок с одной линией текста
pub struct TextViewLine<'a>{
    base:TextViewStaticLine<'a>,
    rect:[f32;4],
    align:Align,
}

impl<'a> TextViewLine<'a>{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,font:&'a Font<'static>)->TextViewLine<'a>{
        Self{
            rect:settings.rect,
            align:settings.align.clone(),
            base:TextViewStaticLine::new(settings,font),
        }
    }

    #[inline(always)]
    pub fn font_size(&self)->f32{
        self.base.font_size()
    }

    pub fn set_text<S:Into<String>>(&mut self,text:S,font:&Font<'static>){
        self.base.line=text.into();

        let line_len=text_width(&self.base.line,self.base.base.font_size,font);

        let x=match self.align.x{
            AlignX::Right=>self.rect[0]+self.rect[2]-line_len,
            AlignX::Center=>self.rect[0]+(self.rect[2]-line_len)/2f32,
            AlignX::Left=>self.rect[0],
        };

        self.base.base.set_x(x);
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.base.shift(dx,dy)
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,g:&mut Graphics,font:&Font<'static>){
        self.base.base.draw_str(&self.base.line,font,draw_parameters,g);
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,g:&mut Graphics,font:&Font<'static>){
        self.set_alpha_channel(alpha);
        self.draw(draw_parameters,g,font)
    }
}

pub struct TextViewLined<'a>{
    base:TextViewStaticLined<'a>,
    rect:[f32;4],
    align:Align,
}

impl<'a> TextViewLined<'a>{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,font:&'a Font<'static>)->TextViewLined<'a>{
        Self{
            rect:settings.rect,
            align:settings.align.clone(),
            base:TextViewStaticLined::new(settings,font)
        }
    }

    pub fn set_text<S:Into<String>>(&mut self,text:S){
        self.base.lines.clear(); // Удаление старого текста

        let font_size=self.base.base.font_size;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста

        let line_length=self.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f32; // Длина строки текста
        let mut word_len=0f32; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=char_width(' ',self.base.base.font_size,self.base.font);
        let nl_whitespace_width=char_width('\n',self.base.base.font_size,self.base.font);

        let text=text.into();

        for (c,ch) in text.char_indices(){

            let char_width=char_width(ch,self.base.base.font_size,self.base.font);
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

        self.base.base.move_to([x,y]);
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
       self.base.draw(draw_parameters,graphics)
    }

    // Вывод части текста
    pub fn draw_part(&mut self,chars:usize,draw_parameters:&mut DrawParameters,g:&mut Graphics)->bool{
        self.base.draw_part(chars,draw_parameters,g)
    }
}

// Неизменяемый зависимый текстовый блок с одной линией текста
// Зависим от шрифта
pub struct TextViewStaticLine<'a>{
    base:TextBase,
    line:String,
    font:&'a Font<'static>
}

impl<'a> TextViewStaticLine<'a>{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,font:&'a Font<'static>)->TextViewStaticLine<'a>{
        let line=settings.text.into();

        let text_size=text_size(&line,settings.font_size,font);

        // Выравнивание
        let (x,y)=settings.align.text_position(settings.rect,text_size);

        Self{
            base:TextBase::new([x,y],settings.font_size,settings.text_colour),
            line:line,
            font:font
        }
    }

    #[inline(always)]
    pub fn font_size(&self)->f32{
        self.base.font_size
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha)
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.base.shift(dx,dy)
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,g:&mut Graphics){
        self.base.draw_str(&self.line,self.font,draw_parameters,g);
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,g:&mut Graphics){
        self.set_alpha_channel(alpha);
        self.draw(draw_parameters,g)
    }

    pub fn add_object(&self,graphics:&mut Graphics2D)->usize{
        graphics.add_text_object(self.line.clone(),&self.base,0).unwrap()
    }
}

// Неизменяемый зависимый текстовый блок с множеством линий текста
// Зависим от шрифта
pub struct TextViewStaticLined<'a>{
    base:TextBase,
    font:&'a Font<'static>,
    lines:Vec<(f32,String)>,
}

impl<'a> TextViewStaticLined<'a>{
    pub fn new<S:Into<String>>(settings:TextViewSettings<S>,font:&'a Font<'static>)->TextViewStaticLined<'a>{
        let mut lines=Vec::new();

        let font_size=settings.font_size;
        let dline=line_margin+font_size; // Расстояние между строками

        let mut height=dline; // Высота всего текста
        
        let line_length=settings.rect[2]; // Максимальная длина строки текста

        let mut last_whitespace=0; // Последний пробел - по нему разделяется текст при переходе на новую строку
        let mut line_start=0; // Индекс символа, с которого начинается строка
        let mut line_len=0f32; // Длина строки текста
        let mut word_len=0f32; // Длина слова - нужна для определения начальной длины строки текста при переходе на новую строку

        let whitespace_width=char_width(' ',settings.font_size,font);
        let nl_whitespace_width=char_width('\n',settings.font_size,font);

        let text=settings.text.into();

        for (c,ch) in text.char_indices(){
            let char_width=char_width(ch,settings.font_size,font);
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
            base:TextBase::new([x,y],settings.font_size,settings.text_colour),
            lines,
            font:font,
        }
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        let position=self.base.position; // Сохранение начальной позиции

        let dy=self.base.font_size+line_margin;
        // Перебор строк
        for line in &self.lines{
            let dx=line.0; // Выравнивание строки
            self.base.shift_x(dx);

            self.base.draw_str(&line.1,self.font,draw_parameters,graphics);

            self.base.shift(-dx,dy);
        }

        self.base.move_to(position);
    }

    // Вывод части текста
    pub fn draw_part(
        &mut self,
        chars:usize,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    )->bool{
        let mut point=Point{
            x:self.base.position[0],
            y:self.base.position[1]
        };

        let scale=Scale::uniform(self.base.font_size);

        let dy=self.base.font_size+line_margin as f32;

        let mut chars_passed=0; // Символов выведенно

        let mut whole_text=true;

        // Перебор строк
        'lines:for line in &self.lines{
            point.x+=line.0 as f32; // Сдвиг строки (выравнивание)

            for character in line.1.chars(){
                if chars_passed==chars{
                    whole_text=false;
                    break 'lines
                }
                chars_passed+=1;

                let scaled_glyph=self.font.glyph(character).scaled(scale);

                // ширина символа
                let width_offset=scaled_glyph.h_metrics().advance_width;

                // символ с позицией
                let glyph=scaled_glyph.positioned(point);

                graphics.draw_glyph(glyph,self.base.colour,draw_parameters).unwrap();

                // Сдвиг дальше вдоль горизонтальной линии
                point.x+=width_offset;
            }

            // Переход на новую строку
            point.x=self.base.position[0] as f32;
            point.y+=dy;
        }

        whole_text
    }
}

