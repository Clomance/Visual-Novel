use std::path::Path;

use glium::{
    Display,
    texture::{
        RawImage2d,
        TextureCreationError,
        srgb_texture2d::SrgbTexture2d
    },
};

use image::{RgbaImage,DynamicImage};
use image::error::ImageError;

/// Результат создания текстуры.
/// The result of texture creation.
pub enum TextureCreationResult{
    Ok(Texture),
    TextureCreationError(TextureCreationError),
    ImageCreationError(ImageError)
}

impl TextureCreationResult{
    pub fn unwrap(self)->Texture{
        match self{
            TextureCreationResult::Ok(texture)=>texture,
            TextureCreationResult::TextureCreationError(e)=>panic!(e),
            TextureCreationResult::ImageCreationError(e)=>panic!(e),
        }
    }

    pub fn expect(self,msg:&str)->Texture{
        match self{
            TextureCreationResult::Ok(texture)=>texture,
            TextureCreationResult::TextureCreationError(e)=>panic!("{} {:?}",msg,e),
            TextureCreationResult::ImageCreationError(e)=>panic!("{} {:?}",msg,e),
        }
    }
}

/// Обёртка для 2D текстуры. Wrapper for 2D texture.
pub struct Texture(pub SrgbTexture2d);

impl Texture{
    /// Создаёт текстуру из массива байт.
    /// 
    /// Creates a texture from byte array.
    pub fn create<S:Into<[u32;2]>>(memory:&[u8],size:S,factory:&Display)->TextureCreationResult{
        let [w,h]=size.into();

        let image=RawImage2d::from_raw_rgba_reversed(memory,(w,h));

        match SrgbTexture2d::new(factory,image){
            Ok(texture)=>TextureCreationResult::Ok(Texture(texture)),
            Err(e)=>{
                TextureCreationResult::TextureCreationError(e)
            }
        }
    }

    /// Загружает текстуру из файла.
    /// 
    /// Loading a texture from file.
    pub fn from_path<P:AsRef<Path>>(path:P,factory:&Display)->TextureCreationResult{
        match image::open(path){
            Ok(image)=>{
                let image=match image{
                    DynamicImage::ImageRgba8(img)=>img,
                    img=>img.to_rgba(),
                };
                Texture::from_image(&image,factory)
            },
            Err(e)=>TextureCreationResult::ImageCreationError(e)
        }
    }

    /// Создаёт текстуру из изображения.
    /// 
    /// Creates a texture from given image.
    pub fn from_image(img:&RgbaImage,factory:&Display)->TextureCreationResult{
        let (width,height)=img.dimensions();
        Texture::create(img,[width,height],factory)
    }

    /// Обновляет изображение текстуры, сохраняя размеры.
    /// При не совпадающих размераx возникают ошибки.
    /// 
    /// Updates the texture with the new image.
    /// If the sizes aren't equal something bad can happen :)
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

    /// Возвращает ширину изображения.
    /// 
    /// Returns the width of the image.
    pub fn width(&self)->u32{
        self.0.width()
    }

    /// Возвращает высоту изображения.
    /// 
    /// Returns the height of the image.
    pub fn height(&self)->u32{
        self.0.height()
    }

    /// Возвращает ширину и высоту изображения.
    /// 
    /// Returns the width and height of the image.
    pub fn dimensions(&self)->(u32,u32){
        self.0.dimensions()
    }
}