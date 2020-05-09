use crate::*;

use lib::game_engine::text::Glyphs;

const page_smooth:f32=Pause_menu_smooth;

const background_color:Color=Pause_menu_background_color;

pub struct PauseMenu<'a>{
    menu:Menu<'a>,
}

impl<'a> PauseMenu<'a>{
    pub unsafe fn new()->PauseMenu<'a>{
        // Загрузка шрифта
        let menu_glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");
        
        // Настройка меню
        let menu_settings=MenuSettings::new("Пауза",&["Продолжить","Главное меню","Настройки","Выход"])
            .head_size([180f64,80f64])
            .buttons_size([180f64,60f64]);

        Self{
            menu:Menu::new(menu_settings,menu_glyphs),
        }
    }

    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{
        'page:loop{
            match self.smooth(window){
                Game::Exit=>return Game::Exit,
                Game::Back=>return Game::Back,
                _=>{}
            }
            while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                    GameWindowEvent::Draw=>{ // Рендеринг
                        window.draw(|c,g|{
                            g.clear_color(background_color);
                            self.menu.draw(c,g);
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
    }

    pub unsafe fn smooth(&mut self,window:&mut GameWindow)->Game{
        window.set_new_smooth(page_smooth);

        let mut background=Background::new(background_color,[
            0f64,
            0f64,
            window_width,
            window_height
        ]);

        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ // Рендеринг
                    if 1f32<window.draw_smooth(|alpha,c,g|{
                        background.draw_smooth(alpha,c,g);
                        self.menu.draw_smooth(alpha,c,g);
                    }){
                        break
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
        Game::Current
    }
}