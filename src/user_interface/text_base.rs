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
            image:Image::new_color(Black).rect([0f64;4]),
            color:Black,
            font_size:font_size,
            line_len:line_len,
        }
    }

    pub fn new_color(font_size:u32,line_len:f64,color:Color)->TextBase{
        Self{
            image:Image::new_color(color).rect([0f64;4]),
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

    pub fn draw_slowly(&self,text:&str,mut position:[f64;2],chars:usize,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let next_line=self.font_size as f64+10f64;

        let x=position[0];

        for (passed,ch) in text.chars().enumerate(){
            if passed==chars{
                break
            }
            let character=glyphs.character(self.font_size,ch).unwrap();
            let ch_x:f64=position[0]+character.left();
            let ch_y:f64=position[1]-character.top();

            let rect=[ch_x,ch_y,character.atlas_size[0],character.atlas_size[1]];

            let image=self.image.rect(rect).src_rect([
                character.atlas_offset[0],
                character.atlas_offset[1],
                character.atlas_size[0],
                character.atlas_size[1]
            ]);

            image.draw(character.texture,&c.draw_state,c.transform,g);
            position[0]+=character.advance_width();
            position[1]+=character.advance_height();

            if position[0]>self.line_len{
                position[0]=x;
                position[1]+=next_line;
            }
        }
    }

    pub fn draw(&mut self,text:&str,mut position:[f64;2],c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let next_line=self.font_size as f64+10f64;

        let (x,y)={
            let image_rect=self.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };

        for ch in text.chars(){
            {
                let image_rect=self.image.rectangle.as_mut().unwrap();

            }
            let character=glyphs.character(self.font_size,ch).unwrap();
            let ch_x:f64=position[0]+character.left();
            let ch_y:f64=position[1]-character.top();

            let rect=[ch_x,ch_y,character.atlas_size[0],character.atlas_size[1]];

            let image=self.image.rect(rect).src_rect([
                character.atlas_offset[0],
                character.atlas_offset[1],
                character.atlas_size[0],
                character.atlas_size[1]
            ]);

            image.draw(character.texture,&c.draw_state,c.transform,g);
            position[0]+=character.advance_width();
            position[1]+=character.advance_height();

            if position[0]>self.line_len{
                position[0]=x;
                position[1]+=next_line;
            }
        }
        let image_rect=self.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }
}