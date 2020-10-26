use super::ClickableObject;

pub struct ClickMap{
    /// Области объектов.
    objects:Vec<ClickableObject>,
    /// Индексы объектов в `ObjectMap`.
    indices:Vec<usize>,
    /// Состояние объекта (нажат ли).
    pressed:Vec<bool>,
}

impl ClickMap{
    pub fn new(capacity:usize)->ClickMap{
        Self{
            objects:Vec::with_capacity(capacity),
            indices:Vec::with_capacity(capacity),
            pressed:Vec::with_capacity(capacity),
        }
    }

    /// 
    pub fn add_raw_clickable(&mut self,index:usize,[x1,y1,x2,y2]:[f32;4]){
        let object=ClickableObject{
            x1,
            y1,
            x2,
            y2,
        };

        self.objects.push(object);
        self.indices.push(index);
        self.pressed.push(false);
    }

    /// Добавляет объект.
    pub fn add_object(&mut self,index:usize,object:ClickableObject){
        self.objects.push(object);
        self.indices.push(index);
        self.pressed.push(false);
    }

    pub fn delete(&mut self,index:usize){
        self.objects.remove(index);
        self.indices.remove(index);
        self.pressed.remove(index);
    }

    /// Отчистка массива.
    pub fn clear(&mut self){
        self.objects.clear();
        self.indices.clear();
        self.pressed.clear()
    }

    /// Возвращает количество объектов.
    pub fn len(&self)->usize{
        self.objects.len()
    }

    /// Проверка нажатия.
    /// 
    /// Проверка в обратном порядке (с конца)).
    /// 
    /// (usize,usize) - (local_id,object_map_id)
    pub fn pressed(&mut self,[x,y]:[f32;2])->Option<(usize,usize)>{
        // Перебор всех элементов для проверки.
        // Если какой-то был нажат, проверка заканчивается.
        for (c,object) in self.objects.iter_mut().enumerate().rev(){
            if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                self.pressed[c]=true;
                return Some((c,self.indices[c]))
            }
        }
        None
    }

    /// Проверка находится ли курсор на в поле при освобождении кнопки мыши.
    /// 
    /// Проверка в обратном порядке (с конца)).
    /// 
    /// Функции лучше подходит название "clicked"
    /// true - clicked
    /// 
    /// (usize,usize,bool) - (local_id,object_map_id,clicked)
    pub fn released(&mut self,[x,y]:[f32;2])->Option<(usize,usize,bool)>{
        for (c,object) in self.objects.iter_mut().enumerate().rev(){
            if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                let pressed=&mut self.pressed[c]; // Ссылка на параметр
                if *pressed==true{
                    *pressed=false;
                    return Some((c,self.indices[c],true))
                }
                else{
                    return Some((c,self.indices[c],false))
                }
            }
        }
        None
    }
}