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

    let mut head_settings=TextViewSettings::new()
            .text("Настройки".to_string())
            .font_size(40)
            .text_color(White)
            .rect([0f64,0f64,window_width,80f64]);
    let mut head=TextView::new(head_settings.clone(),head_glyphs);

    head_settings=head_settings.font_size(20)
            .text("Количество символов в секунду".to_string())
            .rect([
                window_center[0],100f64,250f64,20f64
            ]);

    let head_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
    let mut head_signs_per_sec_slider=TextView::new(head_settings,head_glyphs);

    let signs_per_sec_slider_sets=SliderSettings::new()
            .position([window_center[0],160f64])
            .length(250f64)
            .min_value(15f64)
            .max_value(120f64)
            .current_value(Settings.signs_per_frame*60f64);

    let slider_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
    let mut signs_per_sec_slider=SliderViewed::new(signs_per_sec_slider_sets,slider_glyphs);

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
        //mouse_cursor.movement(&e); // Движение мыши
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

                head_signs_per_sec_slider.draw_smooth(alpha_channel,&c,g);
                signs_per_sec_slider.draw_smooth(alpha_channel,&c,g);

                for button in &mut common_buttons{
                    button.draw_smooth(alpha_channel,&c,g);
                }
                //mouse_cursor.draw(&c,g);
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

        if let Some(d)=e.mouse_relative_args(){
            mouse_cursor.shift(d);// Движение мыши
            signs_per_sec_slider.grab();
        }
        
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                background.draw(background_rect,&c.draw_state,c.transform,g);
                head.draw(&c,g);
                
                head_signs_per_sec_slider.draw(&c,g);
                signs_per_sec_slider.draw(&c,g);

                for button in &mut common_buttons{
                    button.draw(&c,g);
                }
                mouse_cursor.draw(&c,g);
            });
        }
    
        if Some(Button::Mouse(MouseButton::Left))==e.press_args(){
            mouse_cursor.pressed();
            signs_per_sec_slider.pressed();
        }

        if let Some(button)=e.release_args(){
            match button{
                Button::Mouse(key)=>{
                    match key{
                        MouseButton::Left=>{
                            mouse_cursor.released();

                            Settings.signs_per_frame=signs_per_sec_slider.released()/60f64;

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

