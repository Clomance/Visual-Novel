use lib::{
    wallpaper_movement_scale,
    load_wallpaper_image,
};

use engine::{
    // statics
    window_width,
    window_height,
    // 
    image::image::{
        self,
        GenericImageView,
        DynamicImage,
        RgbaImage,
        imageops::FilterType,
    }
};

use std::{
    fs::{metadata,read_dir},
    path::{Path,PathBuf},
};

pub struct Textures{
    game_wallpapers:Vec<PathBuf>,
    main:Vec<RgbaImage>,
    characters:Vec<RgbaImage>,
}

impl Textures{
    pub const fn new()->Textures{
        Self{
            game_wallpapers:Vec::new(),
            main:Vec::new(),
            characters:Vec::new(),
        }
    }

    #[inline(always)]
    pub fn load()->Textures{
        unsafe{
            let dx=window_width/(wallpaper_movement_scale*2f32);
            let dy=window_height/(wallpaper_movement_scale*2f32);
            let wallpaper_size=[
                (window_width+2f32*dx),
                (window_height+2f32*dy)
            ];

            let mut vec=Vec::with_capacity(3);

            // Загрузка главных текстур
            let mut wallpaper_texture=load_wallpaper_image("./resources/images/wallpapers/main_menu_wallpaper.png",wallpaper_size[0],wallpaper_size[1]);
            vec.push(wallpaper_texture);
            wallpaper_texture=load_image("./resources/images/dialogue_box.png",window_width as u32,(window_height/3f32) as u32);
            vec.push(wallpaper_texture);
            wallpaper_texture=load_wallpaper_image("./resources/images/wallpapers/ending_wallpaper.png",wallpaper_size[0],wallpaper_size[1]);
            vec.push(wallpaper_texture);

            Self{
                game_wallpapers:load_wallpapers_textures_paths("./resources/images/wallpapers/game"),
                main:vec,
                characters:load_characters_textures(window_height*0.75),
            }
        }
    }

    #[inline(always)]
    pub fn main_menu_wallpaper(&self)->&RgbaImage{
        &self.main[0]
    }

    #[inline(always)]
    pub fn dialogue_box(&self)->&RgbaImage{
        &self.main[1]
    }

    #[inline(always)]
    pub fn ending_wallpaper(&self)->&RgbaImage{
        &self.main[2]
    }

    #[inline(always)]
    pub fn wallpaper(&self,index:usize)->&PathBuf{
        &self.game_wallpapers[index]
    }

    #[inline(always)]
    pub fn character(&self,index:usize)->&RgbaImage{
        &self.characters[index]
    }
}

// Загрузка изображений
fn load_image<P:AsRef<Path>>(path:P,width:u32,height:u32)->RgbaImage{
    let mut image=image::open(path).unwrap();

    image=image.resize_exact(width,height,FilterType::Gaussian);
    if let DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}

// Загрузка фонов
fn load_wallpapers_textures_paths<P:AsRef<Path>+Clone>(path:P)->Vec<PathBuf>{
    let meta=metadata(path.clone()).unwrap();
    let mut textures=Vec::with_capacity(meta.len() as usize);
    let dir=read_dir(path).unwrap();

    for r in dir{
        let file=r.unwrap();
        let path=file.path();
        textures.push(path)
    }

    textures
}

fn load_characters_textures(height:f32)->Vec<RgbaImage>{
    let path="./resources/images/characters";
    let meta=metadata(path).unwrap();

    let mut char_textures=Vec::with_capacity(meta.len() as usize);

    let dir=read_dir(path).unwrap();

    for r in dir{
        let file=r.unwrap();
        let _name=file.file_name();
        let path=file.path();
        let image=load_character_image(path,height);
        char_textures.push(image)
    }

    char_textures
}

// Загрузка изображений
fn load_character_image<P:AsRef<Path>>(path:P,height:f32)->RgbaImage{
    let mut image=image::open(path).unwrap();
    let image_height=image.height() as f32;
    let image_width=image.width() as f32;

    let width=image_width*height/image_height;

    image=image.resize_exact(width as u32,height as u32,FilterType::Gaussian);
    if let DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}