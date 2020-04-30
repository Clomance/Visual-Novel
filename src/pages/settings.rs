use crate::*;

const page_smooth:f32=Settings_page_smooth;

const background_color:Color=Settings_page_color;

pub struct SettingsPage<'a,'b,'d>{
    head:TextViewStaticLineDependent,
    signs_per_sec:Slider<'b>,
    volume:Slider<'d>,
    back_button:ButtonDependent,
    glyphs:GlyphCache<'a>,
}

impl<'a,'b,'d> SettingsPage<'a,'b,'d>{
    #[inline(always)]
    pub unsafe fn new()->SettingsPage<'a,'b,'d>{

        let texture_settings=TextureSettings::new();

        let mut glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let head_settings=TextViewSettings::new("Настройки",[
                    0f64,
                    0f64,
                    window_width,
                    80f64,
                ])
                .font_size(40)
                .text_color(White);


        let signs_per_sec_slider_sets=SliderSettings::new()
                .head("Количество символов в секунду")
                .position([window_center[0],160f64])
                .length(250f64)
                .min_value(15f64)
                .max_value(120f64)
                .current_value(Settings.signs_per_frame*60f64);
        let slider_glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",(),texture_settings).unwrap();


        let volume_settings=SliderSettings::new()
                .head("Громкость")
                .position([window_center[0],250f64])
                .length(250f64)
                .min_value(0f64)
                .max_value(100f64)
                .current_value(Settings.volume*100f64);
        let volume_glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let volume=Slider::new(volume_settings,volume_glyphs);

        // Настройки кнопки выхода
        let button_settings=ButtonSettings::new("Назад",[
                    40f64,
                    window_height-80f64,
                    120f64,
                    60f64
                ]);


        Self{
            head:TextViewStaticLineDependent::new(head_settings,&mut glyphs),
            signs_per_sec:Slider::new(signs_per_sec_slider_sets,slider_glyphs),
            volume:volume,
            back_button:ButtonDependent::new(button_settings,&mut glyphs),
            glyphs:glyphs,
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

                GameWindowEvent::MouseMovementDelta(_)=>{
                    self.signs_per_sec.grab();
                    self.volume.grab();
                }
                
                GameWindowEvent::Draw=>{ //Рендеринг
                    window.draw(|c,g|{
                        g.clear_color(background_color);

                        self.head.draw(c,g,&mut self.glyphs);

                        self.signs_per_sec.draw(c,g);
                        self.volume.draw(c,g);

                        self.back_button.draw(c,g,&mut self.glyphs);
                    });
                }
            
                GameWindowEvent::MousePressed(button)=>{
                    match button{
                        MouseButton::Left=>{
                            self.back_button.pressed();
                            self.signs_per_sec.pressed();
                            self.volume.pressed();
                        },
                        _=>{}
                    }
                }

                GameWindowEvent::MouseReleased(button)=>{
                    match button{
                        MouseButton::Left=>{
                            Settings.signs_per_frame=self.signs_per_sec.released()/60f64;

                            Settings.volume=self.volume.released()/100f64;
                            music::set_volume(Settings.volume); // Установка громкости


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
        window.set_new_smooth(page_smooth);

        let mut background=Background::new(Settings_page_color,[
            0f64,
            0f64,
            window_width,
            window_height
        ]);

        // Плавное открытие
        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ //Рендеринг
                    if !window.draw_smooth(|alpha,c,g|{
                        background.draw_smooth(alpha,c,g);

                        self.head.draw_smooth(alpha,c,g,&mut self.glyphs);

                        self.signs_per_sec.draw_smooth(alpha,c,g);
                        self.volume.draw_smooth(alpha,c,g);

                        self.back_button.set_alpha_channel(alpha);
                        self.back_button.draw(c,g,&mut self.glyphs);
                    }){
                        break
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }
}