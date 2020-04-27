use crate::*;

const dcolor:f32=0.125; // На столько измененяется цвет при нажитии/освобождении

pub struct Button<'a>{
    base:ButtonDependent,
    glyphs:GlyphCache<'a>
}

impl<'a> Button<'a>{
    pub fn new(settings:ButtonSettings,mut glyphs:GlyphCache<'a>)->Button<'a>{
        Self{
            base:ButtonDependent::new(settings,&mut glyphs),
            glyphs:glyphs,
        }
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        self.base.shift(dx,dy)
    }

    pub fn pressed(&mut self)->bool{
        self.base.pressed()
    }

    // Проверка находится ли курсор на кнопке и локальные действия
    pub fn released(&mut self)->bool{ // лучше подходит название "clicked"
        self.base.released()
    }
}

impl<'a> Drawable for Button<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
    }

    fn draw(&mut self,context:&Context,g:&mut GlGraphics){
        self.base.draw(context,g,&mut self.glyphs)
    }
}

// Зависимая от шрифта кнопка для связанных структур (должно быть больше зависимостей)
pub struct ButtonDependent{
    base:ButtonBase,
    text:TextViewDependent<TextLine>, // Зависимый от шрифта текстовый блок
}

impl ButtonDependent{
    pub fn new(settings:ButtonSettings,glyphs:&mut GlyphCache)->ButtonDependent{
        let text_view_settings=TextViewSettings::new()
                .rect(settings.rect)
                .text_color(settings.text_color)
                .text(settings.text)
                .font_size(settings.font_size);
        Self{
            base:ButtonBase::new(settings.rect,settings.background_color),
            text:TextViewDependent::new(text_view_settings,glyphs),
        }
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        self.base.shift(dx,dy);
        self.text.shift(dx,dy)
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
        self.text.set_alpha_channel(alpha);
    }

    pub fn pressed(&mut self)->bool{
        self.base.pressed()
    }

    // Проверка находится ли курсор на кнопке и локальные действия
    pub fn released(&mut self)->bool{ // лучше подходит название "clicked"
        self.base.released()
    }
    
    pub fn draw(&mut self,context:&Context,graphics:&mut GlGraphics,glyphs:&mut GlyphCache){
        self.base.draw(context,graphics);
        self.text.draw(context,graphics,glyphs);
    }
}

#[derive(Clone)]
pub struct ButtonSettings{
    pub rect:[f64;4],
    pub background_color:Color,
    pub text:String,
    pub font_size:u32,
    pub text_color:Color
}

impl ButtonSettings{
    pub fn new()->ButtonSettings{
        Self{
            rect:[0f64;4],
            background_color:Light_blue,
            text:String::new(),
            font_size:20,
            text_color:Black,
        }
    }

    pub fn rect(mut self,rect:[f64;4])->ButtonSettings{
        self.rect=rect;
        self
    }

    pub fn background_color(mut self,color:Color)->ButtonSettings{
        self.background_color=color;
        self
    }

    pub fn text(mut self,text:String)->ButtonSettings{
        self.text=text;
        self
    }

    pub fn font_size(mut self,size:u32)->ButtonSettings{
        self.font_size=size;
        self
    }
    
    pub fn text_color(mut self,color:Color)->ButtonSettings{
        self.text_color=color;
        self
    }
}

// Второе название JmyakButton - предложил Тимур Шайхинуров
// Кнопка, в которую вписывается крестик при нажатии
pub struct CheckButton{
    button_base:ButtonBase,
    tick_color:Color,
    ticked:bool
}

impl CheckButton{
    pub fn new(rect:[f64;4],background_color:Color,ticked:bool)->CheckButton{
        Self{
            button_base:ButtonBase::new(rect,background_color),
            tick_color:Red,
            ticked:ticked
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.button_base.set_alpha_channel(alpha)
    }

    pub fn clicked(&mut self)->bool{
        if self.button_base.released(){
            self.ticked=!self.ticked;
            true
        }
        else{
            false
        }
    }

    pub fn draw(&self,context:&Context,g:&mut GlGraphics){
        self.button_base.draw(context,g);
        if self.ticked{
            let line=Line::new(self.tick_color,1f64);
            
            line.draw(
                [
                    self.button_base.x1,
                    self.button_base.y1,
                    self.button_base.x2,
                    self.button_base.y2
                ],
                &context.draw_state,
                context.transform,
                g
            );

            line.draw(
                [
                    self.button_base.x1,
                    self.button_base.y2,
                    self.button_base.x2,
                    self.button_base.y1
                ],
                &context.draw_state,
                context.transform,
                g
            )
        }
    }
}

// Основа для кнопок
struct ButtonBase{
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    width:f64,
    height:f64,
    rectangle:Rectangle,
    pressed:bool,
}

impl ButtonBase{
    #[inline(always)]
    pub fn new(rect:[f64;4],color:Color)->ButtonBase{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            width:rect[2],
            height:rect[3],
            rectangle:Rectangle::new(color),
            pressed:false,
        }
    }

    #[inline(always)] // Сдвиг
    pub fn shift(&mut self,dx:f64,dy:f64){
        self.x1+=dx;
        self.y1+=dy;
        self.x2+=dx;
        self.y2+=dy;
    }

    #[inline(always)] // Установка альфа-канала
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.rectangle.color[3]=alpha;
    }

    #[inline(always)] // Установка цвета
    pub fn set_color(&mut self,color:Color){
        self.rectangle.color=color
    }

    #[inline(always)] // Изменение цвета при нажатии
    pub fn press_color(&mut self){
        self.rectangle.color[0]-=dcolor;
        self.rectangle.color[1]-=dcolor;
        self.rectangle.color[2]-=dcolor;
    }

    #[inline(always)] // Изменение цвета при освобождении
    pub fn release_color(&mut self){
        self.rectangle.color[0]+=dcolor;
        self.rectangle.color[1]+=dcolor;
        self.rectangle.color[2]+=dcolor;
    }

    #[inline(always)] // Проверка нажатия на кнопку и локальные действия
    pub fn pressed(&mut self)->bool{
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        if self.x1<x && self.x2>x && self.y1<y && self.y2>y{
            self.pressed=true;
            self.press_color();
            true
        }
        else{
            false
        }
    }

    #[inline(always)] // Проверка находится ли курсор на кнопке и локальные действия
    pub fn released(&mut self)->bool{ // лучше подходит название "clicked"
        if self.pressed{
            self.release_color();
            self.pressed=false;

            let position=unsafe{mouse_cursor.position()};
            let x=position[0];
            let y=position[1];

            if self.x1<x && self.x2>x && self.y1<y && self.y2>y{
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
    pub fn draw(&self,context:&Context,g:&mut GlGraphics){
        let rect_pos=[self.x1,self.y1,self.width,self.height];
        self.rectangle.draw(rect_pos,&context.draw_state,context.transform,g);
    }
}