use crate::*;

const circle_radius:f64=16f64;
const circle_diametr:f64=circle_radius*2f64;

const line_radius:f64=5f64;

// Полная комплектация слайдера с надписью и выводом значения
pub struct Slider<'a>{
    head:TextViewDependent<TextLine>, // Надпись над слайдером
    value:TextViewDependent<TextLine>, // Значение слева от слайдера
    glyphs:GlyphCache<'a>,
    base:SimpleSlider,
}

impl<'a> Slider<'a>{
    pub fn new(settings:SliderSettings,mut glyphs:GlyphCache<'a>)->Slider<'a>{
        let rect=[
            settings.position[0],
            settings.position[1]-circle_diametr,
            100f64,
            0f64,
        ];

        let head_view_settings=TextViewSettings::new()
                .align_x(AlignX::Left)
                .align_y(AlignY::Down)
                .rect(rect)
                .text(settings.head.clone())
                .text_color(settings.head_color);

        let rect=[
            settings.position[0]+settings.length,
            settings.position[1]-circle_radius,
            100f64,
            circle_diametr
        ];


        let value_view_settings=TextViewSettings::new()
                .rect(rect)
                .text(format!("{:.2}",settings.current_value))
                .text_color(settings.circle_color);

        Self{
            head:TextViewDependent::new(head_view_settings,&mut glyphs),
            value:TextViewDependent::new(value_view_settings,&mut glyphs),
            glyphs:glyphs,
            base:SimpleSlider::new(settings),
        }
    }

    pub fn pressed(&mut self){
        self.base.pressed();
    }

    pub fn released(&mut self)->f64{
        let value=self.base.released();
        self.value.set_text_raw(format!("{:.2}",value));
        value
    }

    pub fn grab(&mut self){
        self.base.grab();
    }
}

impl<'a> Drawable for Slider<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.head.set_alpha_channel(alpha);
        self.value.set_alpha_channel(alpha);
        self.base.set_alpha_channel(alpha);
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.head.draw(context,graphics,&mut self.glyphs);
        self.value.draw(context,graphics,&mut self.glyphs);
        self.base.draw(context,graphics);
    }
}

pub struct SimpleSlider{
    min_value:f64,
    step:f64,
    current_value:f64,
    circle:Ellipse,
    circle_rect:[f64;4], // x1, y1, width, height
    line:Line,
    line_rect:[f64;4], // x1, y1, x2, y2
    grab:bool,
}

impl SimpleSlider{
    pub fn new(settings:SliderSettings)->SimpleSlider{
        let step=(settings.max_value-settings.min_value)/settings.length;
        let current_value_line=(settings.current_value-settings.min_value)/step;

        let circle_border=Border{
            color:Black,
            radius:1f64,
        };

        let circle_rect=[
            settings.position[0]+current_value_line-circle_radius,
            settings.position[1]-circle_radius,
            circle_diametr,
            circle_diametr
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
            circle:Ellipse::new(settings.circle_color).border(circle_border),
            circle_rect:circle_rect,
            line:Line::new_round(settings.line_color,line_radius),
            line_rect:line_rect,
            grab:false,
        }
    }

    pub fn pressed(&mut self){
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        if self.circle_rect[0]<x && x<self.circle_rect[0]+circle_diametr &&
                self.circle_rect[1]<y && y<self.circle_rect[1]+circle_diametr{
            // Сдвиг вслед за положением мышки
            if x<self.line_rect[0]{
                self.circle_rect[0]=self.line_rect[0]-circle_radius;
            }
            else if x>self.line_rect[2]{
                self.circle_rect[0]=self.line_rect[2]-circle_radius;
            }
            else{
                self.circle_rect[0]=x-circle_radius;
            }
            self.grab=true;
        }
    }

    pub fn released(&mut self)->f64{
        self.grab=false;
        
        let circle_center=self.circle_rect[0]+circle_radius;

        let line=circle_center-self.line_rect[0];

        self.current_value=line*self.step+self.min_value;

        self.current_value
    }

    // Сдвиг вслед за положением мышки
    pub fn grab(&mut self){
        if self.grab{
            unsafe{
                let x=mouse_cursor.position()[0];
                if x<self.line_rect[0]{
                    self.circle_rect[0]=self.line_rect[0]-circle_radius;
                }
                else if x>self.line_rect[2]{
                    self.circle_rect[0]=self.line_rect[2]-circle_radius;
                }
                else{
                    self.circle_rect[0]=x-circle_radius;
                }
            }
        }
    }
}

impl Drawable for SimpleSlider{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.circle.color[3]=alpha;
        self.line.color[3]=alpha;
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        self.line.draw(self.line_rect,&context.draw_state,context.transform,graphics);
        self.circle.draw(self.circle_rect,&context.draw_state,context.transform,graphics);
    }
}

pub struct SliderSettings{
    head:String,
    head_color:Color,
    min_value:f64,
    max_value:f64,
    current_value:f64,
    length:f64,
    position:[f64;2],
    circle_color:Color,
    line_color:Color,
}

impl SliderSettings{
    pub fn new()->SliderSettings{
        Self{
            head:String::new(),
            head_color:White,
            min_value:0f64,
            max_value:0f64,
            current_value:0f64,
            length:0f64,
            position:[0f64;2],
            circle_color:Red,
            line_color:Red,
        }
    }

    pub fn head<S:ToString>(mut self,head:S)->SliderSettings{
        self.head=head.to_string();
        self
    }

    pub fn min_value(mut self,value:f64)->SliderSettings{
        self.min_value=value;
        self
    }

    pub fn max_value(mut self,value:f64)->SliderSettings{
        self.max_value=value;
        self
    }

    pub fn current_value(mut self,value:f64)->SliderSettings{
        self.current_value=value;
        self
    }

    pub fn length(mut self,len:f64)->SliderSettings{
        self.length=len;
        self
    }

    pub fn position(mut self,position:[f64;2])->SliderSettings{
        self.position=position;
        self
    }
}