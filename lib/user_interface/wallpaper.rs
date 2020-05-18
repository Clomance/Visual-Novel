use crate::{
    colors::White,
    traits::Drawable,
};

use engine::{
    // statics
    window_width,
    window_height,
    // structs
    graphics::GameGraphics,
    image::{ImageBase,Texture,image::RgbaImage},
    glium::{Display,DrawParameters},
};

pub const wallpaper_movement_scale:f32=16f32;

pub struct Wallpaper{
    image:ImageBase,
    texture:Texture,
}

impl Wallpaper{
    pub fn new(image:&RgbaImage,display:&mut Display)->Wallpaper{
        unsafe{
            let dx=window_width/(wallpaper_movement_scale*2f32);
            let dy=window_height/(wallpaper_movement_scale*2f32);
            let rect=[
                -dx,
                -dy,
                window_width+2f32*dx,
                window_height+2f32*dy,
            ];

            Self{
                image:ImageBase::new(White,rect),
                texture:Texture::from_image(display,image).unwrap(),
            }
        }
    }

    pub fn mouse_shift(&mut self,dx:f32,dy:f32){
        self.image.x1+=dx/wallpaper_movement_scale;
        self.image.y1+=dy/wallpaper_movement_scale;
        self.image.x2+=dx/wallpaper_movement_scale;
        self.image.y2+=dy/wallpaper_movement_scale;
    }

    // Обновляет картинка (она должна быть такого же размера, как и предыдущая)
    pub fn update_image(&mut self,image:&RgbaImage){
        self.texture.update(image);
    }
}

impl Drawable for Wallpaper{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.colour[3]=alpha
    }

    fn draw(&mut self,draw_parameters:&mut DrawParameters,g:&mut GameGraphics){
        self.image.draw(&self.texture,draw_parameters,g)
    }
}