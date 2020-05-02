#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,dead_code)]

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

use opengl_graphics::{
    GlGraphics,
    GlyphCache
};

use graphics::{
    Rectangle,
    character::CharacterCache,
    types::Color,
    Image,
    Context
};

use image::{
    self,
    DynamicImage,
    RgbaImage,
    imageops::FilterType,
};

use std::{
    path::Path,
    fs::{metadata,read_dir},
};



// Загрузка изображений
pub fn load_image<P:AsRef<Path>>(path:P,width:u32,height:u32)->RgbaImage{
    let mut image=image::open(path).unwrap();
    image=image.resize_exact(width,height,FilterType::Gaussian);
    if let DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}

pub fn load_textures<P:AsRef<Path>+Clone>(path:P,width:u32,height:u32)->Vec<RgbaImage>{
    let meta=metadata(path.clone()).unwrap();
    let mut textures=Vec::with_capacity(meta.len() as usize);
    let dir=read_dir(path).unwrap();

    for r in dir{
        let file=r.unwrap();
        let _name=file.file_name();
        let path=file.path();
        let image=load_image(path,width,height);
        textures.push(image)
    }

    textures
}

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