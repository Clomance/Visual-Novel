use crate::*;

pub struct Wallpaper<'a>{
    image:Image,
    texture:&'a Texture,
}

impl<'a> Wallpaper<'a>{
    pub fn new(texture:&'a Texture)->Wallpaper<'a>{
        unsafe{
            let image=Image::new_color([1.0;4]).rect([
                0.0,
                0.0,
                Settings.window_size.width,
                Settings.window_size.height
            ]);
            Self{
                image,
                texture:texture,
            }
        }
    }

    pub fn set_texture(&mut self,texture:&'a Texture){
        self.texture=texture;
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha;
    }

    // pub fn fit_screen(&mut self){
    //     unsafe{
    //         let rect=self.image.rectangle.as_mut().unwrap();
    //         rect[2]=Settings.window_size.width;
    //         rect[3]=Settings.window_size.height;
    //     }
    // }

    pub fn draw(&self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        g.image(&self.image,self.texture,draw_state,transform);
    }
}