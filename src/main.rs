#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,unused_imports)]
//#![windows_subsystem = "windows"] // Отключение консоли

extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate glutin_window;

use glutin_window::GlutinWindow;

use opengl_graphics::{
    Filter,
    GlGraphics,
    GlyphCache,
    OpenGL,
    TextureSettings,
    Texture,
};

use piston::{
    WindowSettings,
    Window,
    event_loop::{EventLoop,EventSettings,Events},
    input::{
        Button,
        ButtonState,
        Key,
        MouseButton
    },
    ButtonEvent,
    UpdateEvent,
    ReleaseEvent,
    RenderEvent,
    ResizeEvent,
    MouseCursorEvent,
    CloseEvent
};

use graphics::{
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
    fs::{OpenOptions,File},
    io::Read,
    str::Lines,
};

use lib::SyncRawPtr;

mod Settings;

mod page_table;
use page_table::PageTable;

mod wallpaper;
use wallpaper::Wallpaper;

mod character;
use character::Character;

mod dialogue;
use dialogue::Dialogue;

mod dialogue_box;
use dialogue_box::DialogueBox;

mod button;
use button::MenuButton;

mod menu;
use menu::Menu;

mod traits;
use traits::Drawable;

mod colors;
use colors::*;

pub enum Game{
    Current,
    MainMenu,
    NewGamePlay,
    ContinueGamePlay,
    Pause,
    Exit
}

const Game_name:&str="Visual Novel by Clomance";

pub static mut Settings:Settings::GameSettings=Settings::GameSettings::new();

pub static mut mouse_position:[f64;2]=[0f64;2];

