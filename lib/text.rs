use crate::*;

const line_margin:f64=20f64;

pub struct TextBase{
    pub image:Image,
    pub font_size:u32,
}

impl TextBase{
    pub fn new(font_size:u32)->TextBase{
        Self{
            image:Image::new_color(Black).rect([0f64;4]).src_rect([0f64;4]),
            font_size:font_size,
        }
    }

    pub fn new_color(color:Color,font_size:u32)->TextBase{
        Self{
            image:Image::new_color(color).rect([0f64;4]).src_rect([0f64;4]),
            font_size:font_size,
        }
    }

    pub fn position(mut self,position:[f64;2])->TextBase{
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=position[0];
        rect[1]=position[1];
        self
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha
    }

    pub fn set_position(&mut self,position:[f64;2]){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=position[0];
        rect[1]=position[1];
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]+=dx;
        rect[1]+=dy;
    }

    pub fn draw(&mut self,text:&str,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        // Сохранение начального положения
        let (x,y)={
            let image_rect=self.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };
        // Перебор символов
        for ch in text.chars(){
            let character=glyphs.character(self.font_size,ch).unwrap();

            { // Установка положения и размер символа
                let image_rect=self.image.rectangle.as_mut().unwrap();
                image_rect[0]+=character.left();
                image_rect[1]-=character.top();
                image_rect[2]=character.atlas_size[0];
                image_rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                let image_src_rect=self.image.source_rectangle.as_mut().unwrap();
                image_src_rect[0]=character.atlas_offset[0];
                image_src_rect[1]=character.atlas_offset[1];
                image_src_rect[2]=character.atlas_size[0];
                image_src_rect[3]=character.atlas_size[1];
            }

            self.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше по линии и возвращение обратно на линию
            let image_rect=self.image.rectangle.as_mut().unwrap();
            image_rect[0]+=character.advance_width()-character.left();
            image_rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        let image_rect=self.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }
}

pub struct TextLine{
    line:String,
}

impl TextLine{
    pub fn new<S:ToString>(text:S)->TextLine{
        Self{
            line:text.to_string()
        }
    }

    pub fn get_text(&self)->String{
        self.line.clone()
    }

    pub fn set_text<S:ToString>(&mut self,text:S){
        self.line=text.to_string()
    }
}

