#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,dead_code)]
//#![windows_subsystem="windows"]

use engine::{
    // statics
    window_width,
    window_height,
    window_center,
    mouse_cursor,
    // structs
    GameWindow,
    // enums
    GameWindowEvent,
    MouseButton,
    KeyboardButton,
    image::RgbaImage,
    music
};

use std::{
    fmt::Debug,
    path::{Path,PathBuf},
    fs::{File,OpenOptions,metadata,read_dir},
    io::{Read,Write,BufReader,BufRead},
};

use lib::*;

mod game_settings;

mod pages;
use pages::*;

mod page_table;
use page_table::PageTable;

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

#[derive(Copy,Clone,Hash,PartialEq,Eq)]
enum Melody{
    None,
    MainMenu
}

#[derive(Copy,Clone,Hash,PartialEq,Eq)]
enum Sound{
    None
}

fn main(){
    make_page_table_file(); // Создание таблицы ресурсов, если требуется

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
                            GameWindowEvent::Exit=>break 'game, // Закрытие игры

                            GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                                wallpaper.mouse_shift(dx,dy);
                            }

                            GameWindowEvent::Draw=>{ //Рендеринг
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

                            GameWindowEvent::KeyboardReleased(button)=>{
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
                            GameWindowEvent::Exit=>{ // Закрытие игры
                                Settings.set_saved_position(page_table.current_page(),dialogue_box.current_step()); // Сохранение последней позиции
                                break 'game
                            }

                            GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                                wallpaper.mouse_shift(dx,dy);
                            }

                            GameWindowEvent::Draw=>{ //Рендеринг
                                window.draw(|c,g|{
                                    wallpaper.draw(c,g);
                                    characters_view.draw(c,g);
                                    dialogue_box.draw(c,g);
                                });
                            }

                            GameWindowEvent::MouseReleased(button)=>{
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

                            GameWindowEvent::KeyboardReleased(button)=>{
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
                            GameWindowEvent::Exit=>break 'game, // Закрытие игры

                            GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                                wallpaper.mouse_shift(dx,dy);
                            }

                            GameWindowEvent::Draw=>{ //Рендеринг
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

                            GameWindowEvent::KeyboardReleased(button)=>{
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
                    GameWindowEvent::Exit=>break 'game, // Закрытие игры

                    GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                        wallpaper.mouse_shift(dx,dy);
                    }

                    GameWindowEvent::Draw=>{ //Рендеринг
                        if 1f32<window.draw_smooth(|alpha,c,g|{
                            wallpaper.draw_smooth(alpha,c,g)
                        }){
                            break 'smooth_ending
                        }
                    }

                    GameWindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(&window)
                        }
                    }

                    _=>{}
                }
            }

            'gameplay_ending:while let Some(event)=window.next_event(){
                match event{
                    GameWindowEvent::Exit=>break 'game, // Закрытие игры

                    GameWindowEvent::MouseMovementDelta((dx,dy))=>{
                        wallpaper.mouse_shift(dx,dy);
                    }

                    GameWindowEvent::Draw=>{ // Рендеринг
                        window.draw(|c,g|{
                            wallpaper.draw(c,g)
                        });
                    }

                    GameWindowEvent::MouseReleased(_button)=>break 'gameplay_ending,
                    GameWindowEvent::KeyboardReleased(button)=>{
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

// Перенесу в build.rs, когда буду делать полноценную игру
// Создаёт файл с таблицей распределения ресурсов
// Выполняет проверку изменений в файлах и при надобности изменяет таблицу
const paths:&[&str]=&[
    "./resources/images/characters",
    "./resources/images/wallpapers",
    "./resources/dialogues",
    "./settings/page_table.txt"
];

// Формат файла
/*
номер фона, номер диалога, количество персонажей u8, номера персонажей...
*/

fn make_page_table_file(){
    let mut changed=false; // Флаг изменения в папке ресурсов

    let mut page_table_file=OpenOptions::new().write(true).open("./settings/page_table").unwrap();

    { // Поиск изменений по ресурсам
        let flag_last_changed=page_table_file.metadata().unwrap().modified().unwrap();

        for path in paths{
            let meta=metadata(path).unwrap();
            let last_changed=meta.modified().unwrap();
            if last_changed>flag_last_changed{
                changed=true;
                break
            }
        }

        let dia_dir=read_dir("./resources/dialogues").unwrap();
        for dialogue in dia_dir{
            let last_changed=dialogue.unwrap().metadata().unwrap().modified().unwrap();
            if last_changed>flag_last_changed{
                changed=true;
                break
            }
        }
    }

    if changed{
        // Поиск текстур персонажей и сохранение названий файлов
        let char_meta=metadata("./resources/images/characters").unwrap();
        let mut char_names=Vec::with_capacity(char_meta.len() as usize); // Имена персонажей

        let char_dir=read_dir("./resources/images/characters").unwrap();
        for character in char_dir{
            let file=character.unwrap();
            let file_name=file.file_name().into_string().unwrap();

            // Удаление .png из имени файла
            let len=file_name.len();
            let name=file_name[..len-4].to_string();

            char_names.push(name);
        }

        // Поиск текстур обоев и сохранение названий файлов
        let wall_meta=metadata("./resources/images/wallpapers/game").unwrap();
        let mut wall_names=Vec::with_capacity(wall_meta.len() as usize); // Названия фонов

        let wall_dir=read_dir("./resources/images/wallpapers/game").unwrap();
        for wall in wall_dir{
            let file=wall.unwrap();
            let file_name=file.file_name().into_string().unwrap();

            // Удаление .png из имени файла
            let len=file_name.len();
            let name=file_name[..len-4].to_string();

            wall_names.push(name);
        }

        // Поиск диалогов и загрузка их заголовков
        let dia_meta=metadata("./resources/dialogues").unwrap();
        let mut dia_names=Vec::with_capacity(dia_meta.len() as usize); // Названия диалогов
        let mut dialogues_characters=Vec::with_capacity(dia_meta.len() as usize); // Имена персонажей в диалогах

        let dia_dir=read_dir("./resources/dialogues").unwrap();
        for dialogue in dia_dir{
            let file=dialogue.unwrap();
            let path=file.path();

            // Получение названия диалога
            let file_name=file.file_name().into_string().unwrap();

            // Удаление .txt из имени файла
            let len=file_name.len();
            let name=file_name[..len-4].to_string();

            dia_names.push(name);

            let dialogue_file=OpenOptions::new().read(true).open(path).unwrap();
            let characters=dialogue::read_characters(&mut BufReader::new(dialogue_file));

            // Сопоставление имён персонажей в диалогах и текстур персонажей
            let mut dialogue_characters=Vec::new();
            for (ch,location) in characters{
                let index=search(&char_names,&ch);
                dialogue_characters.push((index,location));
            }
            dialogues_characters.push(dialogue_characters);
        }

        let table_file=OpenOptions::new().read(true).open("settings/page_table.txt").unwrap();

        let mut reader=BufReader::new(table_file);
        let mut line=String::new();
        let mut line_str;

        while let Ok(bytes)=reader.read_line(&mut line){
            if bytes==0{
                break // Конец файла
            }

            line_str=line.trim();
            if line_str.is_empty(){
                continue // Пропуск пустой строки
            }

            // Проверка на начало блока страницы
            if let Some(_)=line_str.find("{"){
                let (wallpaper,dialogue)=load_page_settings(&mut reader);

                let wallpaper=search(&wall_names,&wallpaper); // Поиск фона по названию5

                page_table_file.write(&wallpaper.to_be_bytes()).unwrap();
                
                let dialogue=search(&dia_names,&dialogue);
                page_table_file.write(&dialogue.to_be_bytes()).unwrap();

                let len=dialogues_characters[dialogue].len() as u8;
                page_table_file.write(&[len]).unwrap();
                for (ch,location) in &dialogues_characters[dialogue]{
                    page_table_file.write(&ch.to_be_bytes()).unwrap();
                    page_table_file.write(&[location.clone() as u8]).unwrap();
                }
            }
            line.clear();
        }
    }
}

fn search(vec:&Vec<String>,v:&str)->usize{
    for (c,name) in vec.iter().enumerate(){
        if v==name{
            return c
        }
    }
    panic!("search");
}

// (wallpaper, dialogue)
fn load_page_settings(reader:&mut BufReader<File>)->(String,String){
    let mut wallpaper=None;
    let mut dialogue=None;

    let mut line=String::new();
    let mut line_str;

    while let Ok(bytes)=reader.read_line(&mut line){
        line_str=line.trim();
        if line_str=="}" || bytes==0{
            break
        }

        let split_line:Vec<&str>=line.split("=").map(|s|s.trim()).collect();

        // Проверка форматирования
        if split_line.len()!=2{
            panic!("LoadingPageTableError");
        }
        match split_line[0]{
            "wallpaper"=>wallpaper=Some(split_line[1].to_string()),
            "dialogue"=>dialogue=Some(split_line[1].to_string()),
            _=>panic!("LoadingPageTableError: no such field"),
        }

        line.clear();
    }

    (wallpaper.unwrap(),dialogue.unwrap())
}


pub unsafe fn make_screenshot(window:&GameWindow){
    let path=format!("screenshots/screenshot{}.png",Settings.screenshot);
    Settings.screenshot+=1;
    window.screenshot(path)
}