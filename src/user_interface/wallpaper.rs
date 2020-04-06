use crate::*;

pub struct Wallpaper{
    image:Image,
    texture:Texture,
    settings:TextureSettings,
}

impl Wallpaper{
    pub fn new(image:&RgbaImage)->Wallpaper{
        let settings=TextureSettings::new();
        Self{
            image:Image::new_color([1.0;4]),
            texture:Texture::from_image(image,&settings),
            settings:settings,
        }
    }

    pub fn set_image(&mut self,image:&RgbaImage){
        self.texture.update(image);
    }
}

impl Drawable for Wallpaper{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha;
    }

    fn draw(&mut self,c:&Context,g:&mut GlGraphics){
        g.image(&self.image,&self.texture,&c.draw_state,c.transform);
    }
}