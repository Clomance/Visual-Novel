use crate::*;

const page_smooth:f32=Enter_user_name_smooth;

pub struct EnterUserName<'a,'b,'c,'d>{
    head:TextView<'a>,
    input:EditTextView<'b>,
    main_menu:&'c mut MainMenu<'d>
}

impl<'a,'b,'c,'d> EnterUserName<'a,'b,'c,'d>{
    #[inline(always)]
    pub unsafe fn new(main_menu:&'c mut MainMenu<'d>)->EnterUserName<'a,'b,'c,'d>{

        // Загрузка шрифта
        let texture_settings=TextureSettings::new();
        let head_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

        let head_settings=TextViewSettings::new()
                .text("Введите своё имя".to_string())
                .rect([
                    (window_width)/2f64-150f64,
                    (window_height)/2f64-150f64,
                    300f64,
                    70f64,
                ]);

        let glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

        let settings=EditTextViewSettings::new()
                .rect([
                    (window_width)/2f64-150f64,
                    (window_height)/2f64-150f64,
                    300f64,
                    150f64,
                ])
                .background_color(Light_blue)
                .border_color(Blue);

        Self{
            head:TextView::new(head_settings,head_glyphs),
            input:EditTextView::new(settings,glyphs),
            main_menu:main_menu,
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{
        match self.smooth(window){
            Game::Exit=>return Game::Exit,
            Game::Back=>return Game::Back,
            _=>{}
        }

        // Полная отрисовка
        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры
                
                GameWindowEvent::MouseMovementDelta((dx,dy))=>self.main_menu.menu.mouse_shift(dx,dy),

                GameWindowEvent::MouseReleased(button)=>{
                    match button{
                        MouseButton::Left=>{
                            if !self.input.clicked(){
                                return Game::Back
                            }
                        }
                        _=>{}
                    }
                }

                GameWindowEvent::Draw=>{ // Рендеринг
                    window.draw_with_wallpaper(|c,g|{
                        self.main_menu.draw(&c,g);
                        self.input.draw(&c,g);
                        self.head.draw(&c,g);
                    })
                }

                GameWindowEvent::CharacterInput(character)=>self.input.push_char(character),

                GameWindowEvent::KeyboardPressed(button)=>{
                    match button{
                        KeyboardButton::Backspace=>self.input.pop_char(), // Удаление
                        _=>{}
                    }
                }

                GameWindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::Escape=>return Game::Back,
                        
                        KeyboardButton::Enter=>{
                            let name=self.input.get_text();
                            if !name.is_empty(){
                                Settings.user_name=name;
                                return Game::NewGamePlay
                            }
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        }
        Game::Exit
    }

    #[inline(always)] // Сглаживание перехода к странице (открытие)
    pub unsafe fn smooth(&mut self,window:&mut GameWindow)->Game{
        smooth=page_smooth;
        alpha_channel=0f32;

        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::MouseMovementDelta((dx,dy))=>self.main_menu.menu.mouse_shift(dx,dy),

                GameWindowEvent::Draw=>{ // Рендеринг
                    window.draw_with_wallpaper(|c,g|{
                        self.main_menu.draw(&c,g);

                        self.input.draw_smooth(alpha_channel,&c,g);
                        self.head.draw_smooth(alpha_channel,&c,g);
                    });

                    alpha_channel+=smooth;
                    if alpha_channel>1.0{
                        return Game::Current
                    }
                }

                GameWindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::Escape=>return Game::Back,
                        _=>{}
                    }
                }
                _=>{}
            }
        }
        Game::Exit
    }

    #[inline(always)]
    pub unsafe fn close(&mut self,window:&mut GameWindow)->Game{
        smooth=page_smooth;
        alpha_channel=1.0f32;

        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::MouseMovementDelta((dx,dy))=>self.main_menu.menu.mouse_shift(dx,dy),

                GameWindowEvent::Draw=>{ // Рендеринг
                    window.draw_with_wallpaper(|c,g|{
                        self.main_menu.draw(&c,g);

                        self.input.draw_smooth(alpha_channel,&c,g);
                        self.head.draw_smooth(alpha_channel,&c,g);
                    });

                    alpha_channel-=smooth;
                    if alpha_channel<=0.0{
                        return Game::Current
                    }
                }
                _=>{}
            }
        }
        Game::Exit
    }
}