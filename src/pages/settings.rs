use crate::*;

#[inline] // Страница настроек
pub unsafe fn settings_page(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    smooth=1f32/32f32;
    alpha_channel=0f32;

    // Создание заднего фона
    let mut background=Rectangle::new(Settings_page_color);
    let background_rect=[
        0f64,
        0f64,
        window_width,
        window_height
    ];

    // Загрузка шрифта
    let texture_settings=TextureSettings::new();
    let head_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let button_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let head_settings=TextViewSettings::new()
            .text("Настройки".to_string())
            .font_size(40)
            .text_color(White)
            .rect([0f64,0f64,window_width,80f64]);
    let mut head=TextView::new(head_settings,head_glyphs);

    let button_settings=ButtonSettings::new()
            .rect([
                40f64,
                window_height-80f64,
                120f64,
                60f64
            ])
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
                background.color[3]=alpha_channel;
                background.draw(background_rect,&c.draw_state,c.transform,g);
                
                head.draw_smooth(alpha_channel,&c,g);

                for button in &mut common_buttons{
                    button.draw_smooth(alpha_channel,&c,g);
                }
            });
            alpha_channel+=smooth;
            if alpha_channel>=1.0{
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
        mouse_cursor_movement(&e); // Движение мыши
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.draw(background_rect,&c.draw_state,c.transform,g);
                head.draw(&c,g);

                for button in &mut common_buttons{
                    button.draw(&c,g);
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

