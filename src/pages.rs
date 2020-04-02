use crate::*;

#[inline] // Главное меню
pub fn main_menu(wallpaper:&mut Wallpaper,events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    let texture_settings=TextureSettings::new();
    
    let smooth=1f32/32f32; // Сглаживание переходов - 1 к количеству кадров перехода
    let mut alpha;

    // Настройка заголовка меню
    let head=Game_name.to_string();
    let menu_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
    let head_view_settings=TextViewSettings::new()
            .rect([0f64,0f64,100f64,80f64])
            .text(head)
            .font_size(40)
            .text_color(Head_main_menu_color);

    let mut buttons_text=Vec::with_capacity(4);
    unsafe{
        if Settings._continue{
            buttons_text.push("Продолжить".to_string());
        }
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
        // Плавный переход
        alpha=0f32;

        'smooth:while let Some(e)=events.next(window){
            // Закрытие игры
            if let Some(_close)=e.close_args(){
                return Game::Exit
            }
            // Рендеринг
            if let Some(r)=e.render_args(){
                gl.draw(r.viewport(),|c,g|{
                    wallpaper.set_alpha_channel(alpha);
                    wallpaper.draw(&c.draw_state,c.transform,g);
                    
                    menu.set_alpha_channel(alpha);
                    menu.draw(&c.draw_state,c.transform,g);

                    alpha+=smooth;
                });
                if alpha>=1.0{
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
            // Движение мыши
            if let Some(mouse)=e.mouse_cursor_args(){
                unsafe{
                    mouse_position=mouse;
                }
            }
            // Рендеринг
            if let Some(r)=e.render_args(){
                gl.draw(r.viewport(),|c,g|{
                    wallpaper.draw(&c.draw_state,c.transform,g);
                    menu.draw(&c.draw_state,c.transform,g);
                });
            }
            // 
            if let Some(button)=e.release_args(){
                match button{
                    Button::Mouse(key)=>{
                        match key{
                            MouseButton::Left=>{
                                if let Some(button_id)=menu.clicked(){
                                    unsafe{
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
                            }
                            _=>{}
                        }
                    }
                    Button::Keyboard(key)=>{
                        println!("{:?}",key)
                    }
                    _=>{}
                }
            }
        }
    }
    // Конец меню
}

#[inline]
pub fn enter_user_name(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    let smooth=1f32/32f32;
    let mut alpha=0f32;

    // Загрузка шрифта
    let texture_settings=TextureSettings::new();
    let mut glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let settings=TextViewSettings::new()
            .text("Введите своё имя".to_string())
            .rect(unsafe{[
                (window_width)/2f64-150f64,
                (window_height)/2f64-150f64,
                300f64,
                70f64,
            ]});
    
    let mut head=TextView::new(settings,glyphs);

    glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let settings=EditTextViewSettings::new()
            .rect(unsafe{[
                (window_width)/2f64-150f64,
                (window_height)/2f64-150f64,
                300f64,
                150f64,
            ]})
            .background_color(Cyan);

    let mut name_input=EditTextView::new(settings,glyphs);

    while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                name_input.draw(&c.draw_state,c.transform,g);
                head.draw(&c.draw_state,c.transform,g)
            })
        }
        // Получение вводный данных
        if let Some(text)=e.text_args(){
            name_input.push_text(&text);
        }
        //
        if let Some(button)=e.release_args(){
            match button{
                Button::Keyboard(key)=>{
                    match key{
                        Key::Backspace=>name_input.pop_text(), // Удаление
                        Key::Escape=>{
                            return Game::Back
                        }
                        Key::Return=>unsafe{
                            let name=name_input.get_text();
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
    }
    return Game::Exit
}

#[inline] // Страница настроек
pub fn settings_page(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    let smooth=1f32/32f32;
    let mut alpha=0f32;

    let width=unsafe{window_width}; // Ширина окна

    // Создание заднего фона
    let mut background=Rectangle::new(Settings_page_color);
    let background_rect=unsafe{[
        0f64,
        0f64,
        width,
        window_height
    ]};

    // Загрузка шрифта
    let texture_settings=TextureSettings::new();
    let head_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let button_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let head_settings=TextViewSettings::new()
            .text("Настройки".to_string())
            .font_size(40)
            .text_color(White)
            .rect(unsafe{[0f64,0f64,window_width,80f64]});
    let mut head=TextView::new(head_settings,head_glyphs);

    let button_settings=ButtonSettings::new()
            .rect(unsafe{[
                40f64,
                window_height-80f64,
                120f64,
                60f64
            ]})
            .text("Назад".to_string());

    let mut common_buttons=[
        user_interface::Button::new(button_settings,button_glyphs)
    ];

    // Плавное открытие
    'smooth:while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Нажатие клавишь
        if let Some(button)=e.release_args(){
            match button{
                Button::Keyboard(key)=>{
                    match key{
                        Key::Escape=>{
                            return Game::Back
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.color[3]=alpha;
                background.draw(background_rect,&c.draw_state,c.transform,g);
                
                head.set_alpha_channel(alpha);
                head.draw(&c.draw_state,c.transform,g);

                for button in &mut common_buttons{
                    button.set_alpha_channel(alpha);
                    button.draw(&c.draw_state,c.transform,g);
                }

                alpha+=smooth;
                
            });
            if alpha>=1.0{
                break 'smooth
            }
        }
    }   

    // Рабочий вид
    while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Движение мыши
        if let Some(mouse)=e.mouse_cursor_args(){
            unsafe{
                mouse_position=mouse;
            }
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.draw(background_rect,&c.draw_state,c.transform,g);
                head.draw(&c.draw_state,c.transform,g);

                for button in &mut common_buttons{
                    button.draw(&c.draw_state,c.transform,g);
                }
            });
        }
    
        if let Some(button)=e.release_args(){
            match button{
                Button::Mouse(key)=>{
                    match key{
                        MouseButton::Left=>{
                            if common_buttons[0].clicked(){ // Кнопка "Назад"
                                return Game::Back
                            }
                        }
                        _=>{}
                    }
                }
                Button::Keyboard(key)=>{
                    match key{
                        Key::Escape=>{
                            return Game::Back
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        }
    }
    return Game::Exit
}

#[inline] // Меню паузы во время игры
pub fn pause_menu(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    let smooth=1f32/32f32;
    let mut alpha=0f32;

    // Создание заднего фона
    let background_size=[300f64,450f64];
    let mut background=Rectangle::new(Pause_menu_background_color);
    let background_rect=unsafe{[
        (window_width-background_size[0])/2f64,
        (window_height-background_size[1])/2f64,
        background_size[0],
        background_size[1]
    ]};


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
                "Выход".to_string(),
            ]);

    let mut menu=Menu::new(menu_settings,menu_glyphs);

    // Плавная отрисовка
    'smooth:while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.color[3]=alpha;
                background.draw(background_rect,&c.draw_state,c.transform,g);

                menu.set_alpha_channel(alpha);
                menu.draw(&c.draw_state,c.transform,g);

                alpha+=smooth;
            });
            if alpha>=1.0{
                break 'smooth
            }
        }
    }

    // Цикл обработки
    while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Движение мыши
        if let Some(mouse)=e.mouse_cursor_args(){
            unsafe{
                mouse_position=mouse;
            }
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.draw(background_rect,&c.draw_state,c.transform,g);
                menu.draw(&c.draw_state,c.transform,g);
            });
        }

        if let Some(button)=e.release_args(){
            match button{
                Button::Keyboard(key)=>{
                    match key{
                        Key::Escape=>{
                            return Game::ContinueGamePlay
                        }
                        _=>{}
                    }
                }
                Button::Mouse(key)=>{
                    match key{
                        MouseButton::Left=>{
                            if let Some(button_id)=menu.clicked(){
                                match button_id{
                                    0=>return Game::ContinueGamePlay, // Кнопка продолжить
                                    1=>return Game::MainMenu,
                                    2=>return Game::Exit, // Кнопка выхода
                                    _=>{}
                                }
                                
                            }
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        }
    }
    return Game::Exit
}