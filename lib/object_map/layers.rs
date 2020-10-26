use super::{
    DrawableObjectType,
    DrawableObject,
};

use cat_engine::graphics::ColourFilter;

/// Слой для подобъектов.
pub struct DrawableLayer{
    pub drawables:Vec<DrawableObject>,
    /// Цветовой фильтр
    pub filter:ColourFilter,
}

impl DrawableLayer{
    pub fn new()->DrawableLayer{
        Self{
            drawables:Vec::new(),
            filter:ColourFilter::new_mul([1f32;4]),
        }
    }
}

/// Слой для объектов.
/// 
/// Можно при надобности отключить.
pub struct DrawableObjectLayer{
    pub enabled:bool,
    pub objects:Vec<DrawableObjectType>,
}

impl DrawableObjectLayer{
    pub fn new()->DrawableObjectLayer{
        Self{
            enabled:true,
            objects:Vec::new()
        }
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }
}