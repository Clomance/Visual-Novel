use crate::{
    game_name,
    Main_font,
    make_screenshot,
    Game,
    Settings,
};

use super::{
    default_page_smooth,
    EnterUserName,
    SettingsPage,
};

use lib::{
    Wallpaper,
    Menu,
    MenuSettings,
    Drawable,
};

use cat_engine::{
    // statics
    mouse_cursor,
    // structs
    Window,
    graphics::Graphics,
    // enums
    WindowEvent,
    KeyboardButton,
    MouseButton,
    glium::DrawParameters,
    audio::Audio,
};

const page_smooth:f32=default_page_smooth; // Сглаживание переходов - 1 к количеству кадров перехода

const menu_movement_scale:f32=10f32; // Обратный коэфициент сдвига меню при движении мышью

// Кнопки меню
// Для упрощение определения нажатой кнопки,
// так как меню может иметь ещё и кпопку "Продолжить"
enum MenuButtons{
    Continue,
    New,
    Settings,
    Exit
}

impl MenuButtons{
    #[inline(always)]
    fn button(mut id:u8)->MenuButtons{
        if unsafe{!Settings.continue_game}{
            id+=1;
        }
        match id{
            0=>MenuButtons::Continue,
            1=>MenuButtons::New,
            2=>MenuButtons::Settings,
            _=>MenuButtons::Exit
        }
    }
}

// Главное меню
pub struct MainMenu<'a,'wallpaper>{
    pub menu:Menu<'a>,
    pub wallpaper:&'wallpaper mut Wallpaper
}

impl<'a,'wallpaper> MainMenu<'a,'wallpaper>{
    pub fn new(wallpaper:&'wallpaper mut Wallpaper)->MainMenu<'a,'wallpaper>{

        // Настройка заголовка меню
        let mut buttons_text=Vec::with_capacity(4);

        if unsafe{Settings.continue_game}{
            buttons_text.push("Продолжить".to_string());
        }
        buttons_text.push("Новая игра".to_string());
        buttons_text.push("Настройки".to_string());
        buttons_text.push("Выход".to_string());

        // Настройка меню
        let menu_settings=MenuSettings::new(game_name,&buttons_text)
                .head_size([180f32,80f32])
                .buttons_size([180f32,60f32]);

        Self{
            menu:Menu::new(menu_settings,Main_font!()), // Создание меню
            wallpaper,
        }
    }

    pub fn start(mut self,window:&mut Window,music:&Audio)->Game{
        self.mouse_shift(unsafe{
            mouse_cursor.center_radius()
        });

        window.set_smooth(page_smooth);

        'main:while self.smooth(window)!=Game::Exit{

            // Цикл самого меню
            while let Some(event)=window.next_event(){

                match event{
                    WindowEvent::Exit=>return Game::Exit, // Закрытие игры
                    //Рендеринг
                    WindowEvent::Draw=>window.draw(|c,g|{
                        self.draw(c,g);
                    }),
                    
                    // Движение мышки
                    WindowEvent::MouseMovementDelta(shift)=>self.mouse_shift(shift),
                    // Кнопка мышки нажата
                    WindowEvent::MousePressed(button)=>match button{
                        MouseButton::Left=>{
                            self.menu.pressed();
                        }
                        _=>{}
                    }

                    // Кнопка мышки отпущена
                    WindowEvent::MouseReleased(button)=>{
                        match button{
                            MouseButton::Left=>{
                                // Нажата одна из кнопок меню
                                if let Some(button_id)=self.menu.clicked(){
                                    match MenuButtons::button(button_id as u8){
                                        MenuButtons::Continue=>return Game::ContinueGamePlay,

                                        // Кнопка начала нового игрового процесса
                                        // Окно ввода имени захватывает управление над меню
                                        MenuButtons::New=>match EnterUserName::new(&mut self).start(window){
                                            Game::NewGamePlay=>return Game::NewGamePlay,
                                            Game::Exit=>return Game::Exit,
                                            _=>{}
                                        }

                                        MenuButtons::Settings=>unsafe{
                                            mouse_cursor.save_position();
                                            match SettingsPage::new(window).start(window,music){
                                                Game::Exit=>return Game::Exit,
                                                Game::Back=>{
                                                    self.mouse_shift(mouse_cursor.saved_shift());
                                                    continue 'main
                                                }
                                                _=>{}
                                            }
                                        }

                                        MenuButtons::Exit=>return Game::Exit, // Кнопка закрытия игры
                                    }
                                    
                                }
                            }
                            // Отпущенные кнопки мыши
                            _=>{}
                        }
                    }

                    WindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(window,|d,g|{
                                self.wallpaper.draw(d,g);
                                self.draw(d,g);
                            });
                        }
                    }

                    // События окна
                    _=>{}
                }
                // Конец главного цикла (без сглаживания)
            }
            // Конец полного цикла
        }
        Game::Exit
    }

    pub fn smooth(&mut self,window:&mut Window)->Game{
        window.set_new_smooth(page_smooth);

        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::MouseMovementDelta(shift)=>self.mouse_shift(shift),

                WindowEvent::Draw=>{
                    if 1f32<window.draw_smooth(|alpha,c,g|{
                        self.draw_smooth(alpha,c,g);
                    }){
                        break
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.wallpaper.draw_shift(draw_parameters,graphics);
        self.menu.draw(draw_parameters,graphics);
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.wallpaper.draw_shift_smooth(alpha,draw_parameters,graphics);
        self.menu.draw_smooth(alpha,draw_parameters,graphics);
    }

    pub fn mouse_shift(&mut self,[dx,dy]:[f32;2]){
        let [dx,dy]=[dx/menu_movement_scale,dy/menu_movement_scale];
        self.menu.shift([dx,dy]);
        self.wallpaper.mouse_shift(unsafe{mouse_cursor.center_radius()});
    }
}