#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,dead_code)]
#![windows_subsystem="windows"]

use engine::{
    // statics
    window_width,
    window_height,
    window_center,
    mouse_cursor,
    // structs
    GameWindow,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
    // mods
    music
};

use std::{
    fs::{metadata,read_dir},
};

use lib::{
    *,
    colours::*,
};

mod game_settings;

mod pages;
use pages::*;

mod page_table;
use page_table::*;

mod characters;
use characters::*;

mod dialogue;
use dialogue::*;

mod textures;
use textures::Textures;

mod dialogue_box;
pub use dialogue_box::DialogueBox;

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

const page_smooth:f32=1f32/32f32;

pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();


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
    let mut texture_base:Textures=Textures::new();

    unsafe{
        Settings.load(); // Загрузка настроек
        let mut window:GameWindow=GameWindow::new(&Settings.game_name); // Создание окна и загрузка функций OpenGL

        let window_size=window.size();
        window_width=window_size[0];
        window_height=window_size[1];
        window_center[0]=window_width/2f32;
        window_center[1]=window_height/2f32;

        let wallpaper_size={
            let dx=window_width/(wallpaper_movement_scale*2f32);
            let dy=window_height/(wallpaper_movement_scale*2f32);
            [
                (window_width+2f32*dx),
                (window_height+2f32*dy)
            ]
        };
    
        let mut dialogues:Vec<Dialogue>=Vec::new(); // Массив диалогов

        let mut dialogues_ref=SyncRawPtr::new(&mut dialogues as *mut Vec<Dialogue>);

        let mut texture_base_ref=SyncRawPtr::new(&mut texture_base as *mut Textures);

        // Замыкание для допольнительного потока
        let loading_resources_thread=move||{
            let _flag=LoadingFlag; // Флаг загрузки

            *texture_base_ref=Textures::load();// Загрузка текстур
            if !loading{return}

            *dialogues_ref=load_dialogues();// Загрузка диалогов
        };

        // Экран загрузки
        match LoadingScreen::new(&mut window).start(&mut window,loading_resources_thread){
            Game::Exit=>{
                return
            },
            _=>{}
        }

        let mut wallpaper=Wallpaper::new(texture_base.main_menu_wallpaper(),window.display());
        let mut characters_view=CharactersView::new(); // "Сцена" для персонажей

        let mut dialogue_box=DialogueBox::new(texture_base.dialogue_box(),window.display()); // Диалоговое окно

        let mut music=music::Music::new();
        music.add_music("./resources/music/audio.mp3");
        music.set_volume(Settings.volume);
        music.start_music(0);


        // Полный цикл игры
        'game:loop{
            wallpaper.update_image(texture_base.main_menu_wallpaper()); // Устрановка обоев главного меню
            // Цикл главного меню
            match MainMenu::new(&mut wallpaper).start(&mut window,&music){
                Game::ContinueGamePlay=>{
                    //
                }
                Game::NewGamePlay=>{
                    Settings.continue_game=true;
                    Settings.saved_page=0;
                    Settings.saved_dialogue=0;
                    dialogue_box.set_step(0);

                    if Intro::new(&mut window).start()==Game::Exit{
                        break 'game
                    }
                }
                Game::Exit=>break 'game,
                _=>{}
            };

            // Загрузка таблицы страниц игры
            let mut page_table=PageTable::new(&texture_base,&dialogues);

            'gameplay:loop{
                characters_view.clear();
                for (character,location) in page_table.current_character(){
                    characters_view.add_character(character,location.clone(),window.display());
                }

                let wallpaper_path=page_table.current_wallpaper();
                let current_wallpaper=textures::load_wallpaper_image(wallpaper_path,wallpaper_size[0],wallpaper_size[1]);

                wallpaper.update_image(&current_wallpaper); // Установка текущего фона игры

                dialogue_box.set_dialogue(page_table.current_dialogue()); // Установка текущего диалога

                'page:loop{
                    window.set_new_smooth(page_smooth);
                    // Сглаживание перехода
                    'opening_page:while let Some(event)=window.next_event(){
                        match event{
                            WindowEvent::Exit=>break 'game, // Закрытие игры

                            WindowEvent::MouseMovementDelta((dx,dy))=>{
                                wallpaper.mouse_shift(dx,dy);
                            }

                            WindowEvent::Draw=>{ //Рендеринг
                                if 1f32<window.draw_smooth(|alpha,c,g|{
                                    g.clear_colour(White);
                                    wallpaper.draw_smooth(alpha,c,g);
                                    characters_view.draw_smooth(alpha,c,g);
                                    dialogue_box.set_alpha_channel(alpha);
                                    dialogue_box.draw_without_text(c,g);
                                }){
                                    break 'opening_page
                                }
                            }

                            WindowEvent::KeyboardReleased(button)=>{
                                if button==KeyboardButton::F5{
                                    make_screenshot(&window)
                                }
                            }
                            _=>{}
                        }
                    }

                    // Цикл страницы 'page
                    'page_inner:while let Some(event)=window.next_event(){
                        match event{
                            WindowEvent::Exit=>{ // Закрытие игры
                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                break 'game
                            }

                            WindowEvent::MouseMovementDelta((dx,dy))=>{
                                wallpaper.mouse_shift(dx,dy);
                            }

                            WindowEvent::Draw=>{ //Рендеринг
                                window.draw(|c,g|{
                                    wallpaper.draw(c,g);
                                    characters_view.draw(c,g);
                                    dialogue_box.draw(c,g);
                                });
                            }

                            WindowEvent::MouseReleased(button)=>{
                                match button{
                                    MouseButton::Left=>{
                                        if dialogue_box.clicked(){
                                            if dialogue_box.next_page(){
                                                if page_table.next_page(){
                                                    break 'page_inner // Переход к следующей странице (break 'page)
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

                            WindowEvent::KeyboardReleased(button)=>{
                                match button{
                                    KeyboardButton::Space=>{
                                        if dialogue_box.next_page(){
                                            if page_table.next_page(){
                                                break 'page_inner // Переход к следующей странице (break 'page)
                                            }
                                            else{
                                                break 'gameplay
                                            }
                                        }
                                    }
                                    KeyboardButton::Escape=>{
                                        mouse_cursor.save_position(); // Сохранение текущей позиции мышки
                                        // Пауза
                                        match PauseMenu::new().start(&mut window,&music){
                                            Game::ContinueGamePlay=>{
                                                let (dx,dy)=mouse_cursor.saved_movement();
                                                wallpaper.mouse_shift(dx,dy);
                                                continue 'page
                                            }
                                            Game::MainMenu=>{ // Возвращение в гланое меню
                                                let (dx,dy)=mouse_cursor.saved_movement();
                                                wallpaper.mouse_shift(dx,dy);
                                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                                continue 'game
                                            }
                                            Game::Exit=>{ // Выход из игры
                                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                                break 'game
                                            }
                                            _=>{}
                                        }
                                    }
                                    KeyboardButton::F5=>{
                                        let path=format!("screenshots/screenshot{}.png",Settings.screenshot);
                                        Settings.screenshot+=1;
                                        window.screenshot(path)
                                    },
                                    _=>{}
                                }
                            }
                            _=>{}
                        }
                        // Конец цикла страницы
                    }

                    window.set_smooth(-page_smooth);
                    window.set_alpha(1f32);
                    while let Some(event)=window.next_event(){
                        match event{
                            WindowEvent::Exit=>break 'game, // Закрытие игры

                            WindowEvent::MouseMovementDelta((dx,dy))=>{
                                wallpaper.mouse_shift(dx,dy);
                            }

                            WindowEvent::Draw=>{ //Рендеринг
                                if 0f32>window.draw_smooth(|alpha,c,g|{
                                    g.clear_colour(White);
                                    wallpaper.draw_smooth(alpha,c,g);
                                    characters_view.draw_smooth(alpha,c,g);
                                    dialogue_box.set_alpha_channel(alpha);
                                    dialogue_box.draw_without_text(c,g);
                                }){
                                    break 'page
                                }
                            }

                            WindowEvent::KeyboardReleased(button)=>{
                                if button==KeyboardButton::F5{
                                    make_screenshot(&window)
                                }
                            }
                            _=>{}
                        }
                    }
                }
                // Конец цикла только игровой части
            }
            Settings.continue_game=false; // Отключение "продолжить игру"

            wallpaper.update_image(texture_base.ending_wallpaper()); // Конечная заставка игры

            window.set_new_smooth(default_page_smooth);

            'smooth_ending:while let Some(event)=window.next_event(){
                match event{
                    WindowEvent::Exit=>break 'game, // Закрытие игры

                    WindowEvent::MouseMovementDelta((dx,dy))=>{
                        wallpaper.mouse_shift(dx,dy);
                    }

                    WindowEvent::Draw=>{ //Рендеринг
                        if 1f32<window.draw_smooth(|alpha,c,g|{
                            wallpaper.draw_smooth(alpha,c,g)
                        }){
                            break 'smooth_ending
                        }
                    }

                    WindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(&window)
                        }
                    }

                    _=>{}
                }
            }

            'gameplay_ending:while let Some(event)=window.next_event(){
                match event{
                    WindowEvent::Exit=>break 'game, // Закрытие игры

                    WindowEvent::MouseMovementDelta((dx,dy))=>{
                        wallpaper.mouse_shift(dx,dy);
                    }

                    WindowEvent::Draw=>{ // Рендеринг
                        window.draw(|c,g|{
                            wallpaper.draw(c,g)
                        });
                    }

                    WindowEvent::MouseReleased(_button)=>break 'gameplay_ending,
                    WindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(&window)
                        }
                        break 'gameplay_ending
                    }
                    _=>{}
                }
            }
        }
        // Конец программы
        Settings.save(); // Сохранение настроек игры
    }
}

// Загрузка всех диологов
fn load_dialogues()->Vec<Dialogue>{
    let meta=metadata("./resources/dialogues").unwrap();
    let mut dialogues=Vec::with_capacity(meta.len() as usize);
    let dir=read_dir("./resources/dialogues").unwrap();

    for r in dir{
        let file=r.unwrap();
        let path=file.path();
        let dialogue=Dialogue::new(path);
        dialogues.push(dialogue);
    }
    dialogues
}


pub unsafe fn make_screenshot(window:&GameWindow){
    let path=format!("screenshots/screenshot{}.png",Settings.screenshot);
    Settings.screenshot+=1;
    window.screenshot(path)
}