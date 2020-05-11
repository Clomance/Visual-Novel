use std::path::Path;

use glium::{
    backend::Facade,
    uniforms::SamplerWrapFunction,
    texture::{RawImage2d,TextureCreationError,srgb_texture2d::SrgbTexture2d},
};

use graphics::ImageSize;

pub use texture::TextureSettings;
use texture::{
    self,
    Wrap
};

use image::{RgbaImage,DynamicImage};

pub struct Texture(pub SrgbTexture2d,pub [SamplerWrapFunction;2]); // Wrapper for 2D texture.

impl Texture{
    pub const fn new(texture:SrgbTexture2d)->Texture{
        Texture(texture,[SamplerWrapFunction::Clamp;2])
    }

    pub fn create<S:Into<[u32;2]>,F:Facade>(factory:&mut F,memory:&[u8],size:S,settings:&TextureSettings)->Result<Self,TextureCreationError>{
        let size=size.into();

        let f=|wrap|match wrap{
            Wrap::ClampToEdge=>SamplerWrapFunction::Clamp,
            Wrap::Repeat=>SamplerWrapFunction::Repeat,
            Wrap::MirroredRepeat=>SamplerWrapFunction::Mirror,
            Wrap::ClampToBorder=>SamplerWrapFunction::BorderClamp,
        };

        let wrap_u=f(settings.get_wrap_u());
        let wrap_v=f(settings.get_wrap_v());

        let image=RawImage2d::from_raw_rgba_reversed(memory,(size[0],size[1]));

        let texture=SrgbTexture2d::new(factory,image)?;

        Ok(Texture(texture,[wrap_u, wrap_v]))
    }

    /// Creates a texture from path.
    pub fn from_path<F:Facade,P:AsRef<Path>>(factory:&mut F,path:P,settings:&TextureSettings)->Result<Self,String>{
        let img=image::open(path).map_err(|e|e.to_string())?;

        let img=match img{
            DynamicImage::ImageRgba8(img)=>img,
            img=>img.to_rgba(),
        };

        Texture::from_image(factory,&img,settings).map_err(|e|format!("{:?}", e))
    }

    /// Creates a texture from image.

    pub fn from_image<F:Facade>(factory:&mut F,img:&RgbaImage,settings:&TextureSettings)->Result<Self,TextureCreationError>{
        let (width,height)=img.dimensions();
        Texture::create(factory,img,[width,height],settings)
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
}

impl ImageSize for Texture{
    fn get_size(&self)->(u32,u32){
        let ref tex=self.0;
        (tex.get_width(),tex.get_height().unwrap())
    }
}