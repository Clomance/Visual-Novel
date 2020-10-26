use cat_engine::glium::{
    DrawError,
    DrawParameters,
};

use cat_engine::graphics::{
    Graphics,
    DrawType,
    ObjectType,
    ColourFilter,
};

#[derive(Clone)]
pub struct DrawableObject{
    pub index:usize,
    pub object_type:ObjectType,
    pub draw_type:DrawType
}

impl DrawableObject{
    pub fn new(index:usize,object_type:ObjectType,draw_type:DrawType)->DrawableObject{
        Self{
            index,
            object_type,
            draw_type
        }
    }

    pub fn set_draw_type(&mut self,draw_type:DrawType){
        self.draw_type=draw_type
    }

    pub fn update_rotating_angle(&mut self,new_angle:f32){
        if let DrawType::Rotating((angle,_))=&mut self.draw_type{
            *angle=new_angle;
        }
    }

    pub fn draw(
        &self,
        colour_filter:ColourFilter,
        draw_parameters:&DrawParameters,
        graphics:&mut Graphics
    )->Result<(),DrawError>{
        graphics.draw_object(
            self.index,
            self.object_type.clone(),
            self.draw_type.clone(),
            colour_filter,
            draw_parameters
        )
    }
}

#[derive(Clone)]
pub struct ComplexDrawableObject{
    pub objects:Vec<DrawableObject>
}

/// Объект для проверки нажатий.
/// 
/// Содержит координаты верхней левой и 
/// нижней правой точек прямоугольника.
pub struct ClickableObject{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
}

impl ClickableObject{
    pub fn new([x1,y1,x2,y2]:[f32;4])->ClickableObject{
        Self{
            x1,
            y1,
            x2,
            y2,
        }
    }
}