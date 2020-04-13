use crate::*;

#[inline]
pub unsafe fn enter_user_name(window:&mut GameWindow,gl:&mut GlGraphics)->Game{
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
    'smooth:while let Some(event)=window.next_event(){
        match event{
            GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

            GameWindowEvent::Draw(viewport)=>{ // Рендеринг
                gl.draw(viewport,|c,g|{
                    name_input.draw_smooth(alpha_channel,&c,g);
                    head.draw_smooth(alpha_channel,&c,g);
                });

                alpha_channel+=smooth;
                if alpha_channel>=1.0{
                    break 'smooth
                }
            }

            GameWindowEvent::KeyboardReleased(button)=>{
                match button{
                    KeyboardButton::Escape=>return Game::Back,
                    _=>{}
                }
            }
            _=>{}
        }
    }

    // Полная отрисовка
    while let Some(event)=window.next_event(){
        match event{
            GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры
            
            GameWindowEvent::Draw(viewport)=>{ // Рендеринг
                gl.draw(viewport,|c,g|{
                    name_input.draw(&c,g);
                    head.draw(&c,g)
                })
            }

            GameWindowEvent::CharacterInput(character)=>name_input.push_char(character),

            GameWindowEvent::KeyboardPressed(button)=>{
                match button{
                    KeyboardButton::Backspace=>name_input.pop_char(), // Удаление
                    _=>{}
                }
            }

            GameWindowEvent::KeyboardReleased(button)=>{
                match button{
                    KeyboardButton::Escape=>return Game::Back,
                    
                    KeyboardButton::Enter=>{
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
    Game::Exit
}