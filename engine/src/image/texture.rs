use std::path::Path;

use glium::{
    Display,
    texture::{RawImage2d,TextureCreationError,srgb_texture2d::SrgbTexture2d},
};

use image::{RgbaImage,DynamicImage};

// Обёртка для 2D текстуры
pub struct Texture(pub SrgbTexture2d);

impl Texture{
    // Создание текстуры из массива байт
    pub fn create<S:Into<[u32;2]>>(factory:&mut Display,memory:&[u8],size:S)->Result<Self,TextureCreationError>{
        let size=size.into();

        let image=RawImage2d::from_raw_rgba_reversed(memory,(size[0],size[1]));

        let texture=SrgbTexture2d::new(factory,image)?;

        Ok(Texture(texture))
    }

    // Загрузка текстуры из файла
    pub fn from_path<P:AsRef<Path>>(factory:&mut Display,path:P)->Result<Self,String>{
        let img=image::open(path).map_err(|e|e.to_string())?;

        let img=match img{
            DynamicImage::ImageRgba8(img)=>img,
            img=>img.to_rgba(),
        };

        Texture::from_image(factory,&img).map_err(|e|format!("{:?}", e))
    }

    // Создание текстуры из изображения
    pub fn from_image(factory:&mut Display,img:&RgbaImage)->Result<Self,TextureCreationError>{
        let (width,height)=img.dimensions();
        Texture::create(factory,img,[width,height])
    }

    // Обновляет изображение текстуры, сохраняя размеры
    // При не совпадающих размерых появляются пробелы
    pub fn update(&mut self,img:&RgbaImage){
        let (width,height)=img.dimensions();

        self.0.write(glium::Rect{
                left:0u32,
                bottom:0u32,
                width:width,
                height:height,
            },
            RawImage2d::from_raw_rgba_reversed(img,(width,height)),
        );
    }

    pub fn get_size(&self)->(u32,u32){
        let ref tex=self.0;
        (tex.get_width(),tex.get_height().unwrap())
    }
}