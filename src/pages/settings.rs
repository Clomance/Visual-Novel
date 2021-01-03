use crate::{
    // consts
    game_name,
    mouse_cursor_icon_index,
    wallpaper_index,
    wallpaper_movement_scale,
    swipe_updates,
    swipe_screen_index,
    // statics
    game_settings,
    // enums
    Game,
    // functions
    get_swipe_texture,
    draw_on_texture,
    make_screenshot,
};

use super::button_pressed;

use lib::{
    colours::{White,Gray,Dark_gray,Light_blue},
    user_interface::{
        Button,
        ButtonSettings,
    },
};

use cat_engine::{
    // types
    Colour,
    // statics
    mouse_cursor,
    window_center,
    window_height,
    window_width,
    // enums
    KeyboardButton,
    // structs
    Window,
    WindowEvent,
    MouseButton,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,ImageBase,Texture},
    image::RgbaImage,
    audio::AudioWrapper,
};

pub const page_colour:Colour=Dark_gray;

pub struct Settings{
    button_pressed:Option<usize>,
    reset_game_progress:Button,
    escape:Button,
}

impl Settings{
    pub fn new(_window:&Window,graphics:&mut Graphics2D)->Settings{
        let escape_rect=unsafe{[
            10f32,
            window_height-70f32,
            160f32,
            60f32,
        ]};
        let escape_settings=ButtonSettings::new("Назад",escape_rect);
        let escape=Button::new(escape_settings,graphics);

        let reset_game_progress_rect=unsafe{[
            window_center[0]-125f32,
            70f32,
            250f32,
            60f32,
        ]};
        let reset_game_progress_settings=ButtonSettings::new("Сбросить прогресс игры",reset_game_progress_rect);
        let reset_game_progress=Button::new(reset_game_progress_settings,graphics);

        Self{
            button_pressed:None,
            reset_game_progress,
            escape,
        }
    }

    pub fn open(&mut self,window:&mut Window,graphics:&mut Graphics2D)->Game{
        let mut result=Game::Next;

        let mut frames=0u8;

        let mut shift=0f32;

        let dshift=unsafe{window_width/swipe_updates as f32};

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>result=Game::Exit,
                WindowEvent::Update=>{
                    frames+=1;
                    if frames==swipe_updates{
                        window.stop_events();
                    }
                    else{
                        shift-=dshift;
                    }
                }

                WindowEvent::RedrawRequested=>{
                    let next_page_shift=unsafe{window_width+shift};

                    window.draw(&graphics,|graphics|{
                        graphics.clear_colour(page_colour);

                        graphics.draw_shift_textured_object(swipe_screen_index,[shift,0f32]);

                        self.reset_game_progress.draw_shift([next_page_shift,0f32],graphics);
                        self.escape.draw_shift([next_page_shift,0f32],graphics);
                    });
                }

                _=>{}
            }
        });

        result
    }

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D,audio:&AudioWrapper)->Game{
        let mut result=Game::Next;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>result=Game::Exit,

                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                    window.draw(graphics,|graphics|{
                        // Фон
                        graphics.clear_colour(page_colour);

                        self.reset_game_progress.draw(graphics);
                        self.escape.draw(graphics);

                        // Отрисовка курсора
                        graphics.draw_shift_textured_object(mouse_cursor_icon_index,[dx,dy]).unwrap();
                    }).unwrap();
                }

                WindowEvent::MousePressed(button)=>{
                    if let MouseButton::Left=button{
                        let [x,y]=unsafe{mouse_cursor.position()};

                        self.button_pressed=None;

                        if self.reset_game_progress.pressed(x,y){
                            audio.play_track("button_pressed",1u32);
                            *graphics.get_simple_object_colour(self.reset_game_progress.background_index())=button_pressed;
                            self.button_pressed=Some(self.reset_game_progress.background_index());
                        }
                        else if self.escape.pressed(x,y){
                            audio.play_track("button_pressed",1u32);
                            *graphics.get_simple_object_colour(self.escape.background_index())=button_pressed;
                            self.button_pressed=Some(self.escape.background_index());
                        }
                    }
                }

                WindowEvent::MouseReleased(button)=>{
                    if let MouseButton::Left=button{
                        if let Some(button)=self.button_pressed{
                            let [x,y]=unsafe{mouse_cursor.position()};
                            *graphics.get_simple_object_colour(button)=Light_blue;

                            if button==self.escape.background_index(){
                                if self.escape.released(x,y){
                                    // escape action
                                    window.stop_events();
                                }
                            }
                            else{
                                if self.reset_game_progress.released(x,y){
                                    // reset action
                                    unsafe{
                                        game_settings.continue_game=false;
                                    }
                                }
                            }
                        }
                    }
                }

                WindowEvent::KeyboardPressed(button)=>match button{
                    KeyboardButton::Escape=>{
                        window.stop_events();
                    }

                    KeyboardButton::F5=>make_screenshot(window,audio),

                    _=>{}
                }

                _=>{

                }
            }
        });

        self.render_to_texture(window,graphics);

        // Удаление всех простых объектов
        graphics.remove_last_simple_object();
        graphics.remove_last_simple_object();
        // Удаление всех текстовых объектов
        graphics.remove_last_text_object();
        graphics.remove_last_text_object();
        result
    }

    fn render_to_texture(&self,window:&Window,graphics:&mut Graphics2D){
        let swipe_screen_texture=get_swipe_texture(graphics);

        draw_on_texture(&swipe_screen_texture,window,graphics,|graphics|{
            graphics.clear_colour(page_colour);

            self.reset_game_progress.draw(graphics);
            self.escape.draw(graphics);
        });
    }
}