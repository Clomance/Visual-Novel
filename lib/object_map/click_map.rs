use super::ClickableObject;

pub struct ClickMap{
    objects:Vec<ClickableObject>,
    pressed:Vec<bool>,
}

impl ClickMap{
    pub fn new()->ClickMap{
        Self{
            objects:Vec::new(),
            pressed:Vec::new(),
        }
    }

    /// Добавляет объект.
    /// 
    /// Возвращает индекс в массиве.
    pub fn add_object(&mut self,[x1,y1,x2,y2]:[f32;4])->usize{
        let object=ClickableObject{
            x1,
            y1,
            x2,
            y2,
        };

        let len=self.objects.len();
        self.objects.push(object);
        self.pressed.push(false);
        len
    }

    /// Отчистка массива.
    pub fn clear(&mut self){
        self.objects.clear();
        self.pressed.clear()
    }

    /// Возвращает количество объектов.
    pub fn len(&self)->usize{
        self.objects.len()
    }

    /// Проверка нажатия.
    /// 
    /// Проверка в обратном порядке (с конца)).
    pub fn pressed(&mut self,[x,y]:[f32;2])->Option<usize>{
        // Перебор всех элементов для проверки.
        // Если какой-то был нажат, проверка заканчивается.
        for (c,object) in self.objects.iter_mut().enumerate().rev(){
            if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                self.pressed[c]=true;
                return Some(c)
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
    pub fn released(&mut self,[x,y]:[f32;2])->Option<(usize,bool)>{
        for (c,object) in self.objects.iter_mut().enumerate().rev(){
            if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                let pressed=&mut self.pressed[c]; // Ссылка на параметр
                if *pressed==true{
                    *pressed=false;
                    return Some((c,true))
                }
                else{
                    return Some((c,false))
                }
            }
        }
        None
    }
}