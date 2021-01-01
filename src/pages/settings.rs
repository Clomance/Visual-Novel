use crate::{
    // consts
    game_name,
    mouse_cursor_icon_index,
    wallpaper_index,
    wallpaper_movement_scale,
    // statics
    game_settings,
    // enums
    Game,
};

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
};

pub const page_colour:Colour=Dark_gray;

const button_pressed:Colour=[
    Light_blue[0]-0.05,
    Light_blue[1]-0.05,
    Light_blue[2]-0.05,
    Light_blue[3],
];

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

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D)->Game{
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
                            self.button_pressed=Some(self.reset_game_progress.background_index());
                        }

                        if self.escape.pressed(x,y){
                            self.button_pressed=Some(self.escape.background_index());
                        }
                    }
                }

                WindowEvent::MouseReleased(button)=>{
                    if let MouseButton::Left=button{
                        let [x,y]=unsafe{mouse_cursor.position()};

                        self.reset_game_progress.released(x,y);
                        self.escape.released(x,y);
                    }
                }

                WindowEvent::KeyboardPressed(button)=>match button{
                    KeyboardButton::Escape=>{
                        window.stop_events();
                    }

                    KeyboardButton::F5=>unsafe{
                        let path=format!("./screenshots/screenshot{}.png",game_settings.screenshot);
                        game_settings.screenshot+=1;
                        window.save_screenshot(path);
                    }
                    _=>{}
                }

                _=>{

                }
            }
        });

        // Удаление всех простых объектов
        graphics.remove_last_simple_object();
        graphics.remove_last_simple_object();
        // Удаление всех текстовых объектов
        graphics.remove_last_text_object();
        graphics.remove_last_text_object();
        result
    }
}