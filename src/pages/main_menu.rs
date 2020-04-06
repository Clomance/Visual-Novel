use crate::*;

#[inline] // Главное меню
pub unsafe fn main_menu(wallpaper:&mut Wallpaper,events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    let texture_settings=TextureSettings::new();
    
    smooth=1f32/32f32; // Сглаживание переходов - 1 к количеству кадров перехода

    // Настройка заголовка меню
    let head=Game_name.to_string();
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

    // Создание меню
    let mut menu=Menu::new(menu_settings,menu_glyphs);

    //                    //
    // Цикл главного меню //
    //                    //
    'main_menu:loop{

        alpha_channel=0f32;

        'smooth:while let Some(e)=events.next(window){
        // Закрытие игры
            if let Some(_close)=e.close_args(){
                return Game::Exit
            }
            // Рендеринг
            if let Some(r)=e.render_args(){
                gl.draw(r.viewport(),|c,g|{
                    wallpaper.draw_smooth(alpha_channel,&c,g);
                    menu.draw_smooth(alpha_channel,&c,g);
                });
                alpha_channel+=smooth;
                if alpha_channel>=1.0{
                    break 'smooth
                }
            }
        }

        // Цикл самого меню
        while let Some(e)=events.next(window){
            // Закрытие игры
            if let Some(_close)=e.close_args(){
                return Game::Exit
            }
            mouse_cursor_movement(&e); // Движение мыши
            //Рендеринг
            if let Some(r)=e.render_args(){
                gl.draw(r.viewport(),|c,g|{
                    wallpaper.draw(&c,g);
                    menu.draw(&c,g);
                });
            }
            
            if let Some(button)=e.release_args(){
                match button{
                    Button::Mouse(key)=>{
                        match key{
                            MouseButton::Left=>{
                                if let Some(button_id)=menu.clicked(){
                                    if Settings._continue{
                                        match button_id{
                                            0=>return Game::ContinueGamePlay,
                                            1=>{ // Кнопка начала нового игрового процесса
                                                match enter_user_name(events,window,gl){
                                                    Game::NewGamePlay=>return Game::NewGamePlay,
                                                    Game::Exit=>return Game::Exit,
                                                    _=>{}
                                                }
                                            }
                                            2=>{
                                                match settings_page(events,window,gl){
                                                    Game::Exit=>return Game::Exit,
                                                    Game::Back=>continue 'main_menu,
                                                    _=>{}
                                                }
                                            }
                                            3=>return Game::Exit, // Кнопка закрытия игры
                                            _=>{}
                                        }
                                    }
                                    else{
                                        match button_id{
                                            0=>{ // Кнопка начала нового игрового процесса
                                                match enter_user_name(events,window,gl){
                                                    Game::NewGamePlay=>return Game::NewGamePlay,
                                                    Game::Exit=>return Game::Exit,
                                                    _=>{}
                                                }
                                            }
                                            1=>{
                                                match settings_page(events,window,gl){
                                                    Game::Exit=>return Game::Exit,
                                                    Game::Back=>continue 'main_menu,
                                                    _=>{}
                                                }
                                            }
                                            2=>return Game::Exit, // Кнопка закрытия игры
                                            _=>{}
                                        }
                                    }
                                }
                            }
                            _=>{}
                        }
                    }
                    _=>{}
                }
            }
            // Конец цикла
        }
        // Конец полного цикла
    }
    // Конец меню
}