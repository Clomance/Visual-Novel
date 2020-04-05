#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,dead_code)]
//#![windows_subsystem="windows"] // Отключение консоли

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
    Size,
    WindowSettings,
    event_loop::{EventLoop,EventSettings,Events},
    input::{Button,Key,MouseButton},
    ReleaseEvent,
    RenderEvent,
    MouseCursorEvent,
    CloseEvent,
    TextEvent,
    Event
};

use graphics::{
    line::Line,
    character::CharacterCache,
    text::Text,
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
    fmt::Debug,
    path::Path,
    fs::OpenOptions,
    io::{Read,Write},
    str::Lines,
};

use lib::SyncRawPtr;

mod game_settings;

mod pages;
use pages::*;

mod page_table;
use page_table::PageTable;

mod wallpaper;
use wallpaper::Wallpaper;

mod character;
use character::Character;

mod dialogue;
use dialogue::Dialogue;

mod user_interface;
use user_interface::*;

mod colors;
use colors::*;

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

const Game_name:&str="Visual Novel by Clomance";

const default_window_size:Size=Size{
    width:0f64,
    height:0f64
};

pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();

pub static mut window_height:f64=0f64;
pub static mut window_width:f64=0f64;

pub static mut mouse_position:[f64;2]=[0f64;2];

pub static mut smooth:f32=0f32; // Сглаживание для переходов
pub static mut alpha_channel:f32=0f32; // Значение альфа-канала

pub static mut loading:bool=true;

