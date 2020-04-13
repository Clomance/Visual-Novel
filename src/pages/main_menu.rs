use crate::*;

const page_smooth:f32=1f32/32f32; // Сглаживание переходов - 1 к количеству кадров перехода

enum MenuButtons{
    Continue,
    New,
    Settings,
    Exit
}

impl MenuButtons{
    fn button(mut id:u8)->MenuButtons{
        if unsafe{!Settings._continue}{
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

pub struct MainMenu<'a>{
    menu:Menu<'a>,
}

impl<'a> MainMenu<'a>{
    #[inline(always)]
    pub unsafe fn new()->MainMenu<'a>{
        let texture_settings=TextureSettings::new();
        // Настройка заголовка меню
        let head=Settings.game_name.to_string();
        let menu_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let head_view_settings=TextViewSettings::new()
                .rect([0f64,0f64,100f64,80f64])
                .text(head)
                .font_size(40)
                .text_color(Head_main_menu_color);

        let mut buttons_text=Vec::with_capacity(4);

        if Settings._continue{
            buttons_text.push("Продолжить".to_string());
        }
        buttons_text.push("Новая игра".to_string());
        buttons_text.push("Настройки".to_string());
        buttons_text.push("Выход".to_string());

        // Настройка меню
        let menu_settings=MenuSettings::new()
                .head_text_settings(head_view_settings)
                .buttons_size([180f64,60f64])
                .buttons_text(buttons_text);

        Self{
            menu:Menu::new(menu_settings,menu_glyphs), // Создание меню
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self,wallpaper:&mut Wallpaper,window:&mut GameWindow,gl:&mut GlGraphics)->Game{
        smooth=page_smooth;

        //                    //
        // Цикл главного меню //
        //                    //
        'main_menu:while self.smooth(wallpaper,window,gl)!=Game::Exit{

            // Цикл самого меню
            while let Some(event)=window.next_event(){

                match event{
                    GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                    GameWindowEvent::MouseMovement((x,y))=>{
                        mouse_cursor.set_position([x,y]);
                        wallpaper.move_with_cursor([x,y]);
                    }

                    GameWindowEvent::Draw(viewport)=>{ //Рендеринг
                        gl.draw(viewport,|c,g|{
                            wallpaper.draw(&c,g);
                            self.menu.draw(&c,g);
                            mouse_cursor.draw(&c,g);
                        });
                    }

                    GameWindowEvent::MousePressed(button)=>{
                        match button{
                            MouseButton::Left=>mouse_cursor.pressed(),
                            _=>{}
                        }
                    }

                    GameWindowEvent::MouseReleased(button)=>{
                        match button{
                            MouseButton::Left=>{
                                mouse_cursor.released();
                                // Нажата одна из кнопок меню
                                if let Some(button_id)=self.menu.clicked(){
                                    match MenuButtons::button(button_id as u8){
                                        MenuButtons::Continue=>return Game::ContinueGamePlay,

                                        MenuButtons::New=>{ // Кнопка начала нового игрового процесса
                                            match enter_user_name(window,gl){
                                                Game::NewGamePlay=>return Game::NewGamePlay,
                                                Game::Exit=>return Game::Exit,
                                                _=>{}
                                            }
                                            wallpaper.move_with_cursor(mouse_cursor.position())
                                        }
                                        MenuButtons::Settings=>{
                                            match settings_page(window,gl){
                                                Game::Exit=>return Game::Exit,
                                                Game::Back=>{
                                                    wallpaper.move_with_cursor(mouse_cursor.position());
                                                    continue 'main_menu
                                                },
                                                _=>{}
                                            }
                                            wallpaper.move_with_cursor(mouse_cursor.position())
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
    pub unsafe fn smooth(&mut self,wallpaper:&mut Wallpaper,window:&mut GameWindow,gl:&mut GlGraphics)->Game{
        alpha_channel=0f32;

        while let Some(event)=window.next_event(){
            
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw(viewport)=>{
                    gl.draw(viewport,|c,g|{
                        wallpaper.draw_smooth(alpha_channel,&c,g);
                        self.menu.draw_smooth(alpha_channel,&c,g);
                    });

                    alpha_channel+=smooth;
                    if alpha_channel>=1.0{
                        return Game::MainMenu
                    }
                }
                _=>{}
            }
        }

        Game::Exit
    }
}