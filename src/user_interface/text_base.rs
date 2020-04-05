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
            image:Image::new_color(Black),
            color:Black,
            font_size:font_size,
            line_len:line_len,
        }
    }

    pub fn new_color(font_size:u32,line_len:f64,color:Color)->TextBase{
        Self{
            image:Image::new_color(color),
            color:color,
            font_size:font_size,
            line_len:line_len,
        }
    }

    pub fn draw(&self,text:&str,mut position:[f64;2],draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let next_line=self.font_size as f64+10f64;

        let x=position[0];

        for ch in text.chars(){
            let character=glyphs.character(self.font_size,ch).unwrap();
            let ch_x:f64=position[0]+character.left();
            let ch_y:f64=position[1]-character.top();

            let image=self.image.src_rect([
                character.atlas_offset[0],
                character.atlas_offset[1],
                character.atlas_size[0],
                character.atlas_size[1]
            ]);

            image.draw(character.texture,draw_state,transform.trans(ch_x,ch_y),g);
            position[0]+=character.advance_width();
            position[1]+=character.advance_height();

            if position[0]>self.line_len{
                position[0]=x;
                position[1]+=next_line;
            }
        }
    }
}