fn main(){
    unsafe{
        // Загрузка настроек
        Settings.load();

        //                                         \\
        // Создание окна и загрузка функций OpenGL \\
        //                                         \\
        let opengl=OpenGL::V3_2;

        let mut window:GlutinWindow=WindowSettings::new(Game_name,default_window_size)
                .exit_on_esc(false)
                .vsync(true)
                .fullscreen(true)
                .graphics_api(opengl)
                .resizable(false)
                .build().expect("Could not create window"); // Получение окна либо вылет
        window.set_automatic_close(true);
        
        let mut gl=GlGraphics::new(opengl);

        let mut events=Events::new(EventSettings::new().lazy(false).ups(60));

        //-----------------------------------------\\
        {
            let size=glutin::event_loop::EventLoop::new().primary_monitor().size(); //
            window_width=size.width as f64;                                         // Получение размеров экрана
            window_height=size.height as f64;                                       // и заполнение его окном игры
            window.set_size([window_width,window_height]);                          //
        }


        let texture_settings=TextureSettings::new();

        let mut characters:Vec<Character>=Vec::with_capacity(Settings.characters_len); // Массив персонажей
        let mut wallpaper_textures:Vec<RgbaImage>=Vec::with_capacity(Settings.pages); // Массив обоев для игры
        let mut dialogues:Vec<Dialogue>=Vec::with_capacity(Settings.pages); // Массив диалогов


        let mut wallpaper_textures_ref=SyncRawPtr::new(&mut wallpaper_textures as *mut Vec<RgbaImage>);
        
        let ending_wallpaper;
        let dialogue_box_texture;
        let main_menu_wallpaper;

        //   Загрузка данных   \\
        //                     \\
        {
            // Главные текстуры (главное меню, диалоговое окно и конечная заставка)
            let mut main_textures:Vec<RgbaImage>=Vec::with_capacity(3);
            let mut main_textures_ref=SyncRawPtr::new(&mut main_textures as *mut Vec<RgbaImage>);

            // Доп поток для загрузки данных
            let loading_resources_thread=std::thread::spawn(move||{
                'loading:loop{

                    // Загрузка обоев
                    let mut path;
                    let mut wallpaper_texture;

                    // Загрузка текстур для игрового процесса
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

        // Обои
        let mut wallpaper=Wallpaper::new(&main_menu_wallpaper);

        // Диалоговое окно
        let dialogue_box_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let mut dialogue_box=DialogueBox::new(dialogue_box_texture,dialogue_box_glyphs);

        // Полный цикл игры
        'game:loop{
            // Устрановка обоев главного меню
            wallpaper.set_image(&main_menu_wallpaper);
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

            dialogues.clear();

            // Загрузка диалогов
            for i in 0..Settings.pages{
                let path=format!("text/dialogue{}.txt",i);
                let dialogue=Dialogue::new(path,&Settings.user_name);
                dialogues.push(dialogue);
            }

            // Загрузка таблицы страниц игры
            let mut page_table=PageTable::new(&characters,&mut wallpaper_textures,&dialogues);

            smooth=1f32/32f32; // 1 к количеству кадров перехода

            'gameplay:loop{
                alpha_channel=0f32;
                // Установка текущего фона игры
                wallpaper.set_image(page_table.current_wallpaper());
                // Установка текущего диалога
                dialogue_box.set_dialogue(page_table.current_dialogue());

                // Сглаживание перехода
                'smooth:while let Some(e)=events.next(&mut window){
                    // Закрытие игры
                    if let Some(_close)=e.close_args(){
                        break 'game
                    }
                    // Рендеринг
                    if let Some(r)=e.render_args(){
                        gl.draw(r.viewport(),|c,g|{
                            wallpaper.set_alpha_channel(alpha_channel);
                            wallpaper.draw(&c,g);

                            dialogue_box.set_alpha_channel(alpha_channel);
                            dialogue_box.draw(&c.draw_state,c.transform,g);
                        });

                        alpha_channel+=smooth;
                        if alpha_channel>=1.0{
                            break 'smooth
                        }
                    }
                }

                // Цикл страницы
                'page:while let Some(e)=events.next(&mut window){
                    // Закрытие игры
                    if let Some(_close)=e.close_args(){
                        Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step());
                        break 'game
                    }
                    mouse_cursor_movement(&e); // Движение мыши
                    // Рендеринг
                    if let Some(r)=e.render_args(){
                        gl.draw(r.viewport(),|c,g|{
                            wallpaper.draw(&c,g);
                            page_table.currents_character().draw(&c.draw_state,c.transform,g);
                            dialogue_box.draw(&c.draw_state,c.transform,g);
                        });
                    }

                    // 
                    if let Some(button)=e.release_args(){
                        match button{
                            Button::Keyboard(key)=>{
                                match key{
                                    Key::Space=>{
                                        if !dialogue_box.next(){
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
                                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step());
                                                continue 'game
                                            }
                                            Game::Exit=>{ // Выход из игры
                                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step());
                                                break 'game
                                            }
                                            _=>{}
                                        }
                                    }
                                    _=>{}
                                }
                            }
                            Button::Mouse(key)=>{
                                match key{
                                    MouseButton::Left=>{
                                        if dialogue_box.clicked(){
                                            if !dialogue_box.next(){
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

            // Конечная заставка игры
            wallpaper.set_image(&ending_wallpaper);

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
        Settings.save();
    }
}

// Движение курсором мыши
pub fn mouse_cursor_movement(event:&Event){
    if let Some(mouse)=event.mouse_cursor_args(){
        unsafe{
            mouse_position=mouse;
        }
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


// Загрузочный экран
pub unsafe fn loading_screen(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    let texture_settings=TextureSettings::new();
    let half_size=100f64;
    let logo=Image::new().rect([
        0f64,
        0f64,
        200f64,
        200f64
    ]);
    let logo_texture=Texture::from_path("images/logo.png",&texture_settings).unwrap();

    let (x,y)=(window_width/2f64,window_height/2f64);
    let mut rotation=0f64;

    'loading:while let Some(e)=events.next(window){
        if !loading{
            break 'loading
        }
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            loading=false;
            return Game::Exit
        } 
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                g.clear_color(White);
                logo.draw(&logo_texture,&c.draw_state,c.transform.trans(x,y).rot_rad(rotation).trans(-half_size,-half_size),g);
            });
            rotation+=0.1f64;
        }
    }

    // Для планого перехода
    let mut frames=5;
    while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            loading=false;
            return Game::Exit
        }
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|_c,g|{
                g.clear_color(White);
            });
            frames-=1;
            if frames==0{
                break
            }
        }
    }

    return Game::MainMenu
}