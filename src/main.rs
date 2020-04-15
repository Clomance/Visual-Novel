#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,dead_code)]
//#![windows_subsystem="windows"] // Отключение консоли

use image::{
    self,
    DynamicImage,
    RgbaImage,
    imageops::FilterType,
};

use opengl_graphics::{
    GlGraphics,
    GlyphCache,
    TextureSettings,
    Texture,
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

mod game_window;
use game_window::{
    window_height,
    window_width,
    window_center,
    mouse_cursor,
    GameWindow,
    GameWindowEvent,
    MouseButton,
    KeyboardButton,
};

#[derive(Eq,PartialEq)]
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

pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();

pub static mut smooth:f32=default_page_smooth; // Сглаживание для переходов
pub static mut alpha_channel:f32=0f32; // Значение альфа-канала

pub static mut loading:bool=true;

fn main(){
    unsafe{
        Settings.load(); // Загрузка настроек

        let mut window:GameWindow=GameWindow::new(); // Создание окна и загрузка функций OpenGL

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
            match LoadingScreen::new().start(&mut window){
                Game::Exit=>{
                    loading_resources_thread.join().unwrap();
                    return
                },
                _=>{}
            }
            loading_resources_thread.join().unwrap();

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

        window.set_cursor_position(window_center);  // Перенос курсора
        mouse_cursor.set_position(window_center);   // в центр экрана

        // Полный цикл игры
        'game:loop{
            wallpaper.set_image(&main_menu_wallpaper); // Устрановка обоев главного меню
            // Цикл главного меню
            match MainMenu::new(&mut wallpaper).start(&mut window){
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
                'smooth:while let Some(event)=window.next_event(){
                    match event{
                        GameWindowEvent::Exit=>break 'game, // Закрытие игры

                        GameWindowEvent::MouseMovement((x,y))=>{
                            mouse_cursor.set_position([x,y]);
                            wallpaper.move_with_cursor([x,y]);
                        }

                        GameWindowEvent::Draw=>{ //Рендеринг
                            window.draw(|c,g|{
                                wallpaper.draw_smooth(alpha_channel,&c,g);

                                dialogue_box.set_alpha_channel(alpha_channel);
                                dialogue_box.draw_without_text(&c,g);
                            });

                            alpha_channel+=smooth;
                            if alpha_channel>=1.0{
                                break 'smooth
                            }
                        }
                        _=>{}
                    }
                }


                // Цикл страницы 'page
                while let Some(event)=window.next_event(){
                    match event{
                        GameWindowEvent::Exit=>{ // Закрытие игры
                            Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                            break 'game
                        }

                        GameWindowEvent::MouseMovement((x,y))=>{
                            mouse_cursor.set_position([x,y]);
                            wallpaper.move_with_cursor([x,y]);
                        }

                        GameWindowEvent::Draw=>{ //Рендеринг
                            window.draw(|c,g|{
                                wallpaper.draw(&c,g);
                                page_table.currents_character().draw(&c.draw_state,c.transform,g);
                                dialogue_box.draw(&c,g);
                                mouse_cursor.draw(&c,g);
                            });
                        }

                        GameWindowEvent::MousePressed(button)=>{
                            match button{
                                MouseButton::Left=>{
                                    mouse_cursor.pressed();
                                }
                                _=>{}
                            }
                        }

                        GameWindowEvent::MouseReleased(button)=>{
                            match button{
                                MouseButton::Left=>{
                                    mouse_cursor.released();
                                    if dialogue_box.clicked(){
                                        if dialogue_box.next_page(){
                                            if page_table.next_page(){
                                                continue 'gameplay // Переход к следующей странице (break 'page)
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

                        GameWindowEvent::KeyboardReleased(button)=>{
                            match button{
                                KeyboardButton::Space=>{
                                    if dialogue_box.next_page(){
                                        if page_table.next_page(){
                                            continue 'gameplay // Переход к следующей странице (break 'page)
                                        }
                                        else{
                                            break 'gameplay
                                        }
                                    }
                                }
                                KeyboardButton::Escape=>{
                                    // Пауза
                                    match PauseMenu::new().start(&mut window){
                                        Game::MainMenu=>{ // Возвращение в гланое меню
                                            Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                            continue 'game
                                        }
                                        Game::Exit=>{ // Выход из игры
                                            Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                            break 'game
                                        }
                                        _=>{}
                                    }
                                    wallpaper.move_with_cursor(mouse_cursor.position())
                                }
                                _=>{}
                            }
                        }
                        _=>{}
                    }
                    //Конец цикла страницы
                }
                // Конец цикла только игровой части
            }

            Settings._continue=false; // Отключение "продолжить игру"

            wallpaper.set_image(&ending_wallpaper);// Конечная заставка игры

            smooth=default_page_smooth;
            alpha_channel=0f32;

            'smooth_ending:while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>break 'game, // Закрытие игры

                    GameWindowEvent::Draw=>{ //Рендеринг
                        window.draw(|c,g|{
                            wallpaper.draw_smooth(alpha_channel,&c,g);
                        });
                        alpha_channel+=smooth;
                        if alpha_channel>=1.0{
                            break 'smooth_ending
                        }
                    }
                    _=>{}
                }
            }

            'gameplay_ending:while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>break 'game, // Закрытие игры

                    GameWindowEvent::MouseMovement((x,y))=>mouse_cursor.set_position([x,y]),

                    GameWindowEvent::Draw=>{ //Рендеринг
                        window.draw(|c,g|{
                            wallpaper.draw(&c,g);
                        });
                    }

                    GameWindowEvent::MouseReleased(_button)=>break 'gameplay_ending,
                    GameWindowEvent::KeyboardReleased(_button)=>break 'gameplay_ending,

                    _=>{}
                }
            }
        }
        // Конец программы
        Settings.save(); // Сохранение настроек игры
    }
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