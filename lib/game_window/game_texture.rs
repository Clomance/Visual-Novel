use glium::{
    backend::Facade,
    uniforms::SamplerWrapFunction,
    texture::{RawImage2d,TextureCreationError,srgb_texture2d::SrgbTexture2d},
};

use graphics::ImageSize;

use texture::{
    self,
    CreateTexture,
    Format,
    TextureOp,
    TextureSettings,
    UpdateTexture
};

use image::{RgbaImage,DynamicImage};

use std::path::Path;

/// Flip settings.
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Flip{
    None, // Does not flip.
    Vertical, // Flips image vertically.
}

pub struct Texture(pub SrgbTexture2d,pub [SamplerWrapFunction;2]); // Wrapper for 2D texture.

impl Texture{
    /// Creates a new `Texture`.
    pub fn new(texture: SrgbTexture2d) -> Texture {
        Texture(texture, [SamplerWrapFunction::Clamp; 2])
    }

    /// Returns empty texture.
    pub fn empty<F:Facade>(factory:&mut F)->Result<Self,TextureCreationError>{
        CreateTexture::create(factory,Format::Rgba8,&[0u8; 4],[1, 1],&TextureSettings::new())
    }

    /// Creates a texture from path.
    pub fn from_path<F:Facade,P:AsRef<Path>>(factory:&mut F,path:P,flip:Flip,settings:&TextureSettings)->Result<Self,String>{
        let img=image::open(path).map_err(|e|e.to_string())?;

        let img=match img {
            DynamicImage::ImageRgba8(img)=>img,
            img=>img.to_rgba(),
        };

        let img=if flip == Flip::Vertical {
            image::imageops::flip_vertical(&img)
        } else {
            img
        };

        Texture::from_image(factory, &img, settings).map_err(|e| format!("{:?}", e))
    }

    /// Creates a texture from image.

    pub fn from_image<F:Facade>(factory:&mut F,img:&RgbaImage,settings:&TextureSettings)->Result<Self,TextureCreationError>{
        let (width, height) = img.dimensions();
        CreateTexture::create(factory, Format::Rgba8, img, [width, height], settings)
    }

    /// Creates texture from memory alpha.
    pub fn from_memory_alpha<F: Facade>(
        factory: &mut F,
        buffer: &[u8],
        width: u32,
        height: u32,
        settings: &TextureSettings,
    ) -> Result<Self, TextureCreationError>{
        if width == 0 || height == 0 {
            return Texture::empty(factory);
        }

        let size = [width, height];
        let buffer = texture::ops::alpha_to_rgba8(buffer, size);
        CreateTexture::create(factory, Format::Rgba8, &buffer, size, settings)
    }

    /// Updates texture with an image.
    pub fn update(&mut self,img:&RgbaImage){
        let (width,height)=img.dimensions();
        let (_, h)=self.get_size();

        self.0.write(glium::Rect{
                left:0u32,
                bottom:h-height,
                width:width,
                height:height,
            },
            RawImage2d::from_raw_rgba_reversed(img,(width,height)),
        );
    }
}

impl ImageSize for Texture {
    fn get_size(&self)->(u32,u32){
        let ref tex=self.0;
        (tex.get_width(), tex.get_height().unwrap())
    }
}

impl<F> TextureOp<F> for Texture {
    type Error = TextureCreationError;
}

impl<F: Facade> CreateTexture<F> for Texture{
    fn create<S: Into<[u32; 2]>>(factory: &mut F,_format: Format,memory: &[u8],size: S,settings: &TextureSettings) -> Result<Self, Self::Error> {
        use texture::Wrap;
        let size = size.into();

        let f = |wrap| match wrap {
            Wrap::ClampToEdge => SamplerWrapFunction::Clamp,
            Wrap::Repeat => SamplerWrapFunction::Repeat,
            Wrap::MirroredRepeat => SamplerWrapFunction::Mirror,
            Wrap::ClampToBorder => SamplerWrapFunction::BorderClamp,
        };

        let wrap_u = f(settings.get_wrap_u());
        let wrap_v = f(settings.get_wrap_v());
        Ok(Texture(
            SrgbTexture2d::new(
                factory,
                RawImage2d::from_raw_rgba_reversed(memory, (size[0], size[1]))
            )?,
            [wrap_u, wrap_v],
        ))
    }
}

impl<F:Facade> UpdateTexture<F> for Texture{
    fn update<O:Into<[u32;2]>,S:Into<[u32;2]>>(
        &mut self,
        _factory:&mut F,
        _format:Format,
        memory:&[u8],
        offset:O,
        size:S,
    )->Result<(),Self::Error>{
        let offset=offset.into();
        let size=size.into();
        let (_,h)=self.get_size();
        self.0.write(
            glium::Rect{
                left:offset[0],
                bottom:h-offset[1]-size[1],
                width:size[0],
                height:size[1],
            },
            RawImage2d::from_raw_rgba_reversed(memory,(size[0],size[1])),
        );
        Ok(())
    }
}
