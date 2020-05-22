use super::{
    Black,
    Drawable,
    Light_blue,
    TextViewSettings,
    TextViewStaticLineDependent,
};

use engine::{
    // statics
    mouse_cursor,
    // types
    Colour,
    // structs
    text::Glyphs,
    graphics::{
        GameGraphics,
        Rectangle,
    },
    glium::DrawParameters,
};

const dcolour:f32=0.125; // На столько измененяется цвет при нажитии/освобождении

pub struct Button<'a>{
    base:ButtonDependent,
    glyphs:Glyphs<'a>
}

impl<'a> Button<'a>{
    pub fn new<S:Into<String>>(settings:ButtonSettings<S>,mut glyphs:Glyphs<'a>)->Button<'a>{
        Self{
            base:ButtonDependent::new(settings,&mut glyphs),
            glyphs:glyphs,
        }
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.base.shift(dx,dy)
    }

    #[inline(always)]
    pub fn pressed(&mut self)->bool{
        self.base.pressed()
    }

    #[inline(always)] // Проверка находится ли курсор на кнопке и локальные действия
    pub fn released(&mut self)->bool{ // лучше подходит название "clicked"
        self.base.released()
    }
}

impl<'a> Drawable for Button<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
    }

    fn draw(&mut self,draw_parameters:&mut DrawParameters,g:&mut GameGraphics){
        self.base.draw(draw_parameters,g,&mut self.glyphs)
    }
}

// Зависимая от шрифта кнопка для связанных структур (должно быть больше зависимостей)
pub struct ButtonDependent{
    base:ButtonBase,
    text:TextViewStaticLineDependent, // Зависимый от шрифта текстовый блок
}

impl ButtonDependent{
    pub fn new<S:Into<String>>(settings:ButtonSettings<S>,glyphs:&Glyphs)->ButtonDependent{
        let text_view_settings=TextViewSettings::new(settings.text,settings.rect)
                .text_colour(settings.text_colour)
                .font_size(settings.font_size);
        Self{
            base:ButtonBase::new(settings.rect,settings.background_colour),
            text:TextViewStaticLineDependent::new(text_view_settings,glyphs),
        }
    }

    pub fn shift(&mut self,dx:f32,dy:f32){
        self.base.shift(dx,dy);
        self.text.shift(dx,dy)
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
        self.text.set_alpha_channel(alpha);
    }

    #[inline(always)]
    pub fn pressed(&mut self)->bool{
        self.base.pressed()
    }

    #[inline(always)] // Проверка находится ли курсор на кнопке и локальные действия
    pub fn released(&mut self)->bool{ // лучше подходит название "clicked"
        self.base.released()
    }
    
    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics,glyphs:&Glyphs){
        self.base.draw(draw_parameters,graphics);
        self.text.draw(draw_parameters,graphics,glyphs);
    }
}

// Основа для кнопок
struct ButtonBase{
    rect:Rectangle,
    pressed:bool,
}

impl ButtonBase{
    pub fn new(rect:[f32;4],colour:Colour)->ButtonBase{
        Self{
            rect:Rectangle::new(rect,colour),
            pressed:false,
        }
    }

    // Сдвиг
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.rect.x1+=dx;
        self.rect.y1+=dy;
        self.rect.x2+=dx;
        self.rect.y2+=dy;
    }

    #[inline(always)] // Установка альфа-канала
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.rect.colour[3]=alpha;
    }

    // Изменение цвета при нажатии
    pub fn press_colour(&mut self){
        self.rect.colour[0]-=dcolour;
        self.rect.colour[1]-=dcolour;
        self.rect.colour[2]-=dcolour;
    }

    // Изменение цвета при освобождении
    pub fn release_colour(&mut self){
        self.rect.colour[0]+=dcolour;
        self.rect.colour[1]+=dcolour;
        self.rect.colour[2]+=dcolour;
    }

    // Проверка нажатия на кнопку и локальные действия
    pub fn pressed(&mut self)->bool{
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        if self.rect.x1<x && self.rect.x2>x && self.rect.y1<y && self.rect.y2>y{
            self.pressed=true;
            self.press_colour();
            true
        }
        else{
            false
        }
    }

    // Проверка находится ли курсор на кнопке при освобождении кнопки мыши
    // и локальные действия
    // Функции лучше подходит название "clicked"
    pub fn released(&mut self)->bool{
        if self.pressed{
            self.release_colour();
            self.pressed=false;

            let position=unsafe{mouse_cursor.position()};
            let x=position[0];
            let y=position[1];

            if self.rect.x1<x && self.rect.x2>x && self.rect.y1<y && self.rect.y2>y{
                true
            }
            else{
                false
            }
        }
        else{
            false
        }
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,g:&mut GameGraphics){
        self.rect.draw(draw_parameters,g);
    }
}

// Настройки для построения кнопок
pub struct ButtonSettings<S:Into<String>>{
    rect:[f32;4],
    background_colour:Colour,
    text:S,
    font_size:f32,
    text_colour:Colour
}

impl<S:Into<String>> ButtonSettings<S>{
    pub fn new(text:S,rect:[f32;4])->ButtonSettings<S>{
        Self{
            rect,
            background_colour:Light_blue,
            text,
            font_size:20f32,
            text_colour:Black,
        }
    }

    pub fn background_colour(mut self,colour:Colour)->ButtonSettings<S>{
        self.background_colour=colour;
        self
    }
    
    pub fn font_size(mut self,size:f32)->ButtonSettings<S>{
        self.font_size=size;
        self
    }
    
    pub fn text_colour(mut self,colour:Colour)->ButtonSettings<S>{
        self.text_colour=colour;
        self
    }
}