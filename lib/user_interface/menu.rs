use crate::{
    Drawable,
    DrawableObject,
    ComplexDrawable,
    Clickable,
    ClickableObject,
    ComplexClickable,
};

use super::{
    TextView,
    TextViewSettings,
    Button,
    ButtonSettings,
    Align,
    AlignX,
    AlignY,
    White,
    Light_blue,
    GeneralSettings,
};

use cat_engine::{
    // fns
    window_rect,
    // types
    Colour,
    // structs
    graphics::{Graphics2D,DrawType},
};

const head_margin:f32=50f32; // Расстояние между заголовком и кнопками
const button_margin:f32=10f32; // Расстояние между кнопками
const dmargin:f32=head_margin-button_margin; // Для расчёта высоты меню - чтобы не вычитать button_margin

pub struct Menu{
    pub header:TextView,
    pub buttons:Vec<Button>
}

impl Menu{
    pub fn new<S:Into<String>,BS:Into<String>,B:Iterator<Item=BS>>(
        settings:MenuSettings<S,BS,B>,
        graphics:&mut Graphics2D,
    )->Menu{
        let buttons_text:Vec<String>=settings.buttons_text.into_iter().map(|t|t.into()).collect();

        let x0=settings.rect[0];        //
        let y0=settings.rect[1];        // Положение и размер
        let width=settings.rect[2];     // области для вставки
        let height=settings.rect[3];    //

        // Полная высота меню
        let menu_height=settings.header_size[1]+dmargin+(settings.buttons_size[1]+button_margin)*buttons_text.len() as f32;

        // Положение заголовка по Y
        let mut y=match settings.align.y{
            AlignY::Up=>y0,
            AlignY::Center=>y0+(height-menu_height)/2f32,
            AlignY::Down=>y0+height-menu_height,
        };

        // Положение заголовка по X
        let mut x=match settings.align.x{
            AlignX::Right=>x0+width-settings.header_size[0],
            AlignX::Center=>x0+(width-settings.header_size[0])/2f32,
            AlignX::Left=>x0,
        };

        // Настройки для заголовка
        let head_settings=TextViewSettings::new(settings.header_text,
                [
                    x,
                    y,
                    settings.header_size[0],
                    settings.header_size[1]
                ])
                .draw_type(settings.general.draw_type.clone())
                .align_x(settings.align.x.clone())
                .font_size(settings.header_font_size)
                .font(settings.font)
                .text_colour(settings.header_text_colour);

        // Положение верней кнопки по Y
        y+=settings.header_size[1]+head_margin;

        // Положение кнопок по X
        x=match settings.align.x{
            AlignX::Right=>x0+width-settings.buttons_size[0],
            AlignX::Center=>x0+(width-settings.buttons_size[0])/2f32,
            AlignX::Left=>x0,
        };

        // Массив кнопок
        let mut buttons=Vec::with_capacity(buttons_text.len());

        // Положение и размер кнопок
        let mut button_rect=[
            x,
            y,
            settings.buttons_size[0],
            settings.buttons_size[1]
        ];

        // Создание кнопок
        for text in buttons_text.into_iter(){
            let button_sets=ButtonSettings::<String>::new(text,button_rect)
                    .draw_type(settings.general.draw_type.clone())
                    .background_colour(settings.buttons_colour)
                    .font(settings.font)
                    .font_size(settings.buttons_font_size);

            let button=Button::new(button_sets,graphics);
            buttons.push(button);
            button_rect[1]+=settings.buttons_size[1]+button_margin;
        }


        Self{
            header:TextView::new(head_settings,graphics),
            buttons,
        }
    }
}

impl ComplexDrawable for Menu{
    fn drawables(&self)->Vec<DrawableObject>{
        let mut drawables=Vec::with_capacity(self.buttons.len()+1);

        drawables.push(self.header.drawable());

        for button in &self.buttons{
            let button_drawables=button.drawables();
            for drawable in button_drawables{
                drawables.push(drawable)
            }
        }

        drawables
    }
}

impl ComplexClickable for Menu{
    fn clickables(&self)->Vec<ClickableObject>{
        let mut clickables=Vec::with_capacity(self.buttons.len());

        for button in &self.buttons{
            clickables.push(button.clickable())
        }

        clickables
    }
}

// Настройки меню
pub struct MenuSettings<S:Into<String>,BS:Into<String>,B:Iterator<Item=BS>>{
    general:GeneralSettings,
    rect:[f32;4], // [x1,y1,width,height] - сюда встроивается меню, по умочанию размер окна
    align:Align, // Выравнивание меню
    header_text:S, // Текст заголовка меню
    header_size:[f32;2], // Ширина и высота заголовка
    font:usize, // Номер сохранённого шрифта
    header_font_size:f32,
    header_text_colour:Colour,
    buttons_size:[f32;2], // [width,height], по умолчанию [100, 60]
    buttons_text:B,
    buttons_font_size:f32,
    buttons_colour:Colour,
}

impl<S:Into<String>,BS:Into<String>,B:Iterator<Item=BS>> MenuSettings<S,BS,B>{
    pub fn new(head:S,buttons:B)->MenuSettings<S,BS,B>{
        Self{
            general:GeneralSettings::new(),
            rect:window_rect(),
            header_text:head,
            header_size:[100f32,60f32],
            font:0usize,
            header_font_size:40f32,
            header_text_colour:White,
            align:Align::center(),
            buttons_size:[100f32,60f32],
            buttons_text:buttons,
            buttons_font_size:18f32,
            buttons_colour:Light_blue,
        }
    }

    pub fn draw_type(mut self,draw_type:DrawType)->MenuSettings<S,BS,B>{
        self.general.draw_type=draw_type;
        self
    }

    pub fn rect(mut self,rect:[f32;4])->MenuSettings<S,BS,B>{
        self.rect=rect;
        self
    }

    pub fn header_size(mut self,size:[f32;2])->MenuSettings<S,BS,B>{
        self.header_size=size;
        self
    }

    pub fn align_x(mut self,align:AlignX)->MenuSettings<S,BS,B>{
        self.align.x=align;
        self
    }

    pub fn align_y(mut self,align:AlignY)->MenuSettings<S,BS,B>{
        self.align.y=align;
        self
    }

    pub fn buttons_size(mut self,size:[f32;2])->MenuSettings<S,BS,B>{
        self.buttons_size=size;
        self
    }

    pub fn buttons_font_size(mut self,size:f32)->MenuSettings<S,BS,B>{
        self.buttons_font_size=size;
        self
    }
}