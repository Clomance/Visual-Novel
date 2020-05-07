use crate::*;

const page_smooth:f32=Main_menu_page_smooth; // Сглаживание переходов - 1 к количеству кадров перехода

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

pub struct MainMenu<'a,'b>{
    pub menu:Menu<'a>,
    pub wallpaper:&'b mut Wallpaper
}

impl<'a,'b> MainMenu<'a,'b>{
    #[inline(always)]
    pub unsafe fn new(wallpaper:&'b mut Wallpaper,window:&mut GameWindow)->MainMenu<'a,'b>{
        let texture_settings=TextureSettings::new();

        // Настройка заголовка меню
        let menu_glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",window.display().clone(),texture_settings).unwrap();

        let mut buttons_text=Vec::with_capacity(4);

        if Settings.continue_game{
            buttons_text.push("Продолжить".to_string());
        }
        buttons_text.push("Новая игра".to_string());
        buttons_text.push("Настройки".to_string());
        buttons_text.push("Выход".to_string());

        // Настройка меню
        let menu_settings=MenuSettings::new(Settings.game_name.clone(),&buttons_text)
                .head_size([180f64,80f64])
                .buttons_size([180f64,60f64]);

        Self{
            menu:Menu::new(menu_settings,menu_glyphs), // Создание меню
            wallpaper,
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{
        let radius=mouse_cursor.center_radius();
        self.wallpaper.mouse_shift(radius[0],radius[1]);

        window.set_smooth(default_page_smooth);

        'main:while self.smooth(window)!=Game::Exit{

            // Цикл самого меню
            while let Some(event)=window.next_event(){
                
                match event{
                    GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                    GameWindowEvent::Draw=>{ //Рендеринг
                        window.draw(|c,g|{
                            self.wallpaper.draw(c,g);
                            self.draw(c,g);
                        });
                    }

                    GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                        self.wallpaper.mouse_shift(dx,dy);
                        self.menu.mouse_shift(dx,dy)
                    }

                    GameWindowEvent::MousePressed(button)=>{
                        match button{
                            MouseButton::Left=>{
                                self.menu.pressed();
                            }
                            _=>{}
                        }
                    }

                    GameWindowEvent::MouseReleased(button)=>{
                        match button{
                            MouseButton::Left=>{
                                // Нажата одна из кнопок меню
                                if let Some(button_id)=self.menu.clicked(){
                                    match MenuButtons::button(button_id as u8){
                                        MenuButtons::Continue=>return Game::ContinueGamePlay,

                                        MenuButtons::New=>{ // Кнопка начала нового игрового процесса
                                            // Окно ввода имени захватывает управление над меню
                                            match EnterUserName::new(self,window).start(){
                                                Game::NewGamePlay=>return Game::NewGamePlay,
                                                Game::Exit=>return Game::Exit,
                                                _=>{}
                                            }
                                        }

                                        MenuButtons::Settings=>{
                                            mouse_cursor.save_position(); // Сохранение текущей позиции мышки
                                            match SettingsPage::new(window).start(window){
                                                Game::Exit=>return Game::Exit,
                                                Game::Back=>{
                                                    let (dx,dy)=mouse_cursor.saved_movement();
                                                    self.menu.mouse_shift(dx,dy);
                                                    self.wallpaper.mouse_shift(dx,dy);
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

                    // События окна
                    _=>{}
                }
                // Конец главного цикла (без сглаживания)
            }
            // Конец полного цикла
        }
        Game::Exit
    }

    #[inline(always)]
    pub unsafe fn smooth(&mut self,window:&mut GameWindow)->Game{
        window.set_alpha(0f32);

        while let Some(event)=window.next_event(){
            
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                    self.wallpaper.mouse_shift(dx,dy);
                    self.menu.mouse_shift(dx,dy)
                }

                GameWindowEvent::Draw=>{
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

    #[inline(always)]
    pub fn draw(&mut self,context:&Context,graphics:&mut GameGraphics){
        self.wallpaper.draw(context,graphics);
        self.menu.draw(context,graphics);
    }

    #[inline(always)]
    pub fn draw_smooth(&mut self,alpha:f32,context:&Context,graphics:&mut GameGraphics){
        self.wallpaper.draw_smooth(alpha,context,graphics);
        self.menu.draw_smooth(alpha,context,graphics);
    }
}