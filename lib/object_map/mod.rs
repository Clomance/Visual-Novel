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
    Colour,
    glium::DrawParameters,
    graphics::{
        ObjectType,
        DrawType,
        Graphics,
        ColourFilter,
    }
};

use std::ops::Range;

/// Тип объекта отрисовки
enum DrawableObjectType{
    Empty,
    /// Простой объект с номером объекта отрисовки
    Simple{
        layer:usize,
        index:usize,
    },
    /// Сложный объект с номерами объектом отрисовки
    Complex{
        layer:usize,
        range:Range<usize>
    },
}

/// Тип объекта нажатия
enum ClickableObjectType{
    Empty,
    /// Простой объект с номером объекта нажатия
    Simple(usize),
    /// Сложный объект с номерами объектов нажатия
    Complex(Range<usize>),
}

struct DrawableLayer{
    drawables:Vec<DrawableObject>,
    /// Цветовой фильтр
    filter:ColourFilter,
}

impl DrawableLayer{
    pub fn new()->DrawableLayer{
        Self{
            drawables:Vec::new(),
            filter:ColourFilter::new_mul([1f32;4]),
        }
    }
}

pub struct ObjectMap{
    /// Тип объекта отрисовки или
    /// количество объектов отрисовки для объекта с индексом `i`.
    drawable_object_types:Vec<Vec<DrawableObjectType>>,
    /// Тип объекта нажатия или
    /// количество объектов нажатия для объекта с индексом `i`.
    clickable_object_types:Vec<ClickableObjectType>,
    /// Массив слоёв объектов отрисовки.
    drawables:Vec<DrawableLayer>,
    click_map:ClickMap,
}

impl ObjectMap{
    pub fn new()->ObjectMap{
        Self{
            drawable_object_types:Vec::new(),
            clickable_object_types:Vec::new(),
            drawables:Vec::new(),
            click_map:ClickMap::new(8),
        }
    }

    pub fn add_new_layer(&mut self){
        self.drawable_object_types.push(Vec::new());
        self.drawables.push(DrawableLayer::new())
    }

    /// Устанавливает цвет для фильтра.
    pub fn set_layer_colour(&mut self,layer:usize,colour:Colour){
        self.drawables[layer].filter.colour=colour
    }

    pub fn set_len(&mut self,len:usize){
        unsafe{
            self.drawable_object_types.set_len(len);
            self.clickable_object_types.set_len(len);
        }
    }

    /// Отчищает все слоит от объектов.
    pub fn clear_layers(&mut self){
        for c in 0..self.drawable_object_types.len(){
            self.drawable_object_types[c].clear();
            self.drawables[c].drawables.clear();
        }
    }

    pub fn clear_click_map(&mut self){
        self.clickable_object_types.clear();
        self.click_map.clear()
    }

    /// Полностью отчищает всю карту объектов и удаляет слои.
    pub fn clear(&mut self){
        self.drawable_object_types.clear();
        self.clickable_object_types.clear();
        self.drawables.clear();
        self.click_map.clear()
    }

    pub fn pressed(&mut self,cursor:[f32;2])->Option<(usize)>{
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

    /// Рендеринг.
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        // Перебор слоёв
        for layer in &self.drawable_object_types{
            // Перебор объектов в слое
            for object in layer{
                // Определение типа объекта
                match object{
                    // Пустой
                    DrawableObjectType::Empty=>continue,
                    // Простой
                    DrawableObjectType::Simple{
                        layer,
                        index,
                    }=>{
                        // Ссылка на слой подобъектов
                        let layer=&self.drawables[layer.clone()];
                        // Ссылка на объект отрисовки
                        let drawable=&layer.drawables[index.clone()];
                        // Рендеринг
                        drawable.draw(layer.filter,draw_parameters,graphics).unwrap();
                    }
                    // Составной
                    DrawableObjectType::Complex{
                        layer,
                        range,
                    }=>{
                        // Ссылка на слой подобъектов
                        let layer=&self.drawables[layer.clone()];
                        // Перебор подобъектов
                        for drawable in &layer.drawables[range.clone()]{
                            // Рендеринг
                            drawable.draw(layer.filter,draw_parameters,graphics).unwrap();
                        }
                    }
                }
            }
        }
    }

    // pub fn draw_range(&self,range:Range<usize>,draw_parameters:&DrawParameters,graphics:&mut Graphics){
    //     for object in &self.drawable_object_types[range]{
    //         match object{
    //             DrawableObjectType::Empty=>continue,

    //             DrawableObjectType::Simple(drawable)=>{
    //                 let drawable=&self.drawables[drawable.clone()];
    //                 drawable.draw(draw_parameters,graphics).unwrap();
    //             }

    //             DrawableObjectType::Complex(drawables)=>{
    //                 for drawable in &self.drawables[drawables.clone()]{
    //                     drawable.draw(draw_parameters,graphics).unwrap();
    //                 }
    //             }
    //         }
    //     }
    // }
}

