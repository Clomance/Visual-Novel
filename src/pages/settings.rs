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

use lib::{
    Menu,
    MenuSettings,
};

use cat_engine::{
    // statics
    mouse_cursor,
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

pub fn set_settings_menu(game:&mut Game,window:&mut PagedWindow){
    // Устновка обоев для главного меню
    game.wallpaper=Wallpaper::Colour([1f32,0f32,0f32,1f32]);

    let mut buttons_text=Vec::with_capacity(4);

    if game.settings.continue_game{
        buttons_text.push("Продолжить");
    }
    buttons_text.push("Новая игра");
    buttons_text.push("Растройки :(");
    buttons_text.push("Выход");

    // Настройка меню
    let menu_settings=MenuSettings::new(game_name,buttons_text.into_iter())
            .draw_type(DrawType::Common)
            .head_size([180f32,80f32])
            .buttons_size([180f32,60f32]);

    let menu=Menu::new(menu_settings,window.graphics2d());

    // Добавление заголовка меню
    game.object_map.add_drawable_object(menu.head);

    // Добавление кнопок меню
    for button in menu.buttons{
        let text=button.text.clone();
        game.object_map.add_object(button);
        game.object_map.add_drawable_object(text);
    }

    game.prerendering=settings_menu_prerendering;
    game.updates=Game::empty_updates;
    game.click_handler=settings_menu_click_handler;

}
pub fn settings_menu_prerendering(game:&mut Game){}

pub fn settings_click_handler(game:&mut Game,pressed:bool,button:MouseButton,window:&mut PagedWindow){
    let shift_position=unsafe{
        let position=mouse_cursor.position();
        let shift=mouse_cursor.center_radius();
        [
            position[0]-shift[0]/menu_movement_scale,
            position[1]-shift[1]/menu_movement_scale,
        ]
    };

    if pressed{
        match button{
            MouseButton::Left=>{
                if let Some(mut button)=game.object_map.pressed(shift_position){

                    if !game.settings.continue_game{
                        button+=1;
                    }
                    match button{
                        // continue
                        0=>{
                            println!("pressed")
                        }
                        // new game
                        1=>{
                            println!("pressed")
                        }
                        // settings
                        2=>{
                            println!("pressed")
                        }
                        // exit
                        3=>{
                            println!("pressed")
                        }
                        _=>{

                        }
                    }
                }
            }
            _=>{}
        }
    }
    else{
        match button{
            MouseButton::Left=>{
                if let Some((mut button,clicked))=game.object_map.released(shift_position){
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
                        _=>{

                        }
                    }
                }
            }
            _=>{}
        }
    }
}