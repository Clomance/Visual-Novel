use crate::{
    colors::*,
    image_base::ImageBase,
    traits::Drawable,
    Texture
};

use super::{
    // statics
    window_width,
    window_height,
    //
    GameGraphics,
};

use texture::TextureSettings;

use image::RgbaImage;

use glium::Display;

use graphics::Context;

pub const wallpaper_movement_scale:f64=16f64;

pub struct Wallpaper{
    image:ImageBase,
    texture:Texture,
}

impl Wallpaper{
    pub fn new(image:&RgbaImage,display:&mut Display)->Wallpaper{
        unsafe{
            let dx=window_width/(wallpaper_movement_scale*2f64);
            let dy=window_height/(wallpaper_movement_scale*2f64);
            let rect=[
                -dx,
                -dy,
                window_width+2f64*dx,
                window_height+2f64*dy,
            ];

            let settings=TextureSettings::new();
            Self{
                image:ImageBase::new(White,rect),
                texture:Texture::from_image(display,image,&settings).unwrap(),
            }
        }
    }

    pub fn mouse_shift(&mut self,dx:f64,dy:f64){
        self.image.rect[0]+=dx/wallpaper_movement_scale;
        self.image.rect[1]+=dy/wallpaper_movement_scale;
    }

    // Обновляет картинка (она должна быть такого же размера, как и предыдущая)
    pub fn update_image(&mut self,image:&RgbaImage){
        self.texture.update(image);
    }
}

impl Drawable for Wallpaper{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color[3]=alpha
    }

    fn draw(&mut self,c:&Context,g:&mut GameGraphics){
        self.image.draw(&self.texture,&c.draw_state,c.transform,g)
    }
}