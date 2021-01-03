use crate::{
    // statics
    game_settings,
    // consts
    alphabet,
    fonts_paths,
    audio_tracks_paths,
    main_menu_wallpaper_path,
    decoration_image_paths,
    wallpaper_movement_scale,
    mouse_cursor_icon_index,
    swipe_screen_index,
    // enums
    Game,
    // structs
    LoadingMainData,
    // fns
    load_image,
    draw_on_texture,
    get_swipe_texture,
    make_screenshot,
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
    // functions
    default_draw_parameters,
    // enums
    KeyboardButton,
    // structs
    Window,
    WindowEvent,
    graphics::{
        Graphics,
        Graphics2D,
        DependentObject
    },
    texture::{ImageObject,Texture},
    text::{Scale,FontOwner,GlyphCache,CachedFont},
    audio::{ChanneledTrack,AudioWrapper},

    glium::{
        Surface,
        framebuffer::SimpleFrameBuffer
    },
};

use std::{
    thread::{spawn,JoinHandle},
    sync::mpsc::{
        channel,
        Receiver,
        TryRecvError
    },
};

const chars_chached_per_update:u8=10u8;

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
        let loading_screen_assets=Texture::from_path("./resources/images/loading_screen_assets.png",window.display()).unwrap();
        let loading_screen_assets=graphics.add_texture(loading_screen_assets);

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

            let mut fonts=Vec::new();
            // Загрузка шрифтов
            for path in fonts_paths{
                if let ThreadState::Finished=loading_flag.get_state(){
                    return data
                }
                let font=FontOwner::load(path).unwrap();
                fonts.push(font);
            }

            data.fonts=Some(fonts);

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

            for path in decoration_image_paths{
                let image=load_image(path,None);
                data.textures.push(image);
            }

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

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D,audio:&AudioWrapper,data:&mut LoadingMainData)->Game{
        let mut frames=0u8;
        let mut angle=0f32;

        let mut result=Game::Next;

        // Кэширование шрифтов
        let mut caching_fonts=false;

        let mut alphabet_iter=alphabet.chars();

        let mut current_font:Option<FontOwner>=None;
        let mut font_iter:Option<std::vec::IntoIter<cat_engine::text::FontOwner>>=None;

        let scale=Scale::new(0.1f32,0.1f32);

        let mut glyph_cache:Option<GlyphCache>=None;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>{
                    self.thread_flag.set_state(ThreadState::Finished);
                    if let Some(thread)=self.thread.take(){
                        thread.join().expect("Ошибка начальной загрузки");
                    }
                    result=Game::Exit;
                }

                WindowEvent::Update=>{
                    // Анимация загрузки
                    frames+=1;
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

                    // 140 градусов в секунду
                    angle+=0.05;

                    // Кэширование шрифтов
                    if caching_fonts{
                        if let Some(font)=&current_font{
                            let mut chars_passed=0;
                            while chars_passed<chars_chached_per_update{
                                if let Some(character)=alphabet_iter.next(){
                                    if let Some(glyph_cache)=&mut glyph_cache{
                                        glyph_cache.insert_char(character,font.face(),scale,window.display());
                                    }
                                    chars_passed+=1;
                                }
                                else{
                                    alphabet_iter=alphabet.chars();

                                    let font=current_font.take().unwrap();

                                    let chached_font=CachedFont::raw(font,glyph_cache.take().unwrap());
                                    graphics.add_font(chached_font);

                                    current_font=font_iter.as_mut().unwrap().next();

                                    if let Some(font)=&current_font{
                                        glyph_cache=Some(GlyphCache::new_alphabet(font.face(),"",scale,window.display()));
                                    }
                                    else{
                                        window.stop_events();
                                    }
                                    break
                                }
                            }
                        }
                    }
                    else{ // Ожидание загрузки нужных ресурсов
                        match self.thread_flag.get_state(){
                            ThreadState::Running=>{}

                            // Завершение загрузки - начало кэширования шрифтов
                            ThreadState::Finished=>
                                if let Some(thread)=self.thread.take(){
                                    *data=thread.join().unwrap();

                                    let mut fonts=data.fonts.take().unwrap().into_iter();
                                    current_font=fonts.next();
                                    font_iter=Some(fonts);

                                    glyph_cache=Some(GlyphCache::new_alphabet(current_font.as_ref().unwrap().face(),"",scale,window.display()));

                                    caching_fonts=true;
                                }

                            // Ошибка загрузки - закрытие игры
                            ThreadState::Panicked=>
                                if let Some(thread)=self.thread.take(){
                                    thread.join();
                                    window.stop_events();
                                    result=Game::Exit;
                                }
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

                WindowEvent::KeyboardPressed(KeyboardButton::F5)=>make_screenshot(window,audio),

                _=>{}
            }
        });

        let swipe_screen_texture=get_swipe_texture(graphics);

        draw_on_texture(&swipe_screen_texture,window,graphics,|graphics|{
            graphics.clear_colour(White);
            // Рендеринг кота
            graphics.draw_textured_object(self.cat).unwrap();
            // Рендеринг шестерни
            graphics.draw_rotate_textured_object(self.gear,unsafe{window_center},angle).unwrap();
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