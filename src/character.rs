use crate::*;

pub struct Character{
    image:Image,
    texture:Texture
}

impl Character{
    pub fn new<P:AsRef<Path>>(image_path:P,texture_settings:&TextureSettings)->Character{
        let image=Image::new_color([1.0;4]);
        let texture=Texture::from_path(image_path,texture_settings).unwrap();
        Self{
            image:image,
            texture:texture,
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha;
    }

    // pub fn set_rect(&mut self,rect:[f64;4]){
    //     self.image.rectangle=Some(rect);
    // }

    pub fn draw(&self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        g.image(&self.image,&self.texture,draw_state,transform);
    }
}