impl Text for TextLine{
    fn draw(&self,text_base:&mut TextBase,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let (x,y)={
            let image_rect=text_base.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };
        // Перебор символов
        for ch in self.line.chars(){
            let character=glyphs.character(text_base.font_size,ch).unwrap();

            { // Установка положения и размер символа
                let image_rect=text_base.image.rectangle.as_mut().unwrap();
                image_rect[0]+=character.left();
                image_rect[1]-=character.top();
                image_rect[2]=character.atlas_size[0];
                image_rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                let image_src_rect=text_base.image.source_rectangle.as_mut().unwrap();
                image_src_rect[0]=character.atlas_offset[0];
                image_src_rect[1]=character.atlas_offset[1];
                image_src_rect[2]=character.atlas_size[0];
                image_src_rect[3]=character.atlas_size[1];
            }

            text_base.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше по линии и возвращение обратно на линию
            let image_rect=text_base.image.rectangle.as_mut().unwrap();
            image_rect[0]+=character.advance_width()-character.left();
            image_rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        let image_rect=text_base.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }

    fn draw_part(&self,chars:usize,text_base:&mut TextBase,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache)->bool{
        let (x,y)={
            let image_rect=text_base.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };

        let mut chars_passed=0; // Символов выведенно
        let mut whole_text=true;

        // Перебор символов
        for ch in self.line.chars(){
            if chars_passed==chars{
                whole_text=false;
                break
            }
            chars_passed+=1;
            let character=glyphs.character(text_base.font_size,ch).unwrap();

            { // Установка положения и размер символа
                let image_rect=text_base.image.rectangle.as_mut().unwrap();
                image_rect[0]+=character.left();
                image_rect[1]-=character.top();
                image_rect[2]=character.atlas_size[0];
                image_rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                let image_src_rect=text_base.image.source_rectangle.as_mut().unwrap();
                image_src_rect[0]=character.atlas_offset[0];
                image_src_rect[1]=character.atlas_offset[1];
                image_src_rect[2]=character.atlas_size[0];
                image_src_rect[3]=character.atlas_size[1];
            }

            text_base.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше по линии и возвращение обратно на линию
            let image_rect=text_base.image.rectangle.as_mut().unwrap();
            image_rect[0]+=character.advance_width()-character.left();
            image_rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        let image_rect=text_base.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;

        whole_text
    }
}


// Текст, разделённый на линии,
// максимальная длина которых равна `line_length`
pub struct TextLines{
    lines:Vec<String>,
}

impl TextLines{
    pub fn new<S:ToString>(text:S,line_length:f64,font_size:u32,glyphs:&mut GlyphCache)->TextLines{
        let mut vec=Vec::<String>::new();
        let mut last_whitespace=0;
        let mut line_start=0;
        let mut line_len=0f64;

        let text=text.to_string();

        for (c,ch) in text.char_indices(){
            if ch.is_whitespace(){
                last_whitespace=c;
            }
            let character=glyphs.character(font_size,ch).unwrap();

            line_len+=character.advance_width();
            if line_len>=line_length{
                line_len=0f64;
                if line_start==last_whitespace{
                    break
                }

                let line=text[line_start..last_whitespace].to_string();
                vec.push(line);
                last_whitespace+=1;
                line_start=last_whitespace;
            }
        }
        let line=text[line_start..].to_string();
        vec.push(line);

        Self{
            lines:vec
        }
    }
}

impl Text for TextLines{

    fn draw(&self,text_base:&mut TextBase,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        // Сохранение начального положения
        let (x,y)={
            let image_rect=text_base.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };

        let dy=text_base.font_size as f64+line_margin;

        // Перебор строк
        for line in &self.lines{
            for ch in line.chars(){
                let character=glyphs.character(text_base.font_size,ch).unwrap();

                { // Установка положения и размер символа
                    let image_rect=text_base.image.rectangle.as_mut().unwrap();
                    image_rect[0]+=character.left();
                    image_rect[1]-=character.top();
                    image_rect[2]=character.atlas_size[0];
                    image_rect[3]=character.atlas_size[1];
                }

                { // Обрезка символа
                    let image_src_rect=text_base.image.source_rectangle.as_mut().unwrap();
                    image_src_rect[0]=character.atlas_offset[0];
                    image_src_rect[1]=character.atlas_offset[1];
                    image_src_rect[2]=character.atlas_size[0];
                    image_src_rect[3]=character.atlas_size[1];
                }

                text_base.image.draw(character.texture,&c.draw_state,c.transform,g);

                // Сдвиг дальше по линии и возвращение обратно на линию
                let image_rect=text_base.image.rectangle.as_mut().unwrap();
                image_rect[0]+=character.advance_width()-character.left();
                image_rect[1]+=character.advance_height()+character.top();
            }

            let image_rect=text_base.image.rectangle.as_mut().unwrap();
            image_rect[0]=x;
            image_rect[1]+=dy;
        }
        // Возвращение в начальное положение
        let image_rect=text_base.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }

    fn draw_part(&self,chars:usize,text_base:&mut TextBase,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache)->bool{
        // Сохранение начального положения
        let (x,y)={
            let image_rect=text_base.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };

        let dy=text_base.font_size as f64+line_margin;

        let mut chars_passed=0; // Символов выведенно

        let mut whole_text=true;

        // Перебор строк
        'lines:for line in &self.lines{
            for ch in line.chars(){
                if chars_passed==chars{
                    whole_text=false;
                    break 'lines
                }
                chars_passed+=1;

                let character=glyphs.character(text_base.font_size,ch).unwrap();

                { // Установка положения и размер символа
                    let image_rect=text_base.image.rectangle.as_mut().unwrap();
                    image_rect[0]+=character.left();
                    image_rect[1]-=character.top();
                    image_rect[2]=character.atlas_size[0];
                    image_rect[3]=character.atlas_size[1];
                }

                { // Обрезка символа
                    let image_src_rect=text_base.image.source_rectangle.as_mut().unwrap();
                    image_src_rect[0]=character.atlas_offset[0];
                    image_src_rect[1]=character.atlas_offset[1];
                    image_src_rect[2]=character.atlas_size[0];
                    image_src_rect[3]=character.atlas_size[1];
                }

                // Вывод символа
                text_base.image.draw(character.texture,&c.draw_state,c.transform,g);

                // Сдвиг дальше по линии и возвращение обратно на линию
                let image_rect=text_base.image.rectangle.as_mut().unwrap();
                image_rect[0]+=character.advance_width()-character.left();
                image_rect[1]+=character.advance_height()+character.top();
            }

            let image_rect=text_base.image.rectangle.as_mut().unwrap();
            image_rect[0]=x;
            image_rect[1]+=dy;
        }
        // Возвращение в начальное положение
        let image_rect=text_base.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;

        whole_text
    }
}