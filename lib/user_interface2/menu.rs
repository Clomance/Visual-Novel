

// Меню, состоящее из заголовка и кнопок под ним
pub struct Menu<'a>{
    pub head:TextViewStaticLine<'a>,
    pub buttons:Vec<Button<'a>>,
}

impl<'a> Menu<'a>{
    pub fn new<'c,S:Into<String>,B:Into<String>+Clone+'a>(
        settings:MenuSettings<'c,S,B>,
        font:&'a Font<'static>
    )->Menu<'a>{
        let x0=settings.rect[0];        //
        let y0=settings.rect[1];        // Положение и размер
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

            let button=Button::new(button_sets,&font);
            menu_buttons.push(button);
            button_rect[1]+=settings.buttons_size[1]+button_margin;
        }

        Self{
            head:TextViewStaticLine::new(head_settings,&font),
            buttons:menu_buttons,
        }
    }

    pub fn shift(&mut self,[dx,dy]:[f32;2]){
        self.head.shift(dx,dy);

        for button in self.buttons.iter_mut(){
            button.shift(dx,dy)
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