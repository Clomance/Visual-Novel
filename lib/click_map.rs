struct ClickObject{
    x1:f32,
    y1:f32,
    x2:f32,
    y2:f32,
    pressed:bool,
}

pub struct ClickMap{
    objects:Vec<ClickObject>,
}

impl ClickMap{
    pub fn new()->ClickMap{
        Self{
            objects:Vec::new(),
        }
    }

    // Возвращает индекс в массиве
    pub fn add_object(&mut self,[x1,y1,x2,y2]:[f32;4])->usize{
        let object=ClickObject{
            x1,
            y1,
            x2,
            y2,
            pressed:false,
        };

        let len=self.objects.len();
        self.objects.push(object);
        len
    }

    /// Проверка нажатия
    pub fn pressed(&mut self,[x,y]:[f32;2])->Option<usize>{
        for (c,object) in self.objects.iter_mut().enumerate(){
            if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                object.pressed=true;
                return Some(c)
            }
        }
        None
    }

    /// Проверка находится ли курсор на в поле при освобождении кнопки мыши.
    /// Функции лучше подходит название "clicked"
    /// true - clicked
    pub fn released(&mut self,[x,y]:[f32;2])->Option<(usize,bool)>{
        for (c,object) in self.objects.iter_mut().enumerate(){
            if object.pressed{
                object.pressed=false;
                if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                    return Some((c,true))
                }
            }
            else{
                if object.x1<x && object.x2>x && object.y1<y && object.y2>y{
                    return Some((c,false))
                }
            }
        }
        None
    }

    pub fn clear(&mut self){
        self.objects.clear()
    }
}

pub trait Clickable{
    /// [x,y,width,height]
    fn area(&self)->[f32;4];
}