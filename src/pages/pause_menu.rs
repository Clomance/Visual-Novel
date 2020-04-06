use crate::*;

#[inline] // Меню паузы во время игры
pub unsafe fn pause_menu(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    smooth=1f32/8f32;
    alpha_channel=0f32;

    // Создание заднего фона
    let background_size=[300f64,450f64];
    let mut background=Rectangle::new(Pause_menu_background_color);
    let background_rect=[
        (window_width-background_size[0])/2f64,
        (window_height-background_size[1])/2f64,
        background_size[0],
        background_size[1]
    ];


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
                background.color[3]=alpha_channel;
                background.draw(background_rect,&c.draw_state,c.transform,g);

                menu.draw_smooth(alpha_channel,&c,g);
            });
            
            alpha_channel+=smooth;
            if alpha_channel>=1.0{
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
        mouse_cursor_movement(&e); // Движение мыши
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.draw(background_rect,&c.draw_state,c.transform,g);
                menu.draw(&c,g);
            });
        }
        // Нажатие кнопок
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