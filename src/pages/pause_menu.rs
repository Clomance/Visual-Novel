use crate::*;

const page_smooth:f32=Pause_menu_smooth;

pub struct PauseMenu<'a>{
    menu:Menu<'a>,
    background:Rectangle,
    background_rect:[f64;4],
}

impl<'a> PauseMenu<'a>{
    #[inline(always)]
    pub unsafe fn new()->PauseMenu<'a>{
        // Загрузка шрифта
        let texture_settings=TextureSettings::new();
        let menu_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        // Создание меню
        let head="Пауза".to_string();
        let head_view_settings=TextViewSettings::new()
                .rect([0f64,0f64,100f64,80f64])
                .text(head)
                .font_size(40)
                .text_color(Head_main_menu_color);

        let menu_settings=MenuSettings::new()
            .buttons_size([180f64,60f64])
            .head_text_settings(head_view_settings)
            .buttons_text(vec![
                "Продолжить".to_string(),
                "Главное меню".to_string(),
                "Настройки".to_string(),
                "Выход".to_string(),
        ]);

        Self{
            menu:Menu::new(menu_settings,menu_glyphs),
            background:Rectangle::new(Pause_menu_background_color),
            background_rect:[
                0f64,
                0f64,
                window_width,
                window_height,
            ]
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{
        'page:while self.smooth(window)!=Game::Exit{

            while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                    GameWindowEvent::Draw=>{ // Рендеринг
                        window.draw(|c,g|{
                            self.background.draw(self.background_rect,&c.draw_state,c.transform,g);
                            self.menu.draw(&c,g);
                        });
                    }

                    GameWindowEvent::MousePressed(button)=>{
                        match button{
                            MouseButton::Left=>{
                                self.menu.pressed();
                            },
                            _=>{}
                        }
                    }

                    GameWindowEvent::MouseReleased(button)=>{
                        match button{
                            MouseButton::Left=>{
                                if let Some(button_id)=self.menu.clicked(){
                                    match button_id{
                                        0=>return Game::ContinueGamePlay, // Кнопка продолжить
                                        1=>return Game::MainMenu, // Кнопка главного меню
                                        2=>{ // Кнопка настроек
                                            match SettingsPage::new().start(window){
                                                Game::Exit=>return Game::Exit,
                                                Game::Back=>continue 'page,
                                                _=>{}
                                            }
                                        }
                                        3=>return Game::Exit, // Кнопка выхода
                                        _=>{}
                                    }
                                }
                            },
                            _=>{}
                        }
                    }

                    GameWindowEvent::KeyboardReleased(button)=>{
                        match button{
                            KeyboardButton::Escape=>return Game::ContinueGamePlay,
                            _=>{}
                        }
                    }
                    _=>{}
                }
            }
        }
        Game::Exit
    }

    #[inline(always)]
    pub unsafe fn smooth(&mut self,window:&mut GameWindow)->Game{
        smooth=page_smooth;
        alpha_channel=0f32;

        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ // Рендеринг
                    window.draw(|c,g|{
                        self.background.color[3]=alpha_channel;
                        self.background.draw(self.background_rect,&c.draw_state,c.transform,g);
                        self.menu.draw_smooth(alpha_channel,&c,g);
                    });

                    alpha_channel+=smooth;
                    if alpha_channel>=1.0{
                        return Game::Current
                    }
                }
                _=>{}
            }
        }
        Game::Exit
    }
}