#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,dead_code)]
#![windows_subsystem="windows"] // Отключение консоли

use glutin_window::GlutinWindow;

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
};

use std::{
    fmt::Debug,
    path::Path,
    fs::OpenOptions,
    io::{Read,Write},
    str::Lines,
};

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

const Game_name:&str="Visual Novel by Clomance";
const default_window_size:Size=Size{
    width:0f64,
    height:0f64
};


pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();

pub static mut window_height:f64=0f64;
pub static mut window_width:f64=0f64;

pub static mut mouse_position:[f64;2]=[0f64;2];

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

        // Загрузка ресурсов игры \\
        //                        \\

        let mut characters:Vec<Character>=Vec::with_capacity(Settings.characters_len); // Массив персонажей
        let mut wallpaper_textures:Vec<Texture>=Vec::with_capacity(Settings.pages); // Массив обоев для игры
        let mut dialogues:Vec<Dialogue>=Vec::with_capacity(Settings.pages); // Массив диалогов


        // Загрузка обоев
        for i in 0..Settings.page_wallpapers{
            let path=format!("images/wallpapers/wallpaper{}.jpg",i);
            let wallpaper_texture=Texture::from_path(path,&texture_settings).unwrap();
            wallpaper_textures.push(wallpaper_texture);
        }

        // Загрузка персонажей
        for i in 0..Settings.characters_len{
            let path=format!("images/characters/character{}.png",i);
            characters.push(Character::new(path,&texture_settings));
        }

        // Создание элементов интерфейса //
        //                               //
        // Обои
        let main_menu_wallpaper_texture=Texture::from_path("images/wallpapers/main_menu_wallpaper.jpg",&texture_settings).unwrap();
        let ending_wallpaper_texture=Texture::from_path("images/wallpapers/ending_wallpaper.jpg",&texture_settings).unwrap();
        let mut wallpaper=Wallpaper::new(&main_menu_wallpaper_texture);

        //-------------------------------//

        // Сглаживание
        let smooth=1f32/32f32; // 1 к количеству кадров перехода
        let mut alpha;

        // Полный цикл игры
        'game:loop{
            // Устрановка обоев главного меню
            wallpaper.set_texture(&main_menu_wallpaper_texture);
            // Цикл главного меню
            match main_menu(&mut wallpaper,&mut events,&mut window,&mut gl){
                Game::ContinueGamePlay=>{

                }
                Game::NewGamePlay=>{
                    Settings.saved_page=0;
                    Settings.saved_dialog=0;
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
            let mut page_table=PageTable::new(&characters,&wallpaper_textures,&dialogues);

            // Установка фона игры
            wallpaper.set_texture(page_table.current_wallpaper());

            // Диалоговое окно
            let dialogue_box_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
            let dialogue_box_texture=Texture::from_path("images/dialogue_box.png",&texture_settings).unwrap();
            let mut dialogue_box=DialogueBox::new(dialogue_box_texture,dialogue_box_glyphs,page_table.current_dialogue());

            alpha=0f32;

            'smooth:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.set_alpha_channel(alpha);
                        wallpaper.draw(&c.draw_state,c.transform,g);
                        
                        dialogue_box.set_alpha_channel(alpha);
                        dialogue_box.draw(&c.draw_state,c.transform,g);
                    });

                    alpha+=smooth;
                    if alpha>=1.0{
                        break 'smooth
                    }
                }
            }

            // Цикл игровой части //
            //                    //
            'gameplay:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }
                mouse_cursor_movement(&e); // Движение мыши
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.draw(&c.draw_state,c.transform,g);
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
                                            wallpaper.set_texture(page_table.current_wallpaper());
                                            dialogue_box.set_dialogue(page_table.current_dialogue());
                                        }
                                        else{
                                            break 'gameplay
                                        }
                                    }
                                }
                                Key::Escape=>{
                                    // Пауза
                                    match pause_menu(&mut events,&mut window,&mut gl){
                                        Game::MainMenu=>break 'gameplay,
                                        Game::Exit=>break 'game,
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
                                                wallpaper.set_texture(page_table.current_wallpaper());
                                                dialogue_box.set_dialogue(page_table.current_dialogue());
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
                // Конец цикла только игровой части
            }
            // Конец полного цикла игры


            // Конечная заставка игры
            wallpaper.set_texture(&ending_wallpaper_texture);

            'gameplay_ending:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.draw(&c.draw_state,c.transform,g);
                    });
                }
                if let Some(button)=e.release_args(){
                    break 'gameplay_ending
                }
            }
        }
        // Конец программы
        Settings.save();
    }
}

pub fn mouse_cursor_movement(event:&Event){
    if let Some(mouse)=event.mouse_cursor_args(){
        unsafe{
            mouse_position=mouse;
        }
    }
}




    // Logo
    // let half_size=100f64;
    // let rect=[0f64,0f64,200f64,200f64];
    // let logo=Image::new().rect(rect);
    // let logo_texture=Texture::from_path("images/logo.png",&texture_settings).unwrap();

    // let (x,y)=(Settings.window_size[0]/2f64,Settings.window_size[1]/2f64);
    // let mut rotation=0f64;


    // logo.draw(&logo_texture,&c.draw_state,
                    //     c.transform.trans(x,y).rot_rad(rotation).trans(-half_size,-half_size),g);
                    // rotation+=0.01f64;



                // Изменение размеров окна, области рендеринга или ...
                // if let Some(resize)=e.resize_args(){
                //     Settings.window_size=resize.window_size;
                //     wallpaper.fit_screen();
                //     dialogue_box.fit_screen();
                // }