/// Добавление объектов.
impl ObjectMap{
    /// Добавляет простой объект, который можно отрисовать.
    pub fn add_raw_simple_drawable_object(
        &mut self,
        layer:usize,
        index:usize,
        object_type:ObjectType,
        draw_type:DrawType,
    ){
        let drawable_object=DrawableObject::new(index,object_type,draw_type);

        // Индекс подобъекта
        let drawable_index=self.drawables[layer].drawables.len();

        self.drawables[layer].drawables.push(drawable_object);

        self.drawable_object_types[layer].push(DrawableObjectType::Simple{index:drawable_index,layer});
        self.clickable_object_types.push(ClickableObjectType::Empty);
    }

    /// Добавляет простой объект, который можно отрисовать.
    pub fn add_simple_drawable_object<O:Drawable>(&mut self,layer:usize,object:O){
        let index=object.index();
        let object_type=object.object_type();
        let draw_type=object.draw_type();

        self.add_raw_simple_drawable_object(layer,index,object_type,draw_type);
    }

    /// Добавляет простой объект,
    /// который можно отрисовать и на который можно нажать.
    pub fn add_simple_object<O:Clickable+Drawable>(&mut self,layer:usize,object:O){
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
        let drawable_index=self.drawables[layer].drawables.len();
        let clickable_index=self.click_map.len();

        // Индекс объекта
        let object_index=self.drawable_object_types[layer].len();

        // Добавление подобъектов
        self.drawables[layer].drawables.push(drawable_object);
        self.click_map.add_raw_clickable(object_index,click_coords);

        self.drawable_object_types[layer].push(DrawableObjectType::Simple{index:drawable_index,layer});
        self.clickable_object_types.push(ClickableObjectType::Simple(clickable_index));
    }

    /// Добавляет составной объект,
    /// подобъекты которого можно отрисовать
    /// и на которые можно нажать.
    pub fn add_complex_object<O:ComplexDrawable+ComplexClickable>(&mut self,layer:usize,object:O){
        // Массивы подобъектов
        let drawables=object.drawables();
        let clickables=object.clickables();

        // Количество подобъектов
        let drawables_len=drawables.len();
        let clickables_len=clickables.len();

        // Индексы для вставки в массивы (начала областей для сложного объекта)
        let drawables_start=self.drawables[layer].drawables.len();
        let clickables_start=self.click_map.len();

        // Индекс объекта
        let object_index=self.drawable_object_types.len();

        // Области для сложного объекта
        let drawables_range=drawables_start..drawables_start+drawables_len;
        let clickables_range=clickables_start..clickables_start+clickables_len;

        // Добавление подобъектов
        for drawable in drawables{
            self.drawables[layer].drawables.push(drawable);
        }
        for clickable in clickables{
            self.click_map.add_object(object_index,clickable);
        }

        self.drawable_object_types[layer].push(DrawableObjectType::Complex{range:drawables_range,layer});
        self.clickable_object_types.push(ClickableObjectType::Complex(clickables_range));
    }
}

/// Замена объектов.
impl ObjectMap{
    // Удаление объекта и всех его подобъектов.
    // pub fn delete_object(&mut self,index:usize){
    //     let drawable_object=self.drawable_object_types.remove(index);
    //     let clickable_object=self.clickable_object_types.remove(index);

    //     match drawable_object{
    //         DrawableObjectType::Empty=>{},
    //         DrawableObjectType::Simple(index)=>{
    //             self.drawables.remove(index);
    //         }
    //         DrawableObjectType::Complex(indices)=>{
    //             for index in indices{
    //                 self.drawables.remove(index);
    //             }
    //         }
    //     }

    //     match clickable_object{
    //         ClickableObjectType::Empty=>{},
    //         ClickableObjectType::Simple(index)=>{
    //             self.click_map.delete(index);
    //         }
    //         ClickableObjectType::Complex(indices)=>{
    //             for index in indices{
    //                 self.click_map.delete(index);
    //             }
    //         }
    //     }
    // }
}


/// Замена объектов.
impl ObjectMap{
    /// Заменяет объект отрисовки, не возвращая его
    pub fn set_drawable(&mut self,layer:usize,index:usize,drawable:DrawableObject){
        self.drawables[layer].drawables[index]=drawable;
    }
}

/// Получение объектов.
impl ObjectMap{
    /// Возвращает объект отрисовки.
    pub fn get_drawable(&mut self,layer:usize,index:usize)->&mut DrawableObject{
        &mut self.drawables[layer].drawables[index]
    }

    /// Возвращает все объекты отрисовки.
    pub fn get_drawables(&mut self,layer:usize)->&mut Vec<DrawableObject>{
        &mut self.drawables[layer].drawables
    }
}