fn main(){
    unsafe{
        // Загрузка настроек
        Settings.load();

        // Создание окна и загрузка функций OpenGL //
        //                                         //
        let opengl=OpenGL::V3_2;

        let mut window:GlutinWindow=WindowSettings::new(Game_name,Settings.window_size)
                .exit_on_esc(false)
                .vsync(true)
                .fullscreen(Settings.fullscreen)
                .graphics_api(opengl)
                .resizable(false)
            .build().expect("Could not create window");

        let mut gl=GlGraphics::new(opengl);

        let mut events=Events::new(EventSettings::new().lazy(false).ups(60));
        //-----------------------------------------//

        // Загрузка ресурсов //
        //                   //

        let texture_settings=TextureSettings::new();

        let mut wallpaper_textures:Vec<Texture>=Vec::with_capacity(Settings.pages_len);
        //let mut current_characters:Vec<&mut Character>=Vec::with_capacity(Settings.pages_len);
        let mut dialogues:Vec<Dialogue>=Vec::with_capacity(Settings.pages_len);

        let mut characters:Vec<Character>=Vec::with_capacity(2);

        
        let main_menu_wallpaper_texture=Texture::from_path("images/main_menu_wallpaper.jpg",&texture_settings).unwrap();

        for i in 0..Settings.pages_len{
            // Загрузка обоев
            let mut path=format!("images/wallpapers/wallpaper{}.jpg",i);
            let wallpaper_texture=Texture::from_path(path,&texture_settings).unwrap();
            wallpaper_textures.push(wallpaper_texture);

            // Загрузка диалогов
            path=format!("text/dialogue{}.txt",i);
            let dialogue=Dialogue::new(path);
            dialogues.push(dialogue);
        }

        for i in 0..Settings.characters_len{
            // Загрузка персонажей
            let path=format!("images/characters/character{}.png",i);
            characters.push(Character::new(path,&texture_settings));
        }

        let dialogue_box_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let dialogue_box_texture=Texture::from_path("images/dialogue_box.png",&texture_settings).unwrap();

        let mut page_table=PageTable::new(&wallpaper_textures,&dialogues);

        // Создание элементов интерфейса //
        //                               //

        // Обои
        let mut wallpaper=Wallpaper::new(page_table.current_wallpaper());

        // Диалоговое окно
        let mut dialogue_box=DialogueBox::new(dialogue_box_texture,dialogue_box_glyphs,&dialogues[0]);
        

        // Главное меню
        let head=Game_name.to_string();

        let menu_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
    
        let menu_buttons_text=["Играть","Выход"];
    
        let mut menu=Menu::new(head,[100f64,50f64],&menu_buttons_text,menu_glyphs);

        //-------------------------------//

        // Полный цикл игры
        'game:loop{
            //       Цикл главного меню      //
            // Начало загрузки главного меню //
            //                               //
            wallpaper.set_texture(&main_menu_wallpaper_texture);

            //                    //
            // Цикл главного меню //
            //                    //
            'main_menu:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }
                // Движение мыши
                if let Some(mouse)=e.mouse_cursor_args(){
                    mouse_position=mouse;
                }
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.draw(&c.draw_state,c.transform,g);
                        menu.draw(&c.draw_state,c.transform,g);
                    });
                }
                // 
                if let Some(button)=e.release_args(){
                    match button{
                        Button::Mouse(key)=>{
                            match key{
                                MouseButton::Left=>{
                                    if let Some(button_id)=menu.clicked(){
                                        match button_id{
                                            0=>break 'main_menu, // Кнопка начала игрового процесса
                                            1=>break 'game, // Кнопка закрытия игры
                                            _=>{}
                                        }
                                    }
                                }
                                _=>{}
                            }
                        }
                        _=>{}
                    }
                }
            }
            //        Конец главного меню        //
            // Начало загрузки игрового процесса //
            //                                   //
            wallpaper.set_texture(page_table.current_wallpaper());

            //                    //
            // Цикл игровой части //
            //                    //
            'gameplay:while let Some(e)=events.next(&mut window){
                // Закрытие игры
                if let Some(_close)=e.close_args(){
                    break 'game
                }

                // Движение мыши
                if let Some(mouse)=e.mouse_cursor_args(){
                    mouse_position=mouse;
                }
                // Рендеринг
                if let Some(r)=e.render_args(){
                    gl.draw(r.viewport(),|c,g|{
                        wallpaper.draw(&c.draw_state,c.transform,g);
                        characters[0].draw(&c.draw_state,c.transform,g);
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
                                    match pause_menu(&mut events,&mut window,&mut gl){
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
        }
        // Конец программы
    }
}

pub fn pause_menu(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
    

        let background_size=[250f64,400f64];
        let background=Rectangle::new(Pause_menu_background);
        let background_rect=unsafe{[
            
            (Settings.window_size[0]-background_size[0])/2f64,
            (Settings.window_size[1]-background_size[1])/2f64,
            background_size[0],
            background_size[1]
        ]};

        let buttons_size=[100f64,50f64];
        let mut rect=unsafe{[
            (Settings.window_size[0]-buttons_size[0])/2f64,
            (Settings.window_size[1]-buttons_size[1])/2f64,
            buttons_size[0],
            buttons_size[1]
        ]};
        let texture_settings=TextureSettings::new();
        let continue_button_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let mut continue_button=button::Button::new(rect,"Продолжить".to_string(),continue_button_glyphs);

        rect[1]+=buttons_size[1]+10f64;
        let close_button_glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let mut close_button=button::Button::new(rect,"Выйти".to_string(),close_button_glyphs);

        // Цикл обработки
        while let Some(e)=events.next(window){
            // Закрытие игры
            if let Some(_close)=e.close_args(){
                return Game::Exit
            }
            // Движение мыши
            if let Some(mouse)=e.mouse_cursor_args(){
                unsafe{
                    mouse_position=mouse;
                }
            }
            // Рендеринг
            if let Some(r)=e.render_args(){
                gl.draw(r.viewport(),|c,g|{
                    //g.clear_color(Pause_menu_background);
                    background.draw(background_rect,&c.draw_state,c.transform,g);
                    continue_button.draw(&c.draw_state,c.transform,g);
                    close_button.draw(&c.draw_state,c.transform,g);
                });
            }

            if let Some(button)=e.release_args(){
                match button{
                    Button::Keyboard(key)=>{
                        match key{
                            Key::Escape=>{
                                return Game::ContinueGamePlay
                            }
                            _=>{}
                        }
                    }
                    Button::Mouse(key)=>{
                        match key{
                            MouseButton::Left=>{
                                if continue_button.clicked(){
                                    return Game::ContinueGamePlay
                                }
                                if close_button.clicked(){
                                    return Game::Exit
                                }
                            }
                            _=>{}
                        }
                    }
                    _=>{}
                }
            }
        }
        return Game::Exit
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