use crate::colours::White;

use cat_engine::{
    // statics
    window_width,
    window_height,
    // structs
    graphics::{Graphics,Graphics2D},
    DefaultWindow,
    Window,
    image::{
        ImageBase,
        Texture,image::{
            self,
            RgbaImage,
            DynamicImage,
            imageops::FilterType,
            GenericImageView
        }
    },
    glium::DrawParameters,
};

use std::path::Path;

pub const wallpaper_movement_scale:f32=16f32;

// Подвижные обои
pub struct Wallpaper{
    index:usize,
    shift:[f32;2],
}

impl Wallpaper{
    pub fn new(image:&RgbaImage,window:&mut DefaultWindow)->Wallpaper{
        unsafe{
            let dx=window_width/(wallpaper_movement_scale*2f32);
            let dy=window_height/(wallpaper_movement_scale*2f32);
            let rect=[
                -dx,
                -dy,
                window_width+2f32*dx,
                window_height+2f32*dy,
            ];

            let image_base=ImageBase::new(White,rect);

            let texture=Texture::from_image(image,window.display()).unwrap();

            let index=window.graphics2d().add_textured_object(&image_base,texture).unwrap();

            Self{
                index,
                shift:[0f32;2]
            }
        }
    }

    #[inline(always)]
    pub fn mouse_shift(&mut self,[dx,dy]:[f32;2]){
        self.shift=[
            dx/wallpaper_movement_scale,
            dy/wallpaper_movement_scale,
        ]
    }

    // Обновляет картинка (она должна быть такого же размера, как и предыдущая)
    #[inline(always)]
    pub fn update_image(&mut self,image:&RgbaImage,graphics:&mut Graphics2D){
        graphics.get_textured_object_texture(self.index).update(image);
    }

    #[inline(always)]
    pub fn update_image_path<P:AsRef<Path>>(&mut self,path:P,size:[f32;2],graphics:&mut Graphics2D){
        self.update_image(&load_wallpaper_image(path,size[0],size[1]),graphics);
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut Graphics){
        graphics.draw_textured_object(self.index,draw_parameters);
    }

    pub fn draw_shift(
        &mut self,
        draw_parameters:&mut DrawParameters,
        graphics:&mut Graphics
    ){
        graphics.draw_shift_textured_object(
            self.index,
            self.shift,
            draw_parameters
        );
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&DrawParameters,graphics:&mut Graphics){
        graphics.graphics2d.get_textured_object_colour(self.index)[3]=alpha;
        self.draw(draw_parameters,graphics);
    }

    pub fn draw_shift_smooth(
        &mut self,
        alpha:f32,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    ){
        graphics.graphics2d.get_textured_object_colour(self.index)[3]=alpha;
        graphics.draw_shift_textured_object(
            self.index,
            self.shift,
            draw_parameters
        );
    }
}


// Загрузка фона
// Фон приводится к размеру экрана

// Если соотношение ширины к высоте картинки меньше, чем у экрана,
// то это значит, что при приведении ширины картинки к ширине экрана, сохраняя соотношение сторон,
// высота картинки будет больше высоты экрана, поэтому высоту нужно обрезать.

// Если наоборот, то приведении высоты картинки к высоте экрана, ширину картинки будеи больше, чем ширина экрана.
pub fn load_wallpaper_image<P:AsRef<Path>>(path:P,width0:f32,height0:f32)->RgbaImage{
    let mut image=image::open(path).unwrap();
    let k0=width0/height0;

    let image_width=image.width() as f32;
    let image_height=image.height() as f32;

    let k=image_width/image_height; // Отношение сторон монитора (ширина к высоте)

    // Расчёт размеров обрезки изображения
    let (x,y,width,height)=if k0>k{

        let height=image_width/k0;
        
        let y=image_height-height;

        (0u32,y as u32,image_width as u32,height as u32)
    }
    else{
        let width=image_height*k0;

        let x=(image_width-width)/2f32;

        (x as u32,0u32,width as u32,image_height as u32)
    };

    // Обрезка и приведение к размеру экрана
    image=image.crop_imm(x,y,width,height).resize_exact(width0 as u32,height0 as u32,FilterType::Gaussian);

    if let DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}