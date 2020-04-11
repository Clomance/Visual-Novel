#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,dead_code)]
#![windows_subsystem="windows"] // Отключение консоли

use glutin_window::GlutinWindow;

use image::{
    self,
    DynamicImage,
    RgbaImage,
    imageops::FilterType,
};

use opengl_graphics::{
    GlGraphics,
    GlyphCache,
    OpenGL,
    TextureSettings,
    Texture,
};

use piston::{
    AdvancedWindow,
    WindowSettings,
    event_loop::{EventLoop,EventSettings,Events},
    input::{Button,Key,MouseButton},
    ReleaseEvent,
    MouseRelativeEvent,
    PressEvent,
    RenderEvent,
    CloseEvent,
    TextEvent,
    Event
};

use graphics::{
    ellipse::Border,
    ellipse::Ellipse,
    line::Line,
    character::CharacterCache,
    types::Color,
    image::Image,
    draw_state::DrawState,
    math::Matrix2d,
    Graphics,
    Transformed,
    rectangle::Rectangle,
    Context,
};

use std::{
    str::FromStr,
    fmt::Debug,
    path::Path,
    fs::OpenOptions,
    io::{Read,Write},
    str::Lines,
};

use lib::*;

mod game_settings;

mod pages;
use pages::*;

mod page_table;
use page_table::PageTable;

mod character;
use character::Character;

mod dialogue;
use dialogue::*;

mod user_interface;
use user_interface::*;

pub enum Game{
    Current,
    Back,
    MainMenu,
    Settings,
    NewGamePlay,
    ContinueGamePlay,
    Pause,
    Exit
}
// Пути главных текстур
const main_textures_paths:&[&'static str]=&[
    "images/wallpapers/main_menu_wallpaper.png", // Главное меню
    "images/dialogue_box.png", // Диалоговое окно
    "images/wallpapers/ending_wallpaper.png", // Конечная заставка
];

pub const dialogues_font_size:u32=24;

pub static mut mouse_cursor:MouseCursor=MouseCursor::new();

pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();

pub static mut window_height:f64=0f64;
pub static mut window_width:f64=0f64;
pub static mut window_center:[f64;2]=[0f64;2];

pub const default_page_smooth:f32=1f32/32f32; // 1 к количеству кадров перехода
pub static mut smooth:f32=default_page_smooth; // Сглаживание для переходов
pub static mut alpha_channel:f32=0f32; // Значение альфа-канала

pub static mut loading:bool=true;

