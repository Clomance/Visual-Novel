use crate::*;

#[inline]
pub unsafe fn enter_user_name(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    smooth=1f32/8f32;
    alpha_channel=0f32;

    // Загрузка шрифта
    let texture_settings=TextureSettings::new();
    let mut glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let settings=TextViewSettings::new()
            .text("Введите своё имя".to_string())
            .rect([
                (window_width)/2f64-150f64,
                (window_height)/2f64-150f64,
                300f64,
                70f64,
            ]);
    
    let mut head=TextView::new(settings,glyphs);

    glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();

    let settings=EditTextViewSettings::new()
            .rect([
                (window_width)/2f64-150f64,
                (window_height)/2f64-150f64,
                300f64,
                150f64,
            ])
            .background_color(Light_blue)
            .border_color(Blue);

    let mut name_input=EditTextView::new(settings,glyphs);

    // Глаживание перехода
    'smooth:while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                name_input.draw_smooth(alpha_channel,&c,g);
                head.draw_smooth(alpha_channel,&c,g);
            });
            alpha_channel+=smooth;
            if alpha_channel>=1.0{
                break 'smooth
            }
        }
        // Закрытие про нажатии Escape
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
    }

    // Полная отрисовка
    while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            return Game::Exit
        }
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                name_input.draw(&c,g);
                head.draw(&c,g)
            })
        }
        // Получение вводный данных
        if let Some(text)=e.text_args(){
            name_input.push_text(&text);
        }

        if let Some(button)=e.press_args(){
            match button{
                Button::Keyboard(key)=>{
                    match key{
                        Key::Backspace=>name_input.pop_char(), // Удаление
                        _=>{}
                    }
                }
                _=>{}
            }
        }
        //
        if let Some(button)=e.release_args(){
            match button{
                Button::Keyboard(key)=>{
                    match key{
                        Key::Escape=>{
                            return Game::Back
                        }
                        Key::Return=>{
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