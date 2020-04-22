use crate::*;

// Пути главных текстур
const main_textures_paths:&[&'static str]=&[
    "images/wallpapers/main_menu_wallpaper.png", // Главное меню
    "images/dialogue_box.png", // Диалоговое окно
    "images/wallpapers/ending_wallpaper.png", // Конечная заставка
];

pub struct Textures{
    game_wallpapers:Vec<RgbaImage>,
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
    pub fn load()->Textures{
        unsafe{
            let dx=window_width/(wallpaper_movement_scale*2f64);
            let dy=window_height/(wallpaper_movement_scale*2f64);
            let wallpaper_size=[
                (window_width+2f64*dx) as u32,
                (window_height+2f64*dy) as u32
            ];

            let mut vec=Vec::with_capacity(3);
            // Загрузка главных текстур
            for path in main_textures_paths{
                let wallpaper_texture=load_image(path,wallpaper_size[0],wallpaper_size[1]);
                vec.push(wallpaper_texture);
            }
            Self{
                game_wallpapers:load_textures("images/wallpapers/game",wallpaper_size[0],wallpaper_size[1]),
                main:vec,
                characters:load_textures("images/characters",(2f64*window_height/5f64) as u32,(4f64*window_height/5f64) as u32),
            }
        }
    }

    pub fn main_menu_wallpaper(&self)->&RgbaImage{
        &self.main[0]
    }

    pub fn dialogue_box(&self)->&RgbaImage{
        &self.main[1]
    }

    pub fn ending_wallpaper(&self)->&RgbaImage{
        &self.main[2]
    }

    pub fn wallpaper(&self,index:usize)->&RgbaImage{
        &self.game_wallpapers[index]
    }

    pub fn character(&self,index:usize)->&RgbaImage{
        &self.characters[index]
    }
}