use crate::*;

pub struct TextBase{
    pub image:Image,
    pub color:Color,
    pub font_size:u32,
    pub line_len:f64, // Длина строки для ввода текста
}

impl TextBase{
    pub fn new(font_size:u32,line_len:f64)->TextBase{
        Self{
            image:Image::new_color(Black).rect([0f64;4]).src_rect([0f64;4]),
            color:Black,
            font_size:font_size,
            line_len:line_len,
        }
    }

    pub fn new_color(font_size:u32,line_len:f64,color:Color)->TextBase{
        Self{
            image:Image::new_color(color).rect([0f64;4]).src_rect([0f64;4]),
            color:color,
            font_size:font_size,
            line_len:line_len,
        }
    }

    pub fn position(mut self,position:[f64;2])->TextBase{
        let mut rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=position[0];
        rect[1]=position[1];
        self
    }

    pub fn draw_slowly(&mut self,text:&str,chars:usize,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let next_line=self.font_size as f64+10f64;

        let (x,y)={
            let image_rect=self.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };

        for (passed,ch) in text.chars().enumerate(){
            if chars==passed{
                break
            }
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

            if image_rect[0]>self.line_len{
                image_rect[0]=x;
                image_rect[1]+=next_line;
            }
        }
        // Возвращение в начальное положение
        let image_rect=self.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }

    pub fn draw(&mut self,text:&str,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let next_line=self.font_size as f64+10f64;

        let (x,y)={
            let image_rect=self.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };

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

            if image_rect[0]>self.line_len{
                image_rect[0]=x;
                image_rect[1]+=next_line;
            }
        }
        // Возвращение в начальное положение
        let image_rect=self.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }
}