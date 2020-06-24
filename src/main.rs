#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use)]
#![cfg_attr(not(debug_assertions),windows_subsystem="windows")]

use lib::{
    *,
    colours::*,
};

use cat_engine::{
    // fns
    window_rect,
    // traits
    image::image::GenericImageView,
    // statics
    window_width,
    window_height,
    mouse_cursor,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
    // structs
    Window,
    graphics::{Rectangle,Graphics},
    text::Glyphs,
    glium::{
        DrawParameters,
        glutin::window::Icon,
        glutin::dpi::Size
    },
    // mods
    audio::{Audio,AudioSettings}
};

use std::{
    fs::{metadata,read_dir},
    path::PathBuf,
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

// Макросы для удобной и понятной замены ссылки на шрифт
#[macro_export]
macro_rules! Main_font {
    () => {
        #[allow(unused_unsafe)]
        unsafe{
            &crate::glyph_cache[0]
        }
    };
}

#[macro_export]
macro_rules! Dialogue_font {
    () => {
        #[allow(unused_unsafe)]
        unsafe{
            &crate::glyph_cache[1]
        }
    };
}


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

pub const game_name:&'static str="Visual Novel by Clomance";

const page_smooth:f32=1f32/32f32;



pub static mut Settings:game_settings::GameSettings=game_settings::GameSettings::new();

pub static mut loading:bool=true; // Флаг загрузки

pub static mut glyph_cache:Vec<Glyphs>=Vec::new(); // Шрифты
static mut _textures:Textures=Textures::new(); // Хранилище тектур и свазанное с ними
static mut _dialogues:Vec<Dialogue>=Vec::new();

fn main(){
    unsafe{
        let mut glyphs=Glyphs::load("./resources/fonts/main.font");
        glyph_cache.push(glyphs);

        glyphs=Glyphs::load("./resources/fonts/dialogue.font");
        glyph_cache.push(glyphs);

        Settings.load(); // Загрузка настроек
    }

    // Настройка и создание окна и загрузка функций OpenGL
    let mut window:Window=match Window::new(|mut monitors,window_settings|{
        let mut path=PathBuf::new();
        path.push("./resources/images/mouse_icon.png");
        window_settings.mouse_cursor_icon_path=path;

        // Установка полноэкранного режима для нужного экрана
        let monitor=unsafe{Settings.monitor};
        let monitor=if monitor<monitors.len(){
            monitors.remove(monitor)
        }
        else{
            unsafe{Settings.monitor=0}
            monitors.remove(0)
        };

        let size=monitor.size();

        let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(monitor);

        let icon=load_window_icon();

        window_settings.initial_colour=Some(Black);

        window_settings.inner_size=Some(Size::Physical(size));
        window_settings.title=game_name.to_string();
        window_settings.fullscreen=Some(fullscreen);
        window_settings.resizable=false;
        window_settings.decorations=false;
        window_settings.always_on_top=true;
        window_settings.window_icon=Some(icon);

        window_settings.vsync=true;
        window_settings.debug=false;

        window_settings.srgb=true;
        window_settings.hardware_accelerated=None;

        window_settings.texture_vertex_buffer_size=12usize;
        window_settings.simple_vertex_buffer_size=100usize;
        window_settings.text_vertex_buffer_size=2000usize;
    }){
        Ok(window)=>window,
        Err(e)=>{
            #[cfg(debug_assertions)]
            println!("{:?}",e);
            return
        }
    };

    let mut audio_settings=AudioSettings::new();
    unsafe{audio_settings.volume=Settings.volume};
    let music=Audio::new(audio_settings).unwrap();

    unsafe{
        let wallpaper_size={
            let dx=window_width/(wallpaper_movement_scale*2f32);
            let dy=window_height/(wallpaper_movement_scale*2f32);
            [
                (window_width+2f32*dx),
                (window_height+2f32*dy)
            ]
        };

        // Замыкание для допольнительного потока
        let loading_resources_thread=move||{

            _textures=Textures::load(); // Загрузка текстур
            if !loading{return}

            // Загрузка диалогов
            let meta=match metadata("./resources/dialogues"){
                Ok(meta)=>meta,
                Err(_)=>{
                    loading=false;
                    return
                },
            };

            let mut dialogues=Vec::with_capacity(meta.len() as usize);
            let dir=match read_dir("./resources/dialogues"){
                Ok(dir)=>dir,
                Err(_)=>{
                    loading=false;
                    return
                },
            };

            for r in dir{
                if !loading{
                    return // Если загрузка прервана
                }

                let file=match r{
                    Ok(f)=>f,
                    Err(_)=>{
                        loading=false;
                        return
                    },
                };
                let path=file.path();
                let dialogue=Dialogue::new(path);
                dialogues.push(dialogue);
            }
            _dialogues=dialogues;

            loading=false;
        };

        // Экран загрузки
        match LoadingScreen::new(&mut window).start(&mut window,loading_resources_thread){
            Game::Exit=>{
                return
            },
            _=>{}
        }

        let texture_base=&_textures; // "Безопасная" ссылка на Хранилище текстур
        let dialogues=&_dialogues; // "Безопасная" ссылка на диалоги

        let mut wallpaper=Wallpaper::new(texture_base.main_menu_wallpaper(),&mut window);
        let mut characters_view=CharactersView::new(); // "Сцена" для персонажей

        let mut dialogue_box=DialogueBox::new(texture_base.dialogue_box(),window.display(),Dialogue_font!()); // Диалоговое окно

        music.add_track("./resources/music/audio.mp3");
        music.play_forever(0);
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

                    if Intro::new().start(&mut window)==Game::Exit{
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

                wallpaper.update_image_path(wallpaper_path,wallpaper_size); // Установка текущего фона игры

                dialogue_box.set_dialogue(page_table.current_dialogue()); // Установка текущего диалога

                'page:loop{
                    window.set_new_smooth(page_smooth);
                    // Сглаживание перехода
                    'opening_page:while let Some(event)=window.next_event(){
                        match event{
                            WindowEvent::Exit=>break 'game, // Закрытие игры

                            WindowEvent::MouseMovementDelta(_)=>{
                                wallpaper.mouse_shift(mouse_cursor.center_radius());
                            }

                            WindowEvent::Draw=>{ //Рендеринг
                                if 1f32<window.draw_smooth(|alpha,c,g|{
                                    g.clear_colour(White);
                                    wallpaper.draw_shift_smooth(alpha,c,g);
                                    characters_view.draw_smooth(alpha,c,g);
                                    dialogue_box.set_alpha_channel(alpha);
                                    dialogue_box.draw(c,g);
                                }){
                                    break 'opening_page
                                }
                            }

                            WindowEvent::KeyboardReleased(button)=>{
                                if button==KeyboardButton::F5{
                                    make_screenshot(&mut window,|p,g|{
                                        g.clear_colour(White);
                                        wallpaper.draw(p,g);
                                        characters_view.draw(p,g);
                                        dialogue_box.draw(p,g);
                                    })
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

                            WindowEvent::MouseMovementDelta(_)=>{
                                wallpaper.mouse_shift(mouse_cursor.center_radius());
                            }

                            WindowEvent::Draw=>{ //Рендеринг
                                window.draw(|c,g|{
                                    wallpaper.draw_shift(c,g);
                                    characters_view.draw(c,g);
                                    dialogue_box.draw(c,g);
                                });
                            }

                            WindowEvent::MouseReleased(button)=>match button{
                                MouseButton::Left=>{
                                    if dialogue_box.next_page(){
                                        if page_table.next_page(){
                                            break 'page_inner // Переход к следующей странице
                                        }
                                        else{
                                            break 'gameplay
                                        }
                                    }
                                }
                                _=>{}
                            }

                            WindowEvent::KeyboardReleased(button)=>match button{
                                KeyboardButton::Space=>{
                                    if dialogue_box.next_page(){
                                        if page_table.next_page(){
                                            break 'page_inner // Переход к следующей странице
                                        }
                                        else{
                                            break 'gameplay
                                        }
                                    }
                                }

                                KeyboardButton::Escape=>{
                                    // Пауза
                                    match PauseMenu::new().start(&mut window,&music){
                                        Game::ContinueGamePlay=>{
                                            wallpaper.mouse_shift(mouse_cursor.center_radius());
                                            continue 'page
                                        }
                                        Game::MainMenu=>{ // Возвращение в гланое меню
                                            wallpaper.mouse_shift(mouse_cursor.center_radius());
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
                                    make_screenshot(&mut window,|p,g|{
                                        wallpaper.draw_shift(p,g);
                                        characters_view.draw(p,g);
                                        dialogue_box.draw(p,g);
                                    })
                                }
                                _=>{}
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

                            WindowEvent::MouseMovementDelta(_)=>wallpaper.mouse_shift(mouse_cursor.center_radius()),

                            WindowEvent::Draw=>{ //Рендеринг
                                if 0f32>window.draw_smooth(|alpha,p,g|{
                                    g.clear_colour(White);
                                    wallpaper.draw_smooth(alpha,p,g);
                                    characters_view.draw_smooth(alpha,p,g);
                                    dialogue_box.set_alpha_channel(alpha);
                                    dialogue_box.draw_without_text(p,g);
                                }){
                                    break 'page
                                }
                            }

                            WindowEvent::KeyboardReleased(button)=>{
                                if button==KeyboardButton::F5{
                                    make_screenshot(&mut window,|p,g|{
                                        g.clear_colour(White);
                                        wallpaper.draw_shift(p,g);
                                        characters_view.draw(p,g);
                                        dialogue_box.draw_without_text(p,g);
                                    })
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

                    WindowEvent::Draw=>{ // Рендеринг
                        if 1f32<window.draw_smooth(|alpha,p,g|{
                            wallpaper.draw_smooth(alpha,p,g);
                        }){
                            break 'smooth_ending
                        }
                    }

                    WindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(&mut window,|p,g|{wallpaper.draw(p,g)})
                        }
                    }

                    _=>{}
                }
            }

            'gameplay_ending:while let Some(event)=window.next_event(){
                match event{
                    WindowEvent::Exit=>break 'game, // Закрытие игры

                    // Рендеринг
                    WindowEvent::Draw=>window.draw(|p,g|{
                        wallpaper.draw(p,g)
                    }),

                    WindowEvent::MouseReleased(_button)=>break 'gameplay_ending,
                    WindowEvent::KeyboardReleased(button)=>{
                        if button==KeyboardButton::F5{
                            make_screenshot(&mut window,|p,g|{wallpaper.draw(p,g)})
                        }
                        else{
                            break 'gameplay_ending
                        }
                    }
                    _=>{}
                }
            }
        }
        // Конец программы
        Settings.save(); // Сохранение настроек игры
    }
}

pub fn make_screenshot<F:FnOnce(&mut DrawParameters,&mut Graphics)>(window:&mut Window,f:F){
    let rect=Rectangle::new(window_rect(),[1f32,1f32,1f32,0.8f32]);

    window.set_cursor_visible(false); // Отключение курсора

    window.draw_event_once(f); // Отрисовка кадра для скриншота

    unsafe{
        let path=format!("screenshots/screenshot{}.png",Settings.screenshot);
        Settings.screenshot+=1;
        window.save_screenshot(path)
    }

    window.set_cursor_visible(true);

    window.draw_event_once(|d,g|{
        rect.draw(d,g);
    });
}

/// Загрузка иконки окна
fn load_window_icon()->Icon{
    let image=cat_engine::image::image::open("./resources/images/window_icon.png").unwrap();
    let vec=image.to_bytes();
    let (width,height)=image.dimensions();

    Icon::from_rgba(vec,width,height).unwrap()
}