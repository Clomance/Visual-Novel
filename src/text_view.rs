use crate::*;


// Текстовый блок
pub struct TextView<'a>{
    x1:f64,
    y1:f64,
    text_base:Text,
    text:String,
    glyphs:GlyphCache<'a>,
}

impl<'a> TextView<'a>{
    pub fn new(rect:[f64;4],text:String,font_size:u32,mut glyphs:GlyphCache<'a>)->TextView<'a>{
        let mut text_len=0f64;
        for ch in text.chars(){
            let character=glyphs.character(font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        let x1=rect[0]+(rect[2]-text_len)/2f64;

        let y1=rect[1]+(rect[2]+font_size as f64/2f64);

        Self{
            x1:x1,
            y1:y1,
            text_base:Text::new(font_size),
            text:text,
            glyphs:glyphs
        }
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        let x=self.x1;
        let y=self.y1;
        self.text_base.draw(&self.text,&mut self.glyphs,draw_state,transform.trans(x,y),g);
    }
}

// Зависимый от шрифта текстовый блок
pub struct TextViewDependent{
    x1:f64,
    y1:f64,
    text_base:Text,
    text:String,
}

impl TextViewDependent{
    pub fn new(rect:[f64;4],text:String,font_size:u32,glyphs:&mut GlyphCache)->TextViewDependent{
        let mut text_len=0f64;
        for ch in text.chars(){
            let character=glyphs.character(font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        let x1=rect[0]+(rect[2]-text_len)/2f64;

        let y1=rect[1]+(rect[3]+font_size as f64)/2f64;

        Self{
            x1:x1,
            y1:y1,
            text_base:Text::new_color(BLACK,font_size),
            text:text
        }
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let x=self.x1;
        let y=self.y1;
        self.text_base.draw(&self.text,glyphs,draw_state,transform.trans(x,y),g);
    }
}