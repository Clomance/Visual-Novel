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
    colours::Gray,
    user_interface::{
        Menu,
        MenuSettings,
    },
};

use cat_engine::{
    // statics
    mouse_cursor,
    window_center,
    // structs
    Window,
    WindowEvent,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,Texture},
    image::RgbaImage,
    // fns
    window_rect,
};


const menu_movement_scale:f32=10f32;

pub struct MainMenu{
    menu:Menu,
}

impl MainMenu{
    pub fn new(_window:&Window,graphics:&mut Graphics2D,wallpaper:&RgbaImage)->MainMenu{
        // Изменение картинки обоев
        graphics.get_textured_object_texture(wallpaper_index).update(wallpaper);

        let mut buttons=Vec::with_capacity(4);
        if unsafe{game_settings.continue_game}{
            buttons.push("Продолжить");
        }
        buttons.push("Новая игра");
        buttons.push("Настройки");
        buttons.push("Выход");

        let menu_settings=MenuSettings::new(game_name,buttons.into_iter())
                .header_font_size(50f32)
                .button_size([160f32,60f32])
                .button_font_size(24f32);

        Self{
            menu:Menu::new(menu_settings,graphics),
        }
    }

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D)->Game{
        let mut result=Game::Next;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>{
                    result=Game::Exit;
                }

                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                    let wallpaper_shift=[
                        dx/wallpaper_movement_scale,
                        dy/wallpaper_movement_scale
                    ];
                    let menu_shift=[
                        dx/menu_movement_scale,
                        dy/menu_movement_scale
                    ];
                    window.draw(graphics,|graphics|{
                        // Отрисовка обоев
                        graphics.draw_shift_textured_object(wallpaper_index,wallpaper_shift).unwrap();

                        // Отрисовка меню
                        self.menu.draw_shift(menu_shift,graphics);

                        // Отрисовка курсора
                        graphics.draw_shift_textured_object(mouse_cursor_icon_index,[dx,dy]).unwrap();
                    }).unwrap();
                }
                _=>{

                }
            }
        });

        // Удаление всех простых объектов
        graphics.clear_simple_object_array();

        result
    }
}