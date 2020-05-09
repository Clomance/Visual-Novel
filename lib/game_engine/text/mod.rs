mod text_base;
pub use text_base::{TextBase,TextGraphics};


use std::{
    fs,
    path::Path,
};

use rusttype::{
    Font,
    Scale,
    Point,
    PositionedGlyph,
    Rect,
};

// Шрифт
pub struct Glyphs<'a>{
    font:Font<'a>
}

impl<'a> Glyphs<'a>{
    pub fn load<P:AsRef<Path>>(path:P)->Glyphs<'a>{
        let data=fs::read(&path).unwrap();
        let font=Font::try_from_vec(data).unwrap();
        Self{
            font
        }
    }

    pub fn glyph_height_unscaled(&self)->f32{
        let v=self.font.v_metrics_unscaled();
        v.ascent-v.descent
    }

    pub fn glyph_height(&self,font_size:f32)->f32{
        let scale=Scale::uniform(font_size);
        let v=self.font.v_metrics(scale);
        v.ascent-v.descent
    }

    pub fn character(&self,character:char,font_size:f32)->Character<'a>{
        let scale=Scale::uniform(font_size*1.47); // Приведение к общему размеру пикселей
        let c=self.font.glyph(character).scaled(scale);

        let point=Point{
            x:0f32,
            y:0f32
        };

        Character{
            c:c.positioned(point)
        }
    }
}

pub struct Character<'a>{
    c:PositionedGlyph<'a>,
}

impl<'a> Character<'a>{
    #[inline(always)]
    pub fn height(&self)->f32{
        if let Some(rect)=self.c.pixel_bounding_box(){
            rect.height() as f32
        }
        else{
            0f32
        }
    }

    #[inline(always)]
    pub fn width(&self)->f32{
        self.c.unpositioned().h_metrics().advance_width
    }

    pub fn width_with_offset(&self)->f32{
        let h=self.c.unpositioned().h_metrics();
        h.advance_width+h.left_side_bearing
    }

    #[inline(always)]
    pub fn pixel_bounding_box(&self)->Option<Rect<i32>>{
        self.c.pixel_bounding_box()
    }

    #[inline(always)]
    pub fn draw<F:FnMut(u32,u32,f32)>(&self,f:F){
        self.c.draw(f)
    }
}