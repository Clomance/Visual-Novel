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
    colours::{Gray,Light_blue},
    user_interface::{
        Menu,
        MenuSettings,
        EditTextView,
        EditTextViewSettings,
    },
};

use cat_engine::{
    // types
    Colour,
    // statics
    mouse_cursor,
    window_center,
    // enums
    KeyboardButton,
    // structs
    Window,
    WindowEvent,
    MouseButton,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,Texture},
    image::RgbaImage,
};


const button_pressed:Colour=[
    Light_blue[0]-0.05,
    Light_blue[1]-0.05,
    Light_blue[2]-0.05,
    Light_blue[3],
];
const menu_movement_scale:f32=10f32;

pub struct MainMenu{
    menu:Menu,
    enter_name:bool,
    user_name:EditTextView,
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
                .header_font_size(60f32)
                .button_size([160f32,60f32])
                .button_font_size(26f32);

        let enter_name_rect=unsafe{[
            window_center[0]-120f32,
            window_center[1]-100f32,
            240f32,
            140f32,
        ]};
        let enter_name_settings=EditTextViewSettings::new("",enter_name_rect);

        Self{
            menu:Menu::new(menu_settings,graphics),
            enter_name:false,
            user_name:EditTextView::new(enter_name_settings,graphics),
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

                        if self.enter_name{
                            self.user_name.draw(graphics);
                        }

                        // Отрисовка курсора
                        graphics.draw_shift_textured_object(mouse_cursor_icon_index,[dx,dy]).unwrap();
                    }).unwrap();
                }

                WindowEvent::MousePressed(button)=>{
                    if let MouseButton::Left=button{
                        let [mut x,mut y]=unsafe{mouse_cursor.position()};

                        if self.enter_name{
                            if !self.user_name.in_area(x,y){
                                self.enter_name=false;
                            }
                        }
                        else{
                            let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                            let menu_shift=[
                                dx/menu_movement_scale,
                                dy/menu_movement_scale
                            ];

                            x-=menu_shift[0];
                            y-=menu_shift[1];
                            if let Some(button)=self.menu.pressed(x,y){
                                // Получение индекса кнопки
                                let button_index=self.menu.button_index(button);
                                // Изменение цвета кнопки
                                *graphics.get_simple_object_colour(button_index)=button_pressed;
                            }
                        }
                    }
                }

                WindowEvent::MouseReleased(button)=>{
                    if let MouseButton::Left=button{
                        if let Some(pressed_button)=self.menu.pressed_button(){
                            let [mut x,mut y]=unsafe{mouse_cursor.position()};

                            let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                            let menu_shift=[
                                dx/menu_movement_scale,
                                dy/menu_movement_scale
                            ];

                            x-=menu_shift[0];
                            y-=menu_shift[1];

                            // Получение индекса кнопки
                            let button_index=self.menu.button_index(pressed_button);
                            // Изменение цвета кнопки
                            *graphics.get_simple_object_colour(button_index)=Light_blue;

                            if let Some(mut button)=self.menu.released(x,y){
                                if unsafe{!game_settings.continue_game}{
                                    button+=1;
                                }

                                match button{
                                    // Продолжить игру
                                    0=>{
                                        window.stop_events();
                                    }

                                    // Начать новую игру
                                    1=>{
                                        // Открытие диалога для ввода имени пользователя
                                        self.enter_name=true;
                                    }

                                    // Настройки
                                    2=>{
                                        window.stop_events();
                                    }

                                    // Выход
                                    3=>{
                                        window.stop_events();
                                        result=Game::Exit;
                                    }

                                    _=>{}
                                }
                            }
                        }
                    }
                }

                WindowEvent::CharacterInput(character)=>if self.enter_name{
                    self.user_name.push_char(character,graphics);
                }

                WindowEvent::KeyboardPressed(button)=>match button{
                    KeyboardButton::Escape=>self.enter_name=false,

                    KeyboardButton::Backspace=>if self.enter_name{
                        self.user_name.pop_char(graphics);
                    }

                    KeyboardButton::Enter=>if self.enter_name{
                        unsafe{game_settings.user_name=self.user_name.text(graphics).clone()}
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
        graphics.remove_all_simple_objects();
        // Удаление всех текстовых объектов
        graphics.remove_all_text_objects();
        result
    }
}