//! Основы работы с текстом. Text basics. `feature = "text_graphics"`, `default-features`.
//!
//! See [rusttype](https://crates.io/crates/rusttype) for more details.
//! 
//! 

pub use rusttype;

mod text_base;
pub use text_base::TextBase;

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

const pixel_scale:f32=1.47; // Коэффициент для приведения к нужному размеру шрифта

/// Шрифт.
/// 
/// Font.
pub struct Glyphs{
    font:Font<'static>
}

impl Glyphs{
    /// Загрузка шрифта из файла.
    /// 
    /// Loading font from file.
    pub fn load<P:AsRef<Path>>(path:P)->Glyphs{
        let data=fs::read(&path).unwrap();
        let font=Font::try_from_vec(data).unwrap();
        Self{
            font
        }
    }

    /// Возвращает символ, определённого размера, с нулейвой позицией.
    /// 
    /// Returns a character with given size and zero position.
    pub fn character(&self,character:char,font_size:f32)->Character{
        let scale=Scale::uniform(font_size*pixel_scale); // Приведение к общему размеру пикселей
        let c=self.font.glyph(character).scaled(scale);

        let point=Point{
            x:0f32,
            y:0f32
        };

        Character{
            c:c.positioned(point)
        }
    }

    /// Символ, определённого размера с определённой позицией.
    /// 
    /// Returns a character with given size and position.
    pub fn character_positioned(&self,character:char,font_size:f32,position:[f32;2])->Character{
        let scale=Scale::uniform(font_size*pixel_scale); // Приведение к общему размеру пикселей
        let c=self.font.glyph(character).scaled(scale);

        let point=Point{
            x:position[0],
            y:position[1]
        };

        Character{
            c:c.positioned(point)
        }
    }
}

/// Обёртка для символа.
/// 
/// Wrapper for a character.
pub struct Character{
    c:PositionedGlyph<'static>,
}

impl Character{
    pub fn position(&self)->[f32;2]{
        let p=self.c.position();
        [p.x,p.y]
    }

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