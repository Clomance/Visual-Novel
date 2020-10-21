mod traits;
pub use traits::{
    Drawable,
    ComplexDrawable,
    Clickable,
};

mod objects;
pub use objects::{
    DrawableObject,
    ClickableObject,
};

mod click_map;
pub use click_map::{
    ClickMap,
};

use cat_engine::{
    glium::DrawParameters,
    graphics::{
        ObjectType,
        DrawType,
        Graphics,
    }
};

pub struct ObjectMap{
    pub drawable:Vec<DrawableObject>,
    pub click_map:ClickMap,
}

impl ObjectMap{
    pub fn new()->ObjectMap{
        Self{
            drawable:Vec::new(),
            click_map:ClickMap::new(),
        }
    }

    pub fn add_object<O:Clickable+Drawable>(&mut self,object:O){
        let index=object.index();
        let object_type=object.object_type();
        let draw_type=object.draw_type();

        let drawable_object=DrawableObject::new(index,object_type,draw_type);

        let click_area=object.area();

        let click_coords=[
            click_area[0],
            click_area[1],
            click_area[0]+click_area[2],
            click_area[1]+click_area[3],
        ];

        self.drawable.push(drawable_object);

        self.click_map.add_object(click_coords);
    }

    pub fn clear(&mut self){
        self.drawable.clear();
        self.click_map.clear()
    }

    pub fn pressed(&mut self,cursor:[f32;2])->Option<usize>{
        self.click_map.pressed(cursor)
    }

    pub fn released(&mut self,cursor:[f32;2])->Option<(usize,bool)>{
        self.click_map.released(cursor)
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut Graphics){
        for drawable in &self.drawable{
            drawable.draw(draw_parameters,graphics).unwrap();
        }
    }
}

/// Drawable Object
impl ObjectMap{
    pub fn add_drawable(&mut self,index:usize,object_type:ObjectType,draw_type:DrawType){
        let drawable_object=DrawableObject::new(index,object_type,draw_type);

        self.drawable.push(drawable_object);
    }

    pub fn add_drawable_object<O:Drawable>(&mut self,object:O){
        let index=object.index();
        let object_type=object.object_type();
        let draw_type=object.draw_type();

        let drawable_object=DrawableObject::new(index,object_type,draw_type);

        self.drawable.push(drawable_object);
    }

    /// Убирает из массива отрисовки объект
    pub fn remove_drawable(&mut self,index:usize)->DrawableObject{
        self.drawable.remove(index)
    }

    /// Заменяет объект
    pub fn set_drawable_object(&mut self,index:usize,object:DrawableObject){
        self.drawable[index]=object
    }

    pub fn get_drawable(&mut self,index:usize)->&mut DrawableObject{
        &mut self.drawable[index]
    }

    pub fn get_drawables(&mut self)->&mut Vec<DrawableObject>{
        &mut self.drawable
    }
}