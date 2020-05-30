use super::*;

use engine::{
    // fns
    window_rect,
    // types
    Colour,
    // structs
    graphics::Graphics,
    mouse_cursor,
    text::Glyphs,
    glium::DrawParameters,
};

const head_margin:f32=50f32; // Расстояние между заголовком и кнопками
const button_margin:f32=10f32; // Расстояние между кнопками
const dmargin:f32=head_margin-button_margin; // Для расчёта высоты меню - чтобы не вычитать button_margin

const menu_movement_scale:f32=10f32; // Обратный коэфициент сдвига меню при движении мышью

// Меню, состоящее из заголовка и кнопок под ним
pub struct Menu<'a>{
    head:TextViewStaticLine<'a>,
    buttons:Vec<Button<'a>>,
}

impl<'a> Menu<'a>{
    pub fn new<'c,S:Into<String>,B:Into<String>+Clone+'a>(settings:MenuSettings<'c,S,B>,glyphs:&'a Glyphs)->Menu<'a>{
        let r=unsafe{mouse_cursor.center_radius()}; //
        let dx=r[0]/menu_movement_scale;            // Сдвиг относительно положения мыши
        let dy=r[1]/menu_movement_scale;            //

        let x0=settings.rect[0]+dx;     //
        let y0=settings.rect[1]+dy;     // Положение и размер
        let width=settings.rect[2];     // области для вставки
        let height=settings.rect[3];    //

        // Полная высота меню
        let menu_height=settings.head_size[1]+dmargin+(settings.buttons_size[1]+button_margin)*settings.buttons_text.len() as f32;

        // Положение заголовка по Y
        let mut y=match settings.align.y{
            AlignY::Up=>y0,
            AlignY::Center=>y0+(height-menu_height)/2f32,
            AlignY::Down=>y0+height-menu_height,
        };

        // Положение заголовка по X
        let mut x=match settings.align.x{
            AlignX::Right=>x0+width-settings.head_size[0],
            AlignX::Center=>x0+(width-settings.head_size[0])/2f32,
            AlignX::Left=>x0,
        };

        // Настройки для заголовка
        let head_settings=TextViewSettings::new(settings.head_text,
                [
                    x,
                    y,
                    settings.head_size[0],
                    settings.head_size[1]
                ])
                .align_x(settings.align.x.clone())
                .font_size(settings.head_font_size)
                .text_colour(settings.head_text_color);

        // Положение верней кнопки по Y
        y+=settings.head_size[1]+head_margin;

        // Положение кнопок по X
        x=match settings.align.x{
            AlignX::Right=>x0+width-settings.buttons_size[0],
            AlignX::Center=>x0+(width-settings.buttons_size[0])/2f32,
            AlignX::Left=>x0,
        };

        // Массив кнопок
        let mut menu_buttons=Vec::with_capacity(settings.buttons_text.len());

        // Положение и размер кнопок
        let mut button_rect=[
            x,
            y,
            settings.buttons_size[0],
            settings.buttons_size[1]
        ];

        // Создание кнопок
        for text in settings.buttons_text{
            let text=text.clone();
            // Настройки кнопок (text.to_string() странно работает: требует fmt::Display без to_string)
            let button_sets=ButtonSettings::<String>::new(text.into(),button_rect)
                    .background_colour(settings.buttons_color)
                    .font_size(settings.buttons_font_size);

            let button=Button::new(button_sets,&glyphs);
            menu_buttons.push(button);
            button_rect[1]+=settings.buttons_size[1]+button_margin;
        }

        Self{
            head:TextViewStaticLine::new(head_settings,&glyphs),
            buttons:menu_buttons,
        }
    }

    // Проверка: нажата ли кнопка в меню
    pub fn pressed(&mut self)->Option<usize>{
        for (c,button) in self.buttons.iter_mut().enumerate(){
            if button.pressed(){
                return Some(c)
            }
        }
        None
    }

    // Проверка: завершён ли клик по кнопке
    pub fn clicked(&mut self)->Option<usize>{
        for (c,button) in self.buttons.iter_mut().enumerate(){
            if button.released(){
                return Some(c)
            }
        }
        None
    }

    // Сдвиг с коэффициентом
    pub fn mouse_shift(&mut self,dx:f32,dy:f32){
        let dx=dx/menu_movement_scale;
        let dy=dy/menu_movement_scale;
        self.head.shift(dx,dy);
        for button in &mut self.buttons{
            button.shift(dx,dy)
        }
    }
}

impl<'a> Drawable for Menu<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.head.set_alpha_channel(alpha);
        
        for button in &mut self.buttons{
            button.set_alpha_channel(alpha);
        }
    }

    fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.head.draw(draw_parameters,graphics);

        for button in &self.buttons{
            button.draw(draw_parameters,graphics);
        }
    }
}

// Настройки меню
pub struct MenuSettings<'a,S:Into<String>,B:Into<String>+'a>{
    rect:[f32;4], // [x1,y1,width,height] - сюда встроивается меню, по умочанию размер окна
    align:Align, // Выравнивание меню
    head_text:S, // Текст заголовка меню
    head_size:[f32;2], // Ширина и высота заголовка
    head_font_size:f32,
    head_text_color:Colour,
    buttons_size:[f32;2], // [width,height], по умолчанию [100, 60]
    buttons_text:&'a [B],
    buttons_font_size:f32,
    buttons_color:Colour,
}

impl<'a,S:Into<String>,B:Into<String>+'a> MenuSettings<'a,S,B>{
    pub fn new(head:S,buttons:&'a [B])->MenuSettings<'a,S,B>{
        Self{
            rect:window_rect(),
            head_text:head,
            head_size:[100f32,60f32],
            head_font_size:40f32,
            head_text_color:White,
            align:Align::center(),
            buttons_size:[100f32,60f32],
            buttons_text:buttons,
            buttons_font_size:18f32,
            buttons_color:Light_blue,
        }
    }

    pub fn rect(mut self,rect:[f32;4])->MenuSettings<'a,S,B>{
        self.rect=rect;
        self
    }

    pub fn head_size(mut self,size:[f32;2])->MenuSettings<'a,S,B>{
        self.head_size=size;
        self
    }

    pub fn align_x(mut self,align:AlignX)->MenuSettings<'a,S,B>{
        self.align.x=align;
        self
    }

    pub fn align_y(mut self,align:AlignY)->MenuSettings<'a,S,B>{
        self.align.y=align;
        self
    }

    pub fn buttons_size(mut self,size:[f32;2])->MenuSettings<'a,S,B>{
        self.buttons_size=size;
        self
    }

    pub fn buttons_font_size(mut self,size:f32)->MenuSettings<'a,S,B>{
        self.buttons_font_size=size;
        self
    }
}