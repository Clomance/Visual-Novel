use crate::{
    // consts
    wallpaper,
    game_name,
    // enums
    Wallpaper,
    // structs
    Game,
    Drawable,
    DrawableObject,
};

use super::{
    default_page_smooth,
};

use lib::{Menu, MenuSettings, colours::Light_blue, ButtonSettings, Align, AlignX, AlignY, Button};

use cat_engine::{
    // statics
    mouse_cursor,
    window_height,
    window_width,
    // enums
    WindowEvent,
    KeyboardButton,
    MouseButton,
    glium::DrawParameters,
    audio::Audio,
    // traits
    Window,
    // structs
    DefaultWindow,
    PagedWindow,
    graphics::{
        Graphics,
        DrawType,
        ObjectType
    },
};
use crate::pages::set_main_menu;
use cat_engine::shapes::Rectangle;

pub fn set_settings_menu(game:&mut Game,window:&mut PagedWindow){
    // Clearing buffer
    clear_all_buffers(game, window);
    // Устновка обоев для меню
    game.wallpaper=Wallpaper::Colour([0.3f32,0.6f32,0.6f32,1f32]);


    let mut buttons_text=Vec::with_capacity(4);

    if game.settings.continue_game{
        buttons_text.push("Продолжить");
    }
    buttons_text.push("Новая игра");
    buttons_text.push("Растройки :(");
    buttons_text.push("Выход");

    // Настройка меню
    let menu_settings=MenuSettings::new("asd",buttons_text.into_iter())
            .draw_type(DrawType::Common)
            .header_size([180f32,80f32])
            .button_size([180f32,60f32]);

    let menu=Menu::new(menu_settings,window.graphics2d());

    // Добавление меню
    game.object_map.add_complex_object(0,menu);

    // Добавление кнопки назад
    let window_size = unsafe{[window_width, window_height]};
    let button_size = [175f32, 30f32];
    let rect = [window_size[0]-button_size[0]-20f32, window_size[1]-button_size[1]-20f32, button_size[0], button_size[1]];
    let back_button_settings = ButtonSettings::new("Назад в меню", rect);
    let back_button=Button::new(back_button_settings, window.graphics2d());
    game.object_map.add_complex_object(0,back_button);

    game.prerendering=settings_menu_prerendering;
    game.updates=Game::empty_updates;
    game.click_handler=settings_menu_click_handler;
    game.keyboard_handler=keyboard_handler;

}
pub fn settings_menu_prerendering(_game:&mut Game){}

pub fn settings_menu_click_handler(game:&mut Game,pressed:bool,button:MouseButton,window:&mut PagedWindow){
    let position=unsafe{mouse_cursor.position()};

    if pressed{
        match button{
            MouseButton::Left=>{
                game.audio.play_track(1,1);
                if let Some(button)=game.object_map.pressed(position){
                    window.graphics2d().set_simple_object_colour(button, [0f32,0f32,1f32,1f32]);
               }
            }
            _=>{}
        }
    }
    else{
        match button{
            MouseButton::Left=>{
                let button_amount = if !game.settings.continue_game{
                    4
                } else {
                    5
                };
                for b in 0..button_amount{
                    window.graphics2d().set_simple_object_colour(b, Light_blue)
                }
                if let Some((mut button,clicked))=game.object_map.released(position){
                    if !game.settings.continue_game{
                        button+=1;
                    }
                    match button{
                        0=>{
                            if clicked{
                                println!("continue")
                            }
                        }
                        1=>{
                            if clicked{
                                println!("continue")
                            }
                        }
                        2=>{
                            if clicked{
                                println!("continue")
                            }
                        }
                        3=>{
                            if clicked{
                                window.stop_events();
                                println!("exit")
                            }
                        }
                        4=>{
                            if clicked{
                                clear_all_buffers(game,window);
                                set_main_menu(game,window)
                            }
                        }
                        _=>{

                        }
                    }
                }
            }
            _=>{}
        }
    }
}
pub fn keyboard_handler(game:&mut Game,pressed:bool,button:KeyboardButton,window:&mut PagedWindow){
   if pressed{
       match button{
           KeyboardButton::Escape => {
               window.graphics2d().clear_simple_object_array();
               game.object_map.clear();
               set_main_menu(game,window)
           }
           _ => {}
       }
   }
}
pub fn clear_all_buffers(game:&mut Game,window:&mut PagedWindow){
    window.graphics2d().clear_simple_object_array();
    window.graphics2d().clear_text_object_array();
    game.object_map.clear_layers();
    game.object_map.clear_click_map();
}