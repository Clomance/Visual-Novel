#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,dead_code)]

use glutin::window::Icon;

mod sync_raw_ptr;
pub use sync_raw_ptr::SyncRawPtr;

mod traits;
pub use traits::*;

mod colors;
pub use colors::*;



use image::{
    self,
    DynamicImage,
    RgbaImage,
    imageops::FilterType,
    GenericImageView
};

use std::path::Path;

// Загрузка изображений
pub fn load_image<P:AsRef<Path>>(path:P,width:u32,height:u32)->RgbaImage{
    let mut image=image::open(path).unwrap();
    
    image=image.resize_exact(width,height,FilterType::Gaussian);
    if let DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}

pub fn load_window_icon()->Icon{
    let image=image::open("./images/window_icon.png").unwrap();
    let vec=image.to_bytes();
    let width=image.width();
    let height=image.height();

    Icon::from_rgba(vec,width,height).unwrap()

}