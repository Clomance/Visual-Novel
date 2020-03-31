use crate::*;

const head_margin:f64=50f64;
const button_margin:f64=10f64;

pub struct Menu<'a>{
    head:TextViewDependent,
    buttons:Vec<ButtonDependent>,
    glyphs:GlyphCache<'a>
}

impl<'a> Menu<'a>{

    pub fn new(mut settings:MenuSettings,mut glyphs:GlyphCache<'a>)->Menu<'a>{
        let x0=settings.rect[0];
        let y0=settings.rect[1];
        let width=settings.rect[2];
        let height=settings.rect[3];

        let head_x=settings.head_text_view_settings.rect[0];
        let head_y=settings.head_text_view_settings.rect[1];

        let mut x1=head_x+x0+(width-settings.head_text_view_settings.rect[2])/2f64; // Для заголовка
        settings.head_text_view_settings.rect[0]=x1;

        x1=x0+(width-settings.buttons_size[0])/2f64; // Для кнопок

        // Полная длина меню
        let mut menu_height=settings.head_text_view_settings.rect[3]+head_margin; // rect[3] - Высота заголовка
        menu_height+=(settings.buttons_size[1]+button_margin)*settings.buttons_text.len() as f64-button_margin;

        let mut y1=head_y+y0+(height-menu_height)/2f64; // Для заголовка
        settings.head_text_view_settings.rect[1]=y1;
        y1+=settings.head_text_view_settings.rect[3]+head_margin;

        let mut rect=[
            x1,
            y1,
            settings.buttons_size[0],
            settings.buttons_size[1]
        ];

        let mut menu_buttons=Vec::with_capacity(settings.buttons_text.len());

        let button_settings=ButtonSettings::new()
                .background_color(Light_blue)
                .font_size(settings.buttons_font_size);

        for text in settings.buttons_text{
            let button_sets=button_settings.clone().text(text).rect(rect);
            let button=ButtonDependent::new(button_sets,&mut glyphs);
            menu_buttons.push(button);
            rect[1]+=settings.buttons_size[1]+button_margin;
        }

        Self{
            head:TextViewDependent::new(settings.head_text_view_settings,&mut glyphs),
            buttons:menu_buttons,
            glyphs:glyphs,
        }
    }

    pub fn clicked(&mut self)->Option<usize>{
        for (c,button) in self.buttons.iter_mut().enumerate(){
            if button.clicked(){
                return Some(c)
            }
        }
        None
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        self.head.draw(draw_state,transform,g,&mut self.glyphs);

        for button in &mut self.buttons{
            button.draw(draw_state,transform,g,&mut self.glyphs);
        }
    }
}

// Настройки меню
pub struct MenuSettings{
    pub rect:[f64;4], // [x1,y1,width,height] - сюда встроивается меню, по умочанию размер окна
    pub head_text_view_settings:TextViewSettings,
    pub buttons_size:[f64;2], // [width,height], по умолчанию [100, 60]
    pub buttons_text:Vec<String>,
    pub buttons_font_size:u32,
}

impl MenuSettings{
    pub fn new()->MenuSettings{
        Self{
            rect:unsafe{[0f64,0f64,Settings.window_size.width,Settings.window_size.height]},
            head_text_view_settings:TextViewSettings::new(),
            buttons_size:[100f64,60f64],
            buttons_text:Vec::new(),
            buttons_font_size:20,
        }
    }

    pub fn rect(mut self,rect:[f64;4])->MenuSettings{
        self.rect=rect;
        self
    }
    
    pub fn head_text_settings(mut self,settings:TextViewSettings)->MenuSettings{
        self.head_text_view_settings=settings;
        self
    }
    
    pub fn buttons_size(mut self,size:[f64;2])->MenuSettings{
        self.buttons_size=size;
        self
    }

    pub fn buttons_text(mut self,text:Vec<String>)->MenuSettings{
        self.buttons_text=text;
        self
    }

    pub fn buttons_font_size(mut self,size:u32)->MenuSettings{
        self.buttons_font_size=size;
        self
    }
}