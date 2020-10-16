use crate::colours::White;

use cat_engine::{
    Colour,
    image::RgbaImage,
    graphics::Graphics2D
};

pub enum Wallpaper{
    Texture,
    Colour(Colour)
}

impl Wallpaper{
    #[inline(always)]
    pub fn set_image(&self,image:&RgbaImage,graphics:&mut Graphics2D){
        graphics.get_textured_object_texture(1).update(image);
    }

    // #[inline(always)]
    // pub fn update_image_path<P:AsRef<Path>>(&mut self,path:P,size:[f32;2],graphics:&mut Graphics2D){
    //     self.update_image(&load_wallpaper_image(path,size[0],size[1]),graphics);
    // }
}