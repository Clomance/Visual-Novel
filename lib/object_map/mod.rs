mod traits;
pub use traits::{
    Drawable,
    ComplexDrawable,
    Clickable,
    ComplexClickable,
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

use std::ops::Range;

/// Тип объекта отрисовки
enum DrawableObjectType{
    Empty,
    /// Простой объект с номером объекта отрисовки
    Simple(usize),
    /// Сложный объект с номерами объектом отрисовки
    Complex(Range<usize>),
}

/// Тип объекта нажатия
enum ClickableObjectType{
    Empty,
    /// Простой объект с номером объекта нажатия
    Simple(usize),
    /// Сложный объект с номерами объектов нажатия
    Complex(Range<usize>),
}

pub struct ObjectMap{
    /// Тип объекта отрисовки или
    /// количество объектов отрисовки для объекта с индексом `i`.
    drawable_type:Vec<DrawableObjectType>,
    /// Тип объекта нажатия или
    /// количество объектов нажатия для объекта с индексом `i`.
    clickable_type:Vec<ClickableObjectType>,
    /// Массив объектов отрисовки.
    drawables:Vec<DrawableObject>,
    click_map:ClickMap,
}

impl ObjectMap{
    pub fn new()->ObjectMap{
        Self{
            drawable_type:Vec::new(),
            clickable_type:Vec::new(),
            drawables:Vec::new(),
            click_map:ClickMap::new(8),
        }
    }

    /// Полностью отчищает всю карту объектов.
    pub fn clear(&mut self){
        self.drawable_type.clear();
        self.clickable_type.clear();
        self.drawables.clear();
        self.click_map.clear()
    }

    pub fn pressed(&mut self,cursor:[f32;2])->Option<usize>{
        if let Some((local_id,object_map_id))=self.click_map.pressed(cursor){
            Some(local_id)
        }
        else{
            None
        }
    }

    pub fn released(&mut self,cursor:[f32;2])->Option<(usize,bool)>{
        if let Some((local_id,object_map_id,pressed))=self.click_map.released(cursor){
            Some((local_id,pressed))
        }
        else{
            None
        }
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut Graphics){
        for object in &self.drawable_type{
            match object{
                DrawableObjectType::Empty=>continue,

                DrawableObjectType::Simple(drawable)=>{
                    let drawable=&self.drawables[drawable.clone()];
                    drawable.draw(draw_parameters,graphics).unwrap();
                }

                DrawableObjectType::Complex(drawables)=>{
                    for drawable in &self.drawables[drawables.clone()]{
                        drawable.draw(draw_parameters,graphics).unwrap();
                    }
                }
            }
        }
    }

    pub fn draw_range(&self,range:Range<usize>,draw_parameters:&DrawParameters,graphics:&mut Graphics){
        for object in &self.drawable_type[range]{
            match object{
                DrawableObjectType::Empty=>continue,

                DrawableObjectType::Simple(drawable)=>{
                    let drawable=&self.drawables[drawable.clone()];
                    drawable.draw(draw_parameters,graphics).unwrap();
                }

                DrawableObjectType::Complex(drawables)=>{
                    for drawable in &self.drawables[drawables.clone()]{
                        drawable.draw(draw_parameters,graphics).unwrap();
                    }
                }
            }
        }
    }
}

/// Добавление объектов.
impl ObjectMap{
    /// Добавляет простой объект, который можно отрисовать.
    pub fn add_raw_simple_drawable_object(
        &mut self,
        index:usize,
        object_type:ObjectType,
        draw_type:DrawType,
    ){
        let drawable_object=DrawableObject::new(index,object_type,draw_type);

        // Индекс подобъекта
        let drawable_index=self.drawables.len();

        self.drawables.push(drawable_object);

        self.drawable_type.push(DrawableObjectType::Simple(drawable_index));
        self.clickable_type.push(ClickableObjectType::Empty);
    }

    /// Добавляет простой объект, который можно отрисовать.
    pub fn add_simple_drawable_object<O:Drawable>(&mut self,object:O){
        let index=object.index();
        let object_type=object.object_type();
        let draw_type=object.draw_type();

        self.add_raw_simple_drawable_object(index,object_type,draw_type);
    }

    /// Добавляет простой объект,
    /// который можно отрисовать и на который можно нажать.
    pub fn add_simple_object<O:Clickable+Drawable>(&mut self,object:O){
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

        // Индексы подобъектов
        let drawable_index=self.drawables.len();
        let clickable_index=self.click_map.len();

        // Индекс объекта
        let object_index=self.drawable_type.len();

        // Добавление подобъектов
        self.drawables.push(drawable_object);
        self.click_map.add_raw_clickable(object_index,click_coords);

        self.drawable_type.push(DrawableObjectType::Simple(drawable_index));
        self.clickable_type.push(ClickableObjectType::Simple(clickable_index));
    }

    /// Добавляет составной объект,
    /// подобъекты которого можно отрисовать
    /// и на которые можно нажать.
    pub fn add_complex_object<O:ComplexDrawable+ComplexClickable>(&mut self,object:O){
        // Массивы подобъектов
        let drawables=object.drawables();
        let clickables=object.clickables();

        // Количество подобъектов
        let drawables_len=drawables.len();
        let clickables_len=clickables.len();

        // Индексы для вставки в массивы (начала областей для сложного объекта)
        let drawables_start=self.drawables.len();
        let clickables_start=self.click_map.len();

        // Индекс объекта
        let object_index=self.drawable_type.len();

        // Области для сложного объекта
        let drawables_range=drawables_start..drawables_start+drawables_len;
        let clickables_range=clickables_start..clickables_start+clickables_len;

        // Добавление подобъектов
        for drawable in drawables{
            self.drawables.push(drawable);
        }
        for clickable in clickables{
            self.click_map.add_object(object_index,clickable);
        }

        self.drawable_type.push(DrawableObjectType::Complex(drawables_range));
        self.clickable_type.push(ClickableObjectType::Complex(clickables_range));
    }
}

/// Замена объектов.
impl ObjectMap{
    /// Заменяет объект отрисовки, не возвращая его
    pub fn set_drawable(&mut self,index:usize,drawable:DrawableObject){
        self.drawables[index]=drawable;
    }
}

/// Получение объектов.
impl ObjectMap{
    /// Возвращает объект отрисовки.
    pub fn get_drawable(&mut self,index:usize)->&mut DrawableObject{
        &mut self.drawables[index]
    }

    /// Возвращает все объекты отрисовки.
    pub fn get_drawables(&mut self)->&mut Vec<DrawableObject>{
        &mut self.drawables
    }
}