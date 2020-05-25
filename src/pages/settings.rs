use crate::{
    Main_font,
    make_screenshot,
    Game,
    Settings,
};

use super::default_page_smooth;

use lib::{
    colours::{White,Settings_page_colour},
    Drawable,
    Slider,
    SliderSettings,
    TextViewSettings,
    TextViewStaticLine,
    Button,
    ButtonSettings,
};

use engine::{
    // statics
    window_width,
    window_height,
    window_center,
    // types
    Colour,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
    music::Music,
    // structs
    GameWindow,
    graphics::Rectangle,
};

const page_smooth:f32=default_page_smooth;

const background_color:Colour=Settings_page_colour;

pub struct SettingsPage<'a>{
    head:TextViewStaticLine<'a>,
    signs_per_sec:Slider<'a>,
    volume:Slider<'a>,
    back_button:Button<'a>,
}

impl<'a> SettingsPage<'a>{
    pub unsafe fn new()->SettingsPage<'a>{
        let head_settings=TextViewSettings::new("Настройки",[
                    0f32,
                    0f32,
                    window_width as f32,
                    80f32,
                ])
                .font_size(40f32)
                .text_colour(White);


        let signs_per_sec_slider_sets=SliderSettings::new()
                .head("Количество символов в секунду")
                .position([window_center[0],160f32])
                .length(250f32)
                .min_value(15f32)
                .max_value(120f32)
                .current_value(Settings.signs_per_frame*60f32);


        let volume_settings=SliderSettings::new()
                .head("Громкость")
                .position([window_center[0],250f32])
                .length(250f32)
                .min_value(0f32)
                .max_value(100f32)
                .current_value(Settings.volume as f32*100f32/128f32);


        let volume=Slider::new(volume_settings,Main_font!());

        // Настройки кнопки выхода
        let button_settings=ButtonSettings::new("Назад",[
                    40f32,
                    window_height-80f32,
                    120f32,
                    60f32
                ]);


        Self{
            head:TextViewStaticLine::new(head_settings,Main_font!()),
            signs_per_sec:Slider::new(signs_per_sec_slider_sets,Main_font!()),
            volume:volume,
            back_button:Button::new(button_settings,Main_font!()),
        }
    }

    pub unsafe fn start(&mut self,window:&mut GameWindow,music:&Music)->Game{

        match self.smooth(window){
            Game::Back=>return Game::Back,
            Game::Exit=>return Game::Exit,
            _=>{}
        }

        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::MouseMovementDelta(_)=>{
                    self.signs_per_sec.grab();
                    self.volume.grab();
                }
                
                WindowEvent::Draw=>{ //Рендеринг
                    window.draw(|c,g|{
                        g.clear_colour(background_color);

                        self.head.draw(c,g);

                        self.signs_per_sec.draw(c,g);
                        self.volume.draw(c,g);

                        self.back_button.draw(c,g);
                    });
                }
            
                WindowEvent::MousePressed(button)=>{
                    match button{
                        MouseButton::Left=>{
                            self.back_button.pressed();
                            self.signs_per_sec.pressed();
                            self.volume.pressed();
                        },
                        _=>{}
                    }
                }

                WindowEvent::MouseReleased(button)=>{
                    match button{
                        MouseButton::Left=>{
                            Settings.signs_per_frame=self.signs_per_sec.released()/60f32;

                            Settings.volume=(self.volume.released()/100f32*128f32) as u8;
                            music.set_volume(Settings.volume); // Установка громкости


                            if self.back_button.released(){ // Кнопка "Назад"
                                return Game::Back
                            }
                        }
                        _=>{}
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
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

        let mut background=Rectangle::new([
                0f32,
                0f32,
                window_width,
                window_height
            ],
            Settings_page_colour
        );

        // Плавное открытие
        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{ //Рендеринг
                    if 1f32<window.draw_smooth(|alpha,c,g|{
                        background.colour[3]=alpha;
                        background.draw(c,g);
                        self.head.draw_smooth(alpha,c,g);
                        self.signs_per_sec.draw_smooth(alpha,c,g);
                        self.volume.draw_smooth(alpha,c,g);
                        self.back_button.set_alpha_channel(alpha);
                        self.back_button.draw(c,g);
                    }){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
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