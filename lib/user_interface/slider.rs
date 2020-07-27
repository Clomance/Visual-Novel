use super::{
    AlignX,
    AlignY,
    Drawable,
    Red,
    TextViewLine,
    TextViewSettings,
    TextViewStaticLine,
    White,
};

use cat_engine::{
    // statics
    mouse_cursor,
    // types
    Colour,
    // structs
    text::Glyphs,
    graphics::{Graphics,SimpleObject},
    shapes::{
        Circle,
        Line,
    },
    glium::DrawParameters,
};

const circle_radius:f32=16f32;
const circle_diametr:f32=circle_radius*2f32;

const line_radius:f32=5f32;

// Полная комплектация слайдера с надписью и выводом значения
pub struct Slider<'a>{
    head:TextViewStaticLine<'a>, // Надпись над слайдером
    value:TextViewLine<'a>, // Значение справа от слайдера
    glyphs:&'a Glyphs,
    base:SimpleSlider,
}

impl<'a> Slider<'a>{
    pub fn new(settings:SliderSettings,glyphs:&'a Glyphs)->Slider<'a>{
        // Настройки заголовка слайдера
        let head_settings=TextViewSettings::new(settings.head.clone(),[
                    settings.position[0],
                    settings.position[1]-circle_diametr,
                    100f32,
                    0f32,
                ])
                .align_x(AlignX::Left)
                .align_y(AlignY::Down)
                .text_colour(settings.head_colour);

        // Настройки текстового блока со значением слайдера
        let value_settings=TextViewSettings::new(format!("{:.2}",settings.current_value),[
                    settings.position[0]+settings.length,
                    settings.position[1]-circle_radius,
                    100f32,
                    circle_diametr
                ])
                .text_colour(settings.circle_colour);

        Self{
            head:TextViewStaticLine::new(head_settings,&glyphs),
            value:TextViewLine::new(value_settings,&glyphs),
            glyphs:glyphs,
            base:SimpleSlider::new(settings),
        }
    }

    pub fn pressed(&mut self){
        self.base.pressed();
    }

    pub fn released(&mut self)->f32{
        let value=self.base.released();
        self.value.set_text(format!("{:.2}",value),&self.glyphs);
        value
    }

    pub fn grab(&mut self){
        if self.base.grab(){
            let value=self.base.current_value();
            self.value.set_text(format!("{:.2}",value),&self.glyphs);
        }
    }
}

impl<'a> Drawable for Slider<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.head.set_alpha_channel(alpha);
        self.value.set_alpha_channel(alpha);
        self.base.set_alpha_channel(alpha);
    }

    fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.head.draw(draw_parameters,graphics);
        self.value.draw(draw_parameters,graphics,&self.glyphs);
        self.base.draw(draw_parameters,graphics);
    }
}

// Простой слайдер без текстовых блоков
pub struct SimpleSlider{
    min_value:f32,
    step:f32,
    current_value:f32,
    circle:Circle,
    line:Line,
    grab:bool,
}

impl SimpleSlider{
    pub fn new(settings:SliderSettings)->SimpleSlider{
        let step=(settings.max_value-settings.min_value)/settings.length;
        let current_value_line=(settings.current_value-settings.min_value)/step;

        let circle_rect=[
            settings.position[0]+current_value_line,
            settings.position[1],
            circle_radius
        ];

        let line_rect=[
            settings.position[0],
            settings.position[1],
            settings.position[0]+settings.length,
            settings.position[1]
        ];

        Self{
            min_value:settings.min_value,
            step:step,
            current_value:settings.current_value,
            circle:Circle::new(circle_rect,settings.circle_colour),
            line:Line::new(line_rect,line_radius,settings.line_colour),
            grab:false,
        }
    }

    pub fn current_value(&self)->f32{
        self.current_value
    }

    pub fn pressed(&mut self){
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        if self.circle.x-self.circle.radius<x
            && x<self.circle.x+self.circle.radius
            && self.circle.y-self.circle.radius<y
            && y<self.circle.y+self.circle.radius
        {
            // Сдвиг вслед за положением мышки
            if x<self.line.x1{
                self.circle.x=self.line.x1;
            }
            else if x>self.line.x2{
                self.circle.x=self.line.x2;
            }
            else{
                self.circle.x=x;
            }
            self.grab=true;
        }
    }

    pub fn released(&mut self)->f32{
        self.grab=false;
        
        let circle_center=self.circle.x;

        let line=circle_center-self.line.x1;

        self.current_value=line*self.step+self.min_value;

        self.current_value
    }

    // Сдвиг вслед за положением мышки
    pub fn grab(&mut self)->bool{
        if self.grab{
            unsafe{
                let x=mouse_cursor.position()[0];
                // Сдвиг вслед за положением мышки
                if x<self.line.x1{
                    self.circle.x=self.line.x1;
                }
                else if x>self.line.x2{
                    self.circle.x=self.line.x2;
                }
                else{
                    self.circle.x=x;
                }
            }

            // Вычисление текущего значения
            let circle_center=self.circle.x;
            let line=circle_center-self.line.x1;
            self.current_value=line*self.step+self.min_value;

            true
        }
        else{
            false
        }
    }
}

impl Drawable for SimpleSlider{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.circle.colour[3]=alpha;
        self.line.colour[3]=alpha;
    }

    fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.line.draw(draw_parameters,graphics);
        self.circle.draw(draw_parameters,graphics);
    }
}

pub struct SliderSettings{
    head:String,
    head_colour:Colour,
    min_value:f32,
    max_value:f32,
    current_value:f32,
    length:f32, // Длина слайдера (width)
    position:[f32;2],
    circle_colour:Colour,
    line_colour:Colour,
}

impl SliderSettings{
    pub fn new()->SliderSettings{
        Self{
            head:String::new(),
            head_colour:White,
            min_value:0f32,
            max_value:0f32,
            current_value:0f32,
            length:0f32,
            position:[0f32;2],
            circle_colour:Red,
            line_colour:Red,
        }
    }

    pub fn head<S:ToString>(mut self,head:S)->SliderSettings{
        self.head=head.to_string();
        self
    }

    pub fn min_value(mut self,value:f32)->SliderSettings{
        self.min_value=value;
        self
    }

    pub fn max_value(mut self,value:f32)->SliderSettings{
        self.max_value=value;
        self
    }

    pub fn current_value(mut self,value:f32)->SliderSettings{
        self.current_value=value;
        self
    }

    pub fn length(mut self,len:f32)->SliderSettings{
        self.length=len;
        self
    }

    pub fn position(mut self,position:[f32;2])->SliderSettings{
        self.position=position;
        self
    }
}