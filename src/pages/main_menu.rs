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
    set_settings_menu,
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

const page_smooth:f32=default_page_smooth; // Сглаживание переходов - 1 к количеству кадров перехода

const menu_movement_scale:f32=10f32; // Обратный коэфициент сдвига меню при движении мышью

// Индекс картинки для главного меню
// Пока что так
const main_menu_image:usize=0;

pub fn set_main_menu(game:&mut Game,window:&mut PagedWindow){
    // Устновка обоев для главного меню
    window.graphics2d().get_textured_object_texture(wallpaper).update(&game.images[main_menu_image]);
    game.wallpaper=Wallpaper::Texture;

    let mut buttons_text=Vec::with_capacity(4);

    if game.settings.continue_game{
        buttons_text.push("Продолжить");
    }
    buttons_text.push("Новая игра");
    buttons_text.push("Настройки");
    buttons_text.push("Выход");

    // Настройка меню
    let menu_settings=MenuSettings::new(game_name,buttons_text.into_iter())
            .draw_type(DrawType::Shifting([0f32;2]))
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

    game.prerendering=main_menu_prerendering;
    game.updates=Game::empty_updates;
    game.click_handler=main_menu_click_handler;

}

pub fn main_menu_prerendering(game:&mut Game){
    for drawable in game.object_map.get_drawables(){
        if let DrawType::Shifting(shift)=&mut drawable.draw_type{
            let new_shift=unsafe{mouse_cursor.center_radius()};
            *shift=[
                new_shift[0]/menu_movement_scale,
                new_shift[1]/menu_movement_scale
            ];
        }
    }
}

pub fn main_menu_click_handler(game:&mut Game,pressed:bool,button:MouseButton,window:&mut PagedWindow){
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
                                set_settings_menu(game, window);
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
/*

impl<'a,'wallpaper> MainMenu<'a,'wallpaper>{

    pub fn start(mut self,window:&mut DefaultWindow,music:&Audio)->Game{
        self.mouse_shift(unsafe{
            mouse_cursor.center_radius()
        });

        window.set_smooth(page_smooth);

        'main:while self.smooth(window)!=Game::Exit{

            // Цикл самого меню
            while let Some(event)=window.next_event(){

                match event{
                    WindowEvent::CloseRequested=>return Game::Exit, // Закрытие игры
                    //Рендеринг
                    WindowEvent::RedrawRequested=>window.draw(|c,g|{
                        self.draw(c,g);
                    }).unwrap(),
                    
                    // Движение мышки
                    WindowEvent::MouseMovementDelta(shift)=>self.mouse_shift(shift),
                    // Кнопка мышки нажата
                    WindowEvent::MousePressed(button)=>match button{
                        MouseButton::Left=>{
                            self.menu.pressed();
                        }
                        _=>{}
                    }

                    // Кнопка мышки отпущена
                    WindowEvent::MouseReleased(button)=>{
                        match button{
                            MouseButton::Left=>{
                                // Нажата одна из кнопок меню
                                if let Some(button_id)=self.menu.clicked(){
                                    match MenuButtons::button(button_id as u8){
                                        MenuButtons::Continue=>return Game::ContinueGamePlay,

                                        // Кнопка начала нового игрового процесса
                                        // Окно ввода имени захватывает управление над меню
                                        MenuButtons::New=>match EnterUserName::new(&mut self).start(window){
                                            Game::NewGamePlay=>return Game::NewGamePlay,
                                            Game::Exit=>return Game::Exit,
                                            _=>{}
                                        }

                                        MenuButtons::Settings=>unsafe{
                                            mouse_cursor.save_position();
                                            match SettingsPage::new(window).start(window,music){
                                                Game::Exit=>return Game::Exit,
                                                Game::Back=>{
                                                    self.mouse_shift(mouse_cursor.saved_shift());
                                                    continue 'main
                                                }
                                                _=>{}
                                            }
                                        }

                                        MenuButtons::Exit=>return Game::Exit, // Кнопка закрытия игры
                                    }
                                    
                                }
                            }
                            // Отпущенные кнопки мыши
                            _=>{}
                        }
                    }

                    WindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(window,|d,g|{
                                self.wallpaper.draw(d,g);
                                self.draw(d,g);
                            });
                        }
                    }

                    // События окна
                    _=>{}
                }
                // Конец главного цикла (без сглаживания)
            }
            // Конец полного цикла
        }
        Game::Exit
    }

    pub fn smooth(&mut self,window:&mut DefaultWindow)->Game{
        window.set_new_smooth(page_smooth);

        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::CloseRequested=>return Game::Exit, // Закрытие игры

                WindowEvent::MouseMovementDelta(shift)=>self.mouse_shift(shift),

                WindowEvent::RedrawRequested=>{
                    if 1f32<window.draw_smooth(|alpha,c,g|{
                        self.draw_smooth(alpha,c,g);
                    }).unwrap(){
                        break
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.wallpaper.draw_shift(draw_parameters,graphics);
        self.menu.draw(draw_parameters,graphics);
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.wallpaper.draw_shift_smooth(alpha,draw_parameters,graphics);
        self.menu.draw_smooth(alpha,draw_parameters,graphics);
    }

    pub fn mouse_shift(&mut self,[dx,dy]:[f32;2]){
        let [dx,dy]=[dx/menu_movement_scale,dy/menu_movement_scale];
        self.menu.shift([dx,dy]);
        self.wallpaper.mouse_shift(unsafe{mouse_cursor.center_radius()});
    }
}
*/