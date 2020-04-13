use crate::*;

pub struct Wallpaper{
    start_position:[f64;2],
    image:Image,
    texture:Texture,
    settings:TextureSettings,
}

impl Wallpaper{
    pub fn new(image:&RgbaImage,rect:[f64;4])->Wallpaper{
        let settings=TextureSettings::new();
        Self{
            start_position:[rect[0],rect[1]],
            image:Image::new_color([1.0;4]).rect(rect),
            texture:Texture::from_image(image,&settings),
            settings:settings,
        }
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]+=dx;
        rect[1]+=dy;
    }

    pub fn set_image(&mut self,image:&RgbaImage){
        self.texture.update(image);
    }

    pub fn move_with_cursor(&mut self,mouse_position:[f64;2]){
        let r_x=unsafe{window_center[0]-mouse_position[0]};

        let r_y=unsafe{window_center[1]-mouse_position[1]};

        let position=[
            r_x/wallpaper_movement_scale,
            r_y/wallpaper_movement_scale
        ];

        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=position[0]+self.start_position[0];
        rect[1]=position[1]+self.start_position[1];
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