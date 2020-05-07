use crate::*;

const page_smooth:f32=Enter_user_name_smooth;

pub struct EnterUserName<'a,'b,'c,'d,'e>{
    head:TextViewStaticLineDependent,
    glyphs:GlyphCache<'a>,
    input:EditTextView<'b>,
    main_menu:&'c mut MainMenu<'e,'d>,
    window:*mut GameWindow,
}

impl<'a,'b,'c,'d,'e> EnterUserName<'a,'b,'c,'d,'e>{
    #[inline(always)]
    pub unsafe fn new(main_menu:&'c mut MainMenu<'e,'d>,window:&mut GameWindow)->EnterUserName<'a,'b,'c,'d,'e>{

        // Загрузка шрифта
        let texture_settings=TextureSettings::new();
        let mut head_glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",window.display().clone(),texture_settings).unwrap();

        let head_settings=TextViewSettings::new("Введите своё имя",[
                    (window_width)/2f64-150f64,
                    (window_height)/2f64-150f64,
                    300f64,
                    70f64,
                ]);

        let glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",window.display().clone(),texture_settings).unwrap();

        let settings=EditTextViewSettings::new("",[
                    (window_width)/2f64-150f64,
                    (window_height)/2f64-150f64,
                    300f64,
                    150f64,
                ])
                .background_color(Light_blue)
                .border_color(Some(Blue));

        Self{
            head:TextViewStaticLineDependent::new(head_settings,&mut head_glyphs),
            glyphs:head_glyphs,
            input:EditTextView::new(settings,glyphs),
            main_menu:main_menu,
            window:window as *mut GameWindow,
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self)->Game{
        match self.smooth(){
            Game::Exit=>return Game::Exit,
            Game::Back=>return Game::Back,
            _=>{}
        }

        // Полная отрисовка
        while let Some(event)=(*self.window).next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                    self.main_menu.wallpaper.mouse_shift(dx,dy);
                    self.main_menu.menu.mouse_shift(dx,dy)
                }

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
                    (*self.window).draw(|c,g|{
                        self.main_menu.draw(c,g);
                        self.input.draw(c,g);
                        self.head.draw(c,g,&mut self.glyphs);
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
                            let name=self.input.text().clone();
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
    pub unsafe fn smooth(&mut self)->Game{
        (*self.window).set_new_smooth(page_smooth);

        while let Some(event)=(*self.window).next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::MouseMovementDelta((dx,dy))=>self.main_menu.menu.mouse_shift(dx,dy),

                GameWindowEvent::Draw=>{ // Рендеринг
                    if 1f32<(*self.window).draw_smooth(|alpha,c,g|{
                        self.main_menu.draw(c,g);

                        self.input.draw_smooth(alpha,c,g);
                        self.head.draw_smooth(alpha,c,g,&mut self.glyphs);
                    }){
                        break
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
        Game::Current
    }
}