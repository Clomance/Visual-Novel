#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use)]

use graphics::{
    character::CharacterCache,
    types::Color,
    rectangle::Rectangle,
    Context,
};


mod sync_raw_ptr;
pub use sync_raw_ptr::SyncRawPtr;

mod traits;
pub use traits::*;

mod colors;
pub use colors::*;

mod text_base;
pub use text_base::TextBase;

mod background;
pub use background::Background;

mod image_base;
pub use image_base::ImageBase;

mod game_window;
pub use game_window::{
    *,
    GameGraphics,
};

mod user_interface;
pub use user_interface::*;

// Выравнивание
#[derive(Clone)]
pub struct Align{
    pub x:AlignX,
    pub y:AlignY
}

impl Align{
    pub const fn center()->Align{
        Self{
            x:AlignX::Center,
            y:AlignY::Center,
        }
    }

    pub fn position(&self,location:[f64;4],size:[f64;2])->(f64,f64){
        // Выравнивание по x
        let x=match self.x{
            AlignX::Left=>location[0],
            AlignX::Center=>location[0]+(location[2]-size[0])/2f64,
            AlignX::Right=>location[0]+location[2]-size[0],
        };
        
        // Выравнивание по y
        let y=match self.y{
            AlignY::Up=>location[1],
            AlignY::Center=>location[1]+(location[3]-size[1])/2f64,
            AlignY::Down=>location[1]+location[3]-size[1],
        };

        (x,y)
    }

    // size - длина текста, максимальная высота текста
    pub fn text_position(&self,location:[f64;4],size:[f64;2])->(f64,f64){
        // Выравнивание по x
        let x=match self.x{
            AlignX::Left=>location[0],
            AlignX::Center=>location[0]+(location[2]-size[0])/2f64,
            AlignX::Right=>location[0]+location[2]-size[0],
        };
        
        // Выравнивание по y
        let y=match self.y{
            AlignY::Up=>location[1]+size[1],
            AlignY::Center=>location[1]+(location[3]+size[1])/2f64,
            AlignY::Down=>location[1]+location[3],
        };

        (x,y)
    }
}

// Тип выравнивания по x
#[derive(Clone)]
pub enum AlignX{
    Left,
    Center,
    Right
}

// Тип выравнивания по y
#[derive(Clone)]
pub enum AlignY{
    Up,
    Center,
    Down
}