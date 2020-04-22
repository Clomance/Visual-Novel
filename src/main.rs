#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,dead_code)]
#![windows_subsystem="windows"] // Отключение консоли

use image::RgbaImage;

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
    Graphics,
    Transformed,
    rectangle::Rectangle,
    Context,
};

use std::{
    str::FromStr,
    fmt::Debug,
    path::Path,
    fs::{File,OpenOptions,metadata,read_dir},
    io::{Read,Write,BufReader,BufRead},
};

use lib::*;

mod game_settings;

mod pages;
use pages::*;

mod page_table;
use page_table::PageTable;

mod character;
use character::*;

mod dialogue;
use dialogue::*;

mod user_interface;
use user_interface::*;

mod game_window;
use game_window::*;

mod textures;
use textures::Textures;

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

pub const dialogues_font_size:u32=24;

pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();

pub static mut smooth:f32=default_page_smooth; // Сглаживание для переходов
pub static mut alpha_channel:f32=0f32; // Значение альфа-канала

pub static mut textures:Textures=Textures::new();


pub static mut loading:bool=true; // Флаг загрузки
pub struct LoadingFlag; // Флаг загрузки, автоматически сбрасывается при завершении загрузки или вылете

impl Drop for LoadingFlag{
    fn drop(&mut self){
        unsafe{
            loading=false
        }
    }
}

fn main(){
    unsafe{
        Settings.load(); // Загрузка настроек

        let mut window:GameWindow=GameWindow::new(); // Создание окна и загрузка функций OpenGL

        let mut dialogues:Vec<Dialogue>=Vec::new(); // Массив диалогов

        let mut dialogues_ref=SyncRawPtr::new(&mut dialogues as *mut Vec<Dialogue>);

        // Замыкание для допольнительного потока
        let loading_resources_thread=move||{
            let _flag=LoadingFlag; // Флаг загрузки

            textures=Textures::load();// Загрузка текстур
            if !loading{return}

            *dialogues_ref=load_dialogues();// Загрузка диалогов
        };

        // Экран загрузки
        match LoadingScreen::new().start(&mut window,loading_resources_thread){
            Game::Exit=>{
                return
            },
            _=>{}
        }

        let mut characters_view=CharactersView::new(); // "Сцена" для персонажей

        let mut dialogue_box=DialogueBox::new(textures.dialogue_box()); // Диалоговое окно

        // Полный цикл игры
        'game:loop{
            window.set_wallpaper_image(textures.main_menu_wallpaper()); // Устрановка обоев главного меню

            // Цикл главного меню
            match MainMenu::new().start(&mut window){
                Game::ContinueGamePlay=>{
                    //
                }
                Game::NewGamePlay=>{
                    Settings._continue=true;
                    Settings.saved_page=0;
                    Settings.saved_dialogue=0;
                    dialogue_box.set_step(0);
                }
                Game::Exit=>break 'game,
                _=>{}
            };

            // Загрузка таблицы страниц игры
            let mut page_table=PageTable::new(&textures,&dialogues);

            smooth=default_page_smooth;

            'gameplay:loop{
                alpha_channel=0f32;

                characters_view.clear();
                characters_view.add_character(page_table.current_character(),CharacterLocation::Left);

                window.set_wallpaper_image(page_table.current_wallpaper()); // Установка текущего фона игры

                dialogue_box.set_dialogue(page_table.current_dialogue()); // Установка текущего диалога

                // Сглаживание перехода
                'smooth:while let Some(event)=window.next_event(){
                    match event{
                        GameWindowEvent::Exit=>break 'game, // Закрытие игры

                        GameWindowEvent::Draw=>{ //Рендеринг
                            window.set_wallpaper_alpha(alpha_channel);
                            window.draw_with_wallpaper(|c,g|{
                                dialogue_box.set_alpha_channel(alpha_channel);
                                dialogue_box.draw_without_text(&c,g);
                            });

                            alpha_channel+=smooth;
                            if alpha_channel>1.0{
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

                        GameWindowEvent::Draw=>{ //Рендеринг
                            window.draw_with_wallpaper(|c,g|{
                                characters_view.draw(&c,g);
                                dialogue_box.draw(&c,g);
                            });
                        }

                        GameWindowEvent::MouseReleased(button)=>{
                            match button{
                                MouseButton::Left=>{
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
                                        Game::Settings=>{
                                            match SettingsPage::new().start(&mut window){
                                                Game::Exit=>{
                                                    Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                                    break 'game
                                                }
                                                _=>{}
                                            }
                                        }
                                        Game::Exit=>{ // Выход из игры
                                            Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                            break 'game
                                        }
                                        _=>{}
                                    }
                                }
                                _=>{}
                            }
                        }
                        _=>{}
                    }
                    // Конец цикла страницы
                }
                // Конец цикла только игровой части
            }

            Settings._continue=false; // Отключение "продолжить игру"

            window.set_wallpaper_image(textures.ending_wallpaper()); // Конечная заставка игры

            smooth=default_page_smooth;
            alpha_channel=0f32;

            'smooth_ending:while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>break 'game, // Закрытие игры

                    GameWindowEvent::Draw=>{ //Рендеринг
                        window.set_wallpaper_alpha(alpha_channel);
                        window.draw_wallpaper();

                        alpha_channel+=smooth;
                        if alpha_channel>1.0{
                            break 'smooth_ending
                        }
                    }
                    _=>{}
                }
            }

            'gameplay_ending:while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>break 'game, // Закрытие игры

                    GameWindowEvent::Draw=>{ //Рендеринг
                        window.draw_wallpaper();
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

pub fn load_dialogues()->Vec<Dialogue>{
    let meta=metadata("text").unwrap();
    let mut dialogues=Vec::with_capacity(meta.len() as usize);
    let dir=read_dir("text").unwrap();

    for r in dir{
        let file=r.unwrap();
        let path=file.path();
        let dialogue=Dialogue::new(path);
        dialogues.push(dialogue);
    }
    dialogues
}