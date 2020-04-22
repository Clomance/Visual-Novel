use crate::*;

pub const wallpaper_movement_scale:f64=16f64;

pub struct Wallpaper{
    image:Image,
    texture:Texture,
}

impl Wallpaper{
    pub fn new()->Wallpaper{
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
                image:Image::new_color([1.0;4]).rect(rect),
                texture:Texture::empty(&settings).unwrap(),
            }
        }
    }

    pub fn mouse_shift(&mut self,dx:f64,dy:f64){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]+=dx/wallpaper_movement_scale;
        rect[1]+=dy/wallpaper_movement_scale;
    }

    pub fn set_image(&mut self,image:&RgbaImage){
        let settings=TextureSettings::new();
        self.texture=Texture::from_image(image,&settings);
    }
}
impl Drawable for Wallpaper{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha
    }

    fn draw(&mut self,c:&Context,g:&mut GlGraphics){
        self.image.draw(&self.texture,&c.draw_state,c.transform,g)
    }
}