use super::{
    DrawableObject,
    ClickableObject
};

use cat_engine::graphics::{
    DrawType,
    ObjectType,
};

pub trait Drawable:Sized{
    fn index(&self)->usize;
    fn object_type(&self)->ObjectType;
    fn draw_type(&self)->DrawType;

    fn drawable(&self)->DrawableObject{
        DrawableObject{
            index:self.index(),
            object_type:self.object_type(),
            draw_type:self.draw_type(),
        }
    }
}

pub trait ComplexDrawable{
    fn drawables(&self)->Vec<DrawableObject>;
}

/// Типаж для определения объектов для нажатий.
pub trait Clickable{
    /// [x,y,width,height]
    fn area(&self)->[f32;4];

    fn clickable(&self)->ClickableObject{
        let [x,y,width,height]=self.area();
        ClickableObject::new([
            x,
            y,
            x+width,
            y+height,
        ])
    }
}

pub trait ComplexClickable{
    fn clickables(&self)->Vec<ClickableObject>;
}
