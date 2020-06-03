use crate::{
    Main_font,
    make_screenshot,
    Game,
};

use super::{
    SettingsPage,
    default_page_smooth,
};

use lib::{
    colours::Pause_menu_background_colour,
    Drawable,
    Menu,
    MenuSettings,
};

use engine::{
    // fns
    window_rect,
    // types
    Colour,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
    music::Music,
    // structs
    Window,
    graphics::Rectangle,
};

const page_smooth:f32=default_page_smooth;

const background_color:Colour=Pause_menu_background_colour;

pub struct PauseMenu<'a>{
    menu:Menu<'a>,
}

impl<'a> PauseMenu<'a>{
    pub unsafe fn new()->PauseMenu<'a>{
        // Настройка меню
        let menu_settings=MenuSettings::new("Пауза",&["Продолжить","Главное меню","Настройки","Выход"])
            .head_size([180f32,80f32])
            .buttons_size([180f32,60f32]);

        Self{
            menu:Menu::new(menu_settings,Main_font!()),
        }
    }

    pub unsafe fn start(mut self,window:&mut Window,music:&Music)->Game{
        'page:loop{
            match self.smooth(window){
                Game::Exit=>return Game::Exit,
                Game::Back=>return Game::Back,
                _=>{}
            }
            while let Some(event)=window.next_event(){
                match event{
                    WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                    WindowEvent::Draw=>{ // Рендеринг
                        window.draw(|c,g|{
                            g.clear_colour(background_color);
                            self.menu.draw(c,g);
                        });
                    }

                    WindowEvent::MousePressed(button)=>{
                        match button{
                            MouseButton::Left=>{
                                self.menu.pressed();
                            },
                            _=>{}
                        }
                    }

                    WindowEvent::MouseReleased(button)=>{
                        match button{
                            MouseButton::Left=>{
                                if let Some(button_id)=self.menu.clicked(){
                                    match button_id{
                                        0=>return Game::ContinueGamePlay, // Кнопка продолжить
                                        1=>return Game::MainMenu, // Кнопка главного меню
                                        2=>{ // Кнопка настроек
                                            match SettingsPage::new(window).start(window,music){
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

                    WindowEvent::KeyboardReleased(button)=>{
                        match button{
                            KeyboardButton::F5=>make_screenshot(window,|p,g|{
                                g.clear_colour(background_color);
                                self.menu.draw(p,g);
                            }),
                            KeyboardButton::Escape=>return Game::ContinueGamePlay,
                            _=>{}
                        }
                    }
                    _=>{}
                }
            }
        }
    }

    pub unsafe fn smooth(&mut self,window:&mut Window)->Game{
        window.set_new_smooth(page_smooth);

        let mut background=Rectangle::new(window_rect(),background_color);

        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{ // Рендеринг
                    if 1f32<window.draw_smooth(|alpha,c,g|{
                        background.colour[3]=alpha;
                        background.draw(c,g);
                        self.menu.draw_smooth(alpha,c,g);
                    }){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::F5=>make_screenshot(window,|p,g|{
                            background.draw(p,g);
                            self.menu.draw(p,g);
                        }),
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