fn main(){
    unsafe{
        Settings.load(); // Загрузка настроек

        //                                         \\
        // Создание окна и загрузка функций OpenGL \\
        //                                         \\
        let opengl=OpenGL::V3_2;

        let mut window:GlutinWindow=WindowSettings::new(&Settings.game_name,[0;2])
                .exit_on_esc(false)
                .vsync(true)
                .fullscreen(true)
                .graphics_api(opengl)
                .resizable(false)
                .build().expect("Could not create window"); // Получение окна либо вылет
        window.set_capture_cursor(true);
        window.set_automatic_close(true);

        let mut gl=GlGraphics::new(opengl);

        let mut events=Events::new(EventSettings::new().lazy(false).ups(60));

        fit_monitor(&mut window); // Заполнение монитора (полноэкранный режим)
        //-----------------------------------------\\

        mouse_cursor.set_position([window_width/2f64,window_height/2f64]);

        let texture_settings=TextureSettings::new();

        let mut characters:Vec<Character>=Vec::with_capacity(Settings.characters_len); // Массив персонажей
        let mut wallpaper_textures:Vec<RgbaImage>=Vec::with_capacity(Settings.pages); // Массив обоев для игры
        let mut dialogues:Vec<Dialogue>=Vec::with_capacity(Settings.pages); // Массив диалогов

        let mut wallpaper_textures_ref=SyncRawPtr::new(&mut wallpaper_textures as *mut Vec<RgbaImage>); // Ссылка для передачи через доп. поток
        let mut dialogues_ref=SyncRawPtr::new(&mut dialogues as *mut Vec<Dialogue>); // Ссылка для передачи через доп. поток

        let ending_wallpaper;
        let dialogue_box_texture;
        let main_menu_wallpaper;

        //                     \\
        //   Загрузка данных   \\
        //                     \\
        {
            let mut main_textures:Vec<RgbaImage>=Vec::with_capacity(3); // Главные текстуры (главное меню, диалоговое окно и конечная заставка)
            let mut main_textures_ref=SyncRawPtr::new(&mut main_textures as *mut Vec<RgbaImage>); // Ссылка для передачи через доп. поток

            // Доп. поток для загрузки данных
            let loading_resources_thread=std::thread::spawn(move||{
                'loading:loop{
                    
                    let mut path;
                    let mut wallpaper_texture;

                    // Загрузка обоев
                    for i in 0..Settings.page_wallpapers{
                        if !loading{
                            break 'loading
                        }
                        path=format!("images/wallpapers/wallpaper{}.png",i);
                        wallpaper_texture=load_image(path);
                        wallpaper_textures_ref.as_mut().push(wallpaper_texture);
                    }

                    // Загрузка главных текстур
                    for path in main_textures_paths{
                        if !loading{
                            break 'loading
                        }
                        wallpaper_texture=load_image(path);
                        main_textures_ref.as_mut().push(wallpaper_texture);
                    }

                    // Загрузка диалогов
                    for i in 0..Settings.pages{
                        if !loading{
                            break 'loading
                        }
                        let path=format!("text/dialogue{}.txt",i);
                        let dialogue=Dialogue::new(path);
                        dialogues_ref.as_mut().push(dialogue);
                    }

                    loading=false;
                    break 'loading
                }
            });
            // Экран загрузки
            match loading_screen(&mut events,&mut window,&mut gl){
                Game::Exit=>{
                    loading_resources_thread.join();
                    return
                },
                _=>{}
            }
            loading_resources_thread.join();

            // Перенос главный текстур
            ending_wallpaper=main_textures.pop().unwrap();
            dialogue_box_texture=Texture::from_image(&main_textures.pop().unwrap(),&texture_settings);
            main_menu_wallpaper=main_textures.pop().unwrap();
        }

        // Загрузка персонажей
        for i in 0..Settings.characters_len{
            let path=format!("images/characters/character{}.png",i);
            characters.push(Character::new(path,&texture_settings));
        }

        let dx=window_width/(wallpaper_movement_scale*2f64);
        let dy=window_height/(wallpaper_movement_scale*2f64);
        let wallpaper_rect=[
            -dx,
            -dy,
            window_width+2f64*dx,
            window_height+2f64*dy,
        ];

        let mut wallpaper=Wallpaper::new(&main_menu_wallpaper,wallpaper_rect); // Обои

        // Диалоговое окно
        let dialogue_box_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let mut dialogue_box=DialogueBox::new(dialogue_box_texture,dialogue_box_glyphs);

        // Полный цикл игры
        'game:loop{
            wallpaper.set_image(&main_menu_wallpaper); // Устрановка обоев главного меню
            // Цикл главного меню
            match main_menu(&mut wallpaper,&mut events,&mut window,&mut gl){
                Game::ContinueGamePlay=>{
                    //
                }
                Game::NewGamePlay=>{
                    Settings._continue=true;
                    Settings.saved_page=0;
                    Settings.saved_dialogue=0;
                }
                Game::Exit=>break 'game,
                _=>{}
            };

            // Загрузка таблицы страниц игры
            let mut page_table=PageTable::new(&characters,&mut wallpaper_textures,&dialogues);

            smooth=default_page_smooth;

            'gameplay:loop{
                alpha_channel=0f32;

                wallpaper.set_image(page_table.current_wallpaper()); // Установка текущего фона игры

                dialogue_box.set_dialogue(page_table.current_dialogue()); // Установка текущего диалога

                // Сглаживание перехода
                'smooth:while let Some(e)=events.next(&mut window){
                    // Закрытие игры
                    if let Some(_close)=e.close_args(){
                        break 'game
                    }
                    //mouse_cursor.movement_wallpaper(&e,&mut wallpaper); // Движение мыши
                    // Рендеринг
                    if let Some(r)=e.render_args(){
                        gl.draw(r.viewport(),|c,g|{
                            wallpaper.draw_smooth(alpha_channel,&c,g);

                            dialogue_box.set_alpha_channel(alpha_channel);
                            dialogue_box.draw_without_text(&c,g);
                            //mouse_cursor.draw(&c,g);
                        });

                        alpha_channel+=smooth;
                        if alpha_channel>=1.0{
                            break 'smooth
                        }
                    }
                }


                // Цикл страницы
                while let Some(e)=events.next(&mut window){
                    // Закрытие игры
                    if let Some(_close)=e.close_args(){
                        Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                        break 'game
                    }
                    mouse_cursor.movement_wallpaper(&e,&mut wallpaper); // Движение мыши
                    // Рендеринг
                    if let Some(r)=e.render_args(){
                        gl.draw(r.viewport(),|c,g|{
                            wallpaper.draw(&c,g);
                            page_table.currents_character().draw(&c.draw_state,c.transform,g);
                            dialogue_box.draw(&c,g);
                            mouse_cursor.draw(&c,g);
                        });
                    }

                    if Some(Button::Mouse(MouseButton::Left))==e.press_args(){
                        mouse_cursor.pressed();
                    }

                    // 
                    if let Some(button)=e.release_args(){
                        match button{
                            Button::Keyboard(key)=>{
                                match key{
                                    Key::Space=>{
                                        if dialogue_box.next_page(){
                                            if page_table.next_page(){
                                                continue 'gameplay // (break 'page)
                                            }
                                            else{
                                                break 'gameplay
                                            }
                                        }
                                    }
                                    Key::Escape=>{
                                        // Пауза
                                        match pause_menu(&mut events,&mut window,&mut gl){
                                            Game::MainMenu=>{ // Возвращение в гланое меню
                                                mouse_cursor.movement_wallpaper_saved(&mut wallpaper);
                                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                                continue 'game
                                            }
                                            Game::Exit=>{ // Выход из игры
                                                mouse_cursor.movement_wallpaper_saved(&mut wallpaper);
                                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                                break 'game
                                            }
                                            _=>{}
                                        }
                                        mouse_cursor.movement_wallpaper_saved(&mut wallpaper);
                                    }
                                    _=>{}
                                }
                            }
                            Button::Mouse(key)=>{
                                match key{
                                    MouseButton::Left=>{
                                        mouse_cursor.released();

                                        if dialogue_box.clicked(){
                                            if dialogue_box.next_page(){
                                                if page_table.next_page(){
                                                    continue 'gameplay
                                                }
                                                else{
                                                    break 'gameplay
                                                }
                                            }
                                        }
                                    }
                                    _=>{}
                                }
                            }
                            _=>{}
                        }
                    }
                    //Конец цикла страницы
                }
                // Конец цикла только игровой части
            }

            Settings._continue=false; // Отключение "продолжить игру"

            wallpaper.set_image(&ending_wallpaper);// Конечная заставка игры

            smooth=default_page_smooth;
            alpha_channel=0f32;

            'smooth_ending:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.draw_smooth(alpha_channel,&c,g);
                    });

                    alpha_channel+=smooth;
                    if alpha_channel>=1.0{
                        break 'smooth_ending
                    }
                }
            }

            'gameplay_ending:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.draw(&c,g);
                    });
                }
                if let Some(_button)=e.release_args(){
                    break 'gameplay_ending
                }
            }
        }
        // Конец программы
        Settings.save(); // Сохранение настроек игры
    }
}
// Получение размеров экрана и заполнение его окном игры
pub unsafe fn fit_monitor(window:&mut GlutinWindow){
    let size=lib::get_monitor_size();

    window_width=size[0];
    window_height=size[1];

    window_center=[size[0]/2f64,size[1]/2f64];

    window.set_size([window_width,window_height]);
}

// Загрузка изображений
pub fn load_image<P:AsRef<Path>>(path:P)->RgbaImage{
    let mut image=image::open(path).unwrap();
    
    image=unsafe{image.resize_exact(window_width as u32,window_height as u32,FilterType::Triangle)};
    if let DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}