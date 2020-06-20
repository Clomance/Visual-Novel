use std::path::Path;

use glium::{
    Display,
    texture::{
        RawImage2d,
        TextureCreationError as CrateTextureCreationError,
        srgb_texture2d::SrgbTexture2d
    },
};

use image::{RgbaImage,DynamicImage};
use image::error::ImageError;

/// Error that can happen when creating a texture.
#[derive(Debug)]
pub enum TextureCreationError{
    TextureError(CrateTextureCreationError),
    ImageError(ImageError)
}

/// Обёртка для 2D текстуры.
pub struct Texture(pub SrgbTexture2d);

impl Texture{
    /// Создание текстуры из массива байт.
    pub fn create<S:Into<[u32;2]>>(factory:&Display,memory:&[u8],size:S)->Result<Self,TextureCreationError>{
        let [w,h]=size.into();

        let image=RawImage2d::from_raw_rgba_reversed(memory,(w,h));

        match SrgbTexture2d::new(factory,image){
            Ok(texture)=>Ok(Texture(texture)),
            Err(e)=>{
                let error=TextureCreationError::TextureError(e);
                return Err(error)
            }
        }
    }

    /// Загрузка текстуры из файла.
    pub fn from_path<P:AsRef<Path>>(path:P,factory:&Display)->Result<Self,TextureCreationError>{
        match image::open(path){
            Ok(image)=>{
                let image=match image{
                    DynamicImage::ImageRgba8(img)=>img,
                    img=>img.to_rgba(),
                };
                Texture::from_image(&image,factory)
            },
            Err(e)=>return Err(TextureCreationError::ImageError(e))
        }
    }

    /// Создание текстуры из изображения.
    pub fn from_image(img:&RgbaImage,factory:&Display)->Result<Self,TextureCreationError>{
        let (width,height)=img.dimensions();
        Texture::create(factory,img,[width,height])
    }

    /// Обновляет изображение текстуры, сохраняя размеры.
    /// При не совпадающих размерых появляются пробелы.
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