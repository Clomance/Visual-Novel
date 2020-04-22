use crate::*;

const page_smooth:f32=Settings_page_smooth;

pub struct SettingsPage<'a,'b,'c,'d>{
    head:TextView<'a>,
    head_signs_per_sec:TextView<'d>,
    signs_per_sec:SliderViewed<'b>,
    back_button:Button<'c>,
    background:Rectangle,
    background_rect:[f64;4],
}

impl<'a,'b,'c,'d> SettingsPage<'a,'b,'c,'d>{
    #[inline(always)]
    pub unsafe fn new()->SettingsPage<'a,'b,'c,'d>{
        // Загрузка шрифта
        let texture_settings=TextureSettings::new();

        let head_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let mut head_settings=TextViewSettings::new()
                .text("Настройки".to_string())
                .font_size(40)
                .text_color(White)
                .rect([0f64,0f64,window_width,80f64]);
        let head=TextView::new(head_settings.clone(),head_glyphs);


        head_settings=head_settings.font_size(20)
                .text("Количество символов в секунду".to_string())
                .rect([
                    window_center[0],100f64,250f64,20f64
                ]);

        let head_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let signs_per_sec_slider_sets=SliderSettings::new()
                .position([window_center[0],160f64])
                .length(250f64)
                .min_value(15f64)
                .max_value(120f64)
                .current_value(Settings.signs_per_frame*60f64);
        let slider_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();


        let button_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let button_settings=ButtonSettings::new()
                .rect([
                    40f64,
                    window_height-80f64,
                    120f64,
                    60f64
                ])
                .text("Назад".to_string());

        Self{
            head:head,
            head_signs_per_sec:TextView::new(head_settings,head_glyphs),
            signs_per_sec:SliderViewed::new(signs_per_sec_slider_sets,slider_glyphs),
            back_button:Button::new(button_settings,button_glyphs),
            background:Rectangle::new(Settings_page_color),
            background_rect:[
                0f64,
                0f64,
                window_width,
                window_height
            ],
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{

        if self.smooth(window)==Game::Exit{
            return Game::Exit
        }

        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::MouseMovement(_)=>{
                    self.signs_per_sec.grab();
                }
                
                GameWindowEvent::Draw=>{ //Рендеринг
                    window.draw(|c,g|{
                        self.background.draw(self.background_rect,&c.draw_state,c.transform,g);
                        self.head.draw(&c,g);

                        self.head_signs_per_sec.draw(&c,g);
                        self.signs_per_sec.draw(&c,g);
                        
                        self.back_button.draw(&c,g);
                    });
                }
            
                GameWindowEvent::MousePressed(button)=>{
                    match button{
                        MouseButton::Left=>{
                            self.back_button.pressed();
                            self.signs_per_sec.pressed();
                        },
                        _=>{}
                    }
                }

                GameWindowEvent::MouseReleased(button)=>{
                    match button{
                        MouseButton::Left=>{
                            Settings.signs_per_frame=self.signs_per_sec.released()/60f64;

                            if self.back_button.released(){ // Кнопка "Назад"
                                return Game::Back
                            }
                        }
                        _=>{}
                    }
                }

                GameWindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::Escape=>return Game::Back,
                        _=>{}
                    }
                }

                _=>{} // Остальные события
            }
        }

        Game::Exit
    }

    #[inline(always)]
    pub unsafe fn smooth(&mut self,window:&mut GameWindow)->Game{
        smooth=page_smooth;
        alpha_channel=0f32;

        // Плавное открытие
        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ //Рендеринг
                    window.draw(|c,g|{
                        self.background.color[3]=alpha_channel;
                        self.background.draw(self.background_rect,&c.draw_state,c.transform,g);
                        
                        self.head.draw_smooth(alpha_channel,&c,g);

                        self.head_signs_per_sec.draw_smooth(alpha_channel,&c,g);
                        self.signs_per_sec.draw_smooth(alpha_channel,&c,g);

                        self.back_button.draw_smooth(alpha_channel,&c,g);
                    });

                    alpha_channel+=smooth;
                    if alpha_channel>1.0{
                        return Game::Current
                    }
                }
                _=>{}
            }
        }
        Game::Exit
    }
}