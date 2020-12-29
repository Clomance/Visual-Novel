use crate::{
    // statics
    game_settings,
    // consts
    fonts_paths,
    audio_tracks_paths,
    main_menu_wallpaper_path,
    wallpaper_movement_scale,
    mouse_cursor_icon_index,
    // enums
    Game,
    // structs
    LoadingMainData,
    // fns
    load_image,
};

use lib::{
    colours::White,
    loading_flag::{
        ThreadState,
        LoadingFlag,
        LoadingFlagSmartPtr
    }
};

use cat_engine::{
    // statics
    mouse_cursor,
    window_center,
    window_width,
    window_height,
    // enums
    KeyboardButton,
    // structs
    Window,
    WindowEvent,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,Texture},
    text::FontOwner,
    audio::ChanneledTrack,
};

use std::{
    thread::{spawn,JoinHandle},
    sync::mpsc::{
        channel,
        Receiver,
        TryRecvError
    },
};

const loading_screen_assets:usize=2;



pub struct LoadingScreen{
    cat:usize,
    cat_image_base:ImageObject,
    gear:usize,
    thread:Option<JoinHandle<LoadingMainData>>,
    thread_flag:LoadingFlag,
}

impl LoadingScreen{
    pub fn new(window:&Window,graphics:&mut Graphics2D)->LoadingScreen{
        // Создание основы для иконки загрузки
        let cat=Texture::from_path("./resources/images/loading_screen_assets.png",window.display()).unwrap();
        graphics.add_texture(cat);

        // Шестерня
        let mut image_base=ImageObject::raw_uv(unsafe{[
                window_center[0]-200f32,
                window_center[1]-200f32,
                400f32,
                400f32
            ]},
            [
                0f32,
                0f32,
                1f32,
                1f32/4f32,
            ],
            White
        );
        let gear=graphics.add_textured_object(&image_base,loading_screen_assets).unwrap();

        // Кот
        image_base.set_new_raw_uv(unsafe{[
                window_center[0]-100f32,
                window_center[1]-100f32,
                200f32,
                200f32
            ]},
            [
                0f32,
                3f32/4f32,
                1f32,
                1f32,
            ],
            White
        );
        let cat=graphics.add_textured_object(&image_base,loading_screen_assets).unwrap();

        let mut loading_flag=LoadingFlag::new();
        let loading_flag_ptr=loading_flag.ptr();
        let thread=spawn(move||{
            // Для переноса в поток
            let loading_flag=loading_flag_ptr;

            let mut data=LoadingMainData::new();

            // Загрузка шрифтов
            for path in fonts_paths{
                if let ThreadState::Finished=loading_flag.get_state(){
                    return data
                }
                let font=FontOwner::load(path).unwrap();
                data.fonts.push(font);
            }

            // Загрузка аудио
            for path in audio_tracks_paths{
                if let ThreadState::Finished=loading_flag.get_state(){
                    return data
                }
                let audio=ChanneledTrack::new(path).unwrap();
                data.audio.push(audio);
            }

            if let ThreadState::Finished=loading_flag.get_state(){
                return data
            }
            // Размеры изображения для обоев
            let (width,height)=unsafe{
                let dx=window_width/(wallpaper_movement_scale*2f32);
                let dy=window_height/(wallpaper_movement_scale*2f32);
                let width=(window_width+2f32*dx).ceil();
                let height=(window_height+2f32*dy).ceil();

                (width as u32,height as u32)
            };
            // Загрузка обоев главного меню
            let main_menu_wallpaper=load_image(main_menu_wallpaper_path,Some([width,height]));
            data.textures.push(main_menu_wallpaper);

            // Передача данных
            data
        });

        Self{
            cat,
            cat_image_base:image_base,
            gear,
            thread:Some(thread),
            thread_flag:loading_flag,
        }
    }

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D,data:&mut LoadingMainData)->Game{
        let mut frames=0u8;
        let mut angle=0f32;

        let mut result=Game::Next;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>{
                    self.thread_flag.set_state(ThreadState::Finished);
                    if let Some(thread)=self.thread.take(){
                        *data=thread.join().expect("Ошибка начальной загрузки");
                        window.stop_events();
                    }
                    result=Game::Exit;
                }

                WindowEvent::Update=>{
                    match self.thread_flag.get_state(){
                        ThreadState::Running=>{
                            // 0.35 секунды глаза открыты, 0.15 секунды закрыты
                            if frames==35{
                                // cat_eyes_half_closed
                                self.cat_image_base.set_raw_uv([0f32,2f32/4f32,1f32,3f32/4f32]);
                                graphics.rewrite_textured_object_vertices(self.cat,&self.cat_image_base.vertices());
                            }
                            else if frames==40{
                                // cat_eyes_closed
                                self.cat_image_base.set_raw_uv([0f32,1f32/4f32,1f32,2f32/4f32]);
                                graphics.rewrite_textured_object_vertices(self.cat,&self.cat_image_base.vertices());
                            }
                            else if frames==45{
                                // cat_eyes_half_closed
                                self.cat_image_base.set_raw_uv([0f32,2f32/4f32,1f32,3f32/4f32]);
                                graphics.rewrite_textured_object_vertices(self.cat,&self.cat_image_base.vertices());
                            }
                            else if frames==50{
                                // cat
                                self.cat_image_base.set_raw_uv([0f32,3f32/4f32,1f32,1f32]);
                                graphics.rewrite_textured_object_vertices(self.cat,&self.cat_image_base.vertices());
                                frames=0;
                            }
                            frames+=1;

                            // 140 градусов в секунду
                            angle+=0.05;
                        }

                        ThreadState::Finished=>
                            if let Some(thread)=self.thread.take(){
                                *data=thread.join().unwrap();
                                window.stop_events();
                            }

                        ThreadState::Panicked=>
                            if let Some(thread)=self.thread.take(){
                                thread.join();
                                window.stop_events();
                                result=Game::Exit;
                            }
                    }
                }
                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                    window.draw(graphics,|graphics|{
                        graphics.clear_colour(White);
                        // Рендеринг кота
                        graphics.draw_textured_object(self.cat).unwrap();
                        // Рендеринг шестерни
                        graphics.draw_rotate_textured_object(self.gear,unsafe{window_center},angle).unwrap();
                        // Рендеринг курсора
                        graphics.draw_shift_textured_object(mouse_cursor_icon_index,[dx,dy]).unwrap();
                    }).unwrap();
                }

                WindowEvent::KeyboardPressed(KeyboardButton::F5)=>unsafe{
                    let path=format!("screenshots/screenshot{}.png",game_settings.screenshot);
                    game_settings.screenshot+=1;
                    window.save_screenshot(path);
                }

                _=>{

                }
            }
        });

        // Кот
        graphics.remove_last_textured_object();
        // Шестерня
        graphics.remove_last_textured_object();
        // loading_screen_asset
        graphics.remove_last_texture();

        result
    }
}