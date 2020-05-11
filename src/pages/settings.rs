use crate::*;

use lib::game_engine::text::Glyphs;

const page_smooth:f32=Settings_page_smooth;

const background_color:Color=Settings_page_color;

pub struct SettingsPage<'a,'b,'d>{
    head:TextViewStaticLineDependent,
    signs_per_sec:Slider<'b>,
    volume:Slider<'d>,
    back_button:ButtonDependent,
    glyphs:Glyphs<'a>,
}

impl<'a,'b,'d> SettingsPage<'a,'b,'d>{
    pub unsafe fn new()->SettingsPage<'a,'b,'d>{
        let mut glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");
        let head_settings=TextViewSettings::new("Настройки",[
                    0f32,
                    0f32,
                    window_width as f32,
                    80f32,
                ])
                .font_size(40f32)
                .text_color(White);


        let signs_per_sec_slider_sets=SliderSettings::new()
                .head("Количество символов в секунду")
                .position([window_center[0],160f32])
                .length(250f32)
                .min_value(15f32)
                .max_value(120f32)
                .current_value(Settings.signs_per_frame*60f32);
        let slider_glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");


        let volume_settings=SliderSettings::new()
                .head("Громкость")
                .position([window_center[0],250f32])
                .length(250f32)
                .min_value(0f32)
                .max_value(100f32)
                .current_value(Settings.volume*100f32);
        let volume_glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");
        let volume=Slider::new(volume_settings,volume_glyphs);

        // Настройки кнопки выхода
        let button_settings=ButtonSettings::new("Назад",[
                    40f32,
                    window_height-80f32,
                    120f32,
                    60f32
                ]);


        Self{
            head:TextViewStaticLineDependent::new(head_settings,&mut glyphs),
            signs_per_sec:Slider::new(signs_per_sec_slider_sets,slider_glyphs),
            volume:volume,
            back_button:ButtonDependent::new(button_settings,&mut glyphs),
            glyphs:glyphs,
        }
    }

    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{

        match self.smooth(window){
            Game::Back=>return Game::Back,
            Game::Exit=>return Game::Exit,
            _=>{}
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
                            Settings.signs_per_frame=self.signs_per_sec.released()/60f32;

                            Settings.volume=self.volume.released()/100f32;
                            music::set_volume(Settings.volume as f64); // Установка громкости


                            if self.back_button.released(){ // Кнопка "Назад"
                                return Game::Back
                            }
                        }
                        _=>{}
                    }
                }

                GameWindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::F5=>make_screenshot(window),
                        KeyboardButton::Escape=>return Game::Back,
                        _=>{}
                    }
                }

                _=>{} // Остальные события
            }
        }

        Game::Exit
    }

    pub unsafe fn smooth(&mut self,window:&mut GameWindow)->Game{
        window.set_new_smooth(page_smooth);

        let mut background=Background::new(Settings_page_color,[
            0f64,
            0f64,
            window_width as f64,
            window_height as f64
        ]);

        // Плавное открытие
        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ //Рендеринг
                    if 1f32<window.draw_smooth(|alpha,c,g|{
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

                GameWindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::F5=>make_screenshot(window),
                        KeyboardButton::Escape=>return Game::Back,
                        _=>{}
                    }
                }

                _=>{}
            }
        }
        Game::Current
    }
}