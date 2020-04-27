use crate::*;

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

    #[inline(always)]
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
            let wallpaper_texture=load_image("./resources/images/wallpapers/main_menu_wallpaper.png",wallpaper_size[0],wallpaper_size[1]);
            vec.push(wallpaper_texture);

            let wallpaper_texture=load_image("./resources/images/dialogue_box.png",window_width as u32,(window_height/3f64) as u32);
            vec.push(wallpaper_texture);
            
            let wallpaper_texture=load_image("./resources/images/wallpapers/ending_wallpaper.png",wallpaper_size[0],wallpaper_size[1]);
            vec.push(wallpaper_texture);


            Self{
                game_wallpapers:load_textures("./resources/images/wallpapers/game",wallpaper_size[0],wallpaper_size[1]),
                main:vec,
                characters:load_textures("./resources/images/characters",(2f64*window_height/5f64) as u32,(4f64*window_height/5f64) as u32),
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
    pub fn wallpaper(&self,index:usize)->&RgbaImage{
        &self.game_wallpapers[index]
    }

    #[inline(always)]
    pub fn character(&self,index:usize)->&RgbaImage{
        &self.characters[index]
    }
}