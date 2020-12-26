#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,unused_imports,dead_code)]
#![cfg_attr(not(debug_assertions),windows_subsystem="windows")]

mod game_settings;
use game_settings::GameSettings;

mod pages;
use pages::{
    LoadingScreen,
    MainMenu
};

use lib::{
    *,
    colours::*,
};

use cat_engine::{
    // statics
    window_center,
    window_width,
    window_height,
    mouse_cursor,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
    // traits
    image::GenericImageView,
    WindowPage,
    // types
    Colour,
    // structs and else
    MouseScrollDelta,
    ModifiersState,
    Window,
    graphics::{
        Graphics,
        DrawType,
        ObjectType,
        DependentObject,
        //ColourFilter
    },
    text::{
        ttf_parser::Face,
        Scale,
        GlyphCache,
        TextBase,
        CachedFont,
        FontOwner
    },
    glium::{
        DrawParameters,
        Blend,
        BlendingFunction,
        LinearBlendingFactor,
        glutin::window::Icon,
        glutin::dpi::Size,
        DrawError,
    },
    texture::{ImageBase,ImageObject,Texture},
    audio::{Audio,AudioSettings,AudioWrapper},
    image::RgbaImage,
};

use std::{
    fs::{metadata,read_dir},
    path::PathBuf,
};

pub enum Game{
    MainMenu,
    Pause,
    Next,
    Exit,
}

// Индексы главных тектстур
const cursor_texture_index:usize=0;

const wallpaper_texture_index:usize=1;

// Алфавит для рендеринга текста (остальные символы будут выведены как неопределённые)
const alphabet:&'static str="АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя1234567890AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZzÕõÄäÖöÜü:();[]!.,";

pub const game_name:&'static str="A Visual Novel by Clomance";

pub const wallpaper_movement_scale:f32=16f32;

pub static mut loading:bool=true; // Флаг загрузки


fn main(){
    //let mut game_settings=GameSettings::load();

    // Коллекция шрифтов
    // let mut fonts=Vec::with_capacity(2);

    // { // Загрузка шрифтов
    //     let main_font_data=FontOwner::load("./resources/fonts/main.font").unwrap();
    //     fonts.push(main_font_data);
    //     let dialogue_font_data=FontOwner::load("./resources/fonts/dialogue.font").unwrap();
    //     fonts.push(dialogue_font_data);
    // }

    // Подключение аудио системы
    let audio=Audio::default(AudioSettings::new()).unwrap();
    let mut audio = AudioWrapper::new(audio);
    audio.load_track("./resources/music/audio.mp3","main_theme".to_string());

    // Настройка и создание окна и загрузка функций OpenGL
    let (mut window,mut graphics)=match Window::new(|mut monitors,window_settings|{
        // Установка полноэкранного режима для нужного экрана
        let monitor=0;//game_settings.monitor;
        let monitor=if monitor<monitors.len(){
            monitors.remove(monitor)
        }
        else{
            //game_settings.monitor=0;
            monitors.remove(0)
        };

        let size=monitor.size();

        let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(Some(monitor));

        let icon=load_window_icon();

        window_settings.general.initial_colour=Some(White);

        window_settings.general.updates_per_second=50;

        window_settings.window_attributes.inner_size=Some(Size::Physical(size));
        window_settings.window_attributes.title=game_name.to_string();
        window_settings.window_attributes.fullscreen=Some(fullscreen);
        window_settings.window_attributes.resizable=false;
        window_settings.window_attributes.decorations=false;
        window_settings.window_attributes.always_on_top=true;
        window_settings.window_attributes.window_icon=Some(icon);


        window_settings.vsync=true;
        window_settings.debug=false;

        window_settings.pixel_fmt_req.srgb=true;
        window_settings.pixel_fmt_req.hardware_accelerated=None;


        window_settings.graphics_base_settings.texture.vertex_buffer_size=20usize;
        window_settings.graphics_base_settings.texture.vertex_buffer_offset=0usize;
        window_settings.graphics_base_settings.texture.object_buffer_size=5usize;


        window_settings.graphics_base_settings.simple.vertex_buffer_size=100usize;
        window_settings.graphics_base_settings.simple.vertex_buffer_offset=0usize;
        window_settings.graphics_base_settings.simple.object_buffer_size=10usize;

        window_settings.graphics_base_settings.text.glyph_texture_size=[512u32;2];
    }){
        Ok(window)=>window,
        Err(e)=>{
            #[cfg(debug_assertions)]
            println!("{:?}",e);
            return
        }
    };
    // Установка видимости курсора
    window.display().gl_window().window().set_cursor_visible(false);

    // Загрузка иконки курсора мыши
    let mut image_base=ImageObject::new(unsafe{[
            window_center[0]-15f32,
            window_center[1]-15f32,
            30f32,
            30f32
        ]},
        [
            0f32,
            0f32,
            1f32,
            1f32,
        ],
        White
    );
    let mouse_texture=Texture::from_path("./resources/images/mouse_icon.png",window.display()).unwrap();
    graphics.add_texture(mouse_texture);
    let mouse_cursor_icon=graphics.add_textured_object(&image_base,0).unwrap();

    // Создание тектуры для обоев
    // Размеры для обоев
    let (dx,dy,width,height)=unsafe{
        let dx=window_width/(wallpaper_movement_scale*2f32);
        let dy=window_height/(wallpaper_movement_scale*2f32);
        let width=(window_width+2f32*dx).ceil();
        let height=(window_height+2f32*dy).ceil();

        (dx,dy,width,height)
    };

    image_base.set_rect([-dx,-dy,width,height]);

    // Создание текстуры чуть больше размера экрана
    let wallpaper_texture=Texture::empty([width as u32,height as u32],window.display()).unwrap();
    graphics.add_texture(wallpaper_texture);
    let wallpaper=graphics.add_textured_object(&image_base,wallpaper_texture_index).unwrap();

    // let scale=Scale::new(0.2f32,0.2f32);
    // for font in fonts{
    //     let glyph_cache=GlyphCache::new_alphabet(font.face(),alphabet,scale,window.display());
    //     let cached_font=CachedFont::raw(font,glyph_cache);
    //     graphics.add_font(cached_font).unwrap();
    // }

    // let loading_resources_thread=move||{
    //     let mut wallpapers=Vec::with_capacity(1);
    //     let menu_wallpaper=load_image("./resources/images/wallpapers/main_menu_wallpaper.png",width as u32,height as u32);
    //     wallpapers.push(menu_wallpaper);
    //     unsafe{loading=false};
    //     wallpapers
    // };

    // Создание и запуск страницы загрузки
    if let Game::Exit=LoadingScreen::new(&window,&mut graphics).run(&mut window,&mut graphics){
        return
    }

    audio.play_track("main_theme",0);

    // Цикл игры
    'game:loop{
        // Главное меню
        match MainMenu::new(&window,&mut graphics).run(&mut window,&mut graphics){
            Game::Exit=>break 'game,
            _=>{}
        }
    }
}

/// Загрузка иконки окна
fn load_window_icon()->Icon{
    let image=cat_engine::image::open("./resources/images/window_icon.png").unwrap();
    let vec=image.to_bytes();
    let (width,height)=image.dimensions();

    Icon::from_rgba(vec,width,height).unwrap()
}

// Загрузка изображений
fn load_image<P:AsRef<std::path::Path>>(path:P,width:u32,height:u32)->RgbaImage{
    let mut image=cat_engine::image::open(path).unwrap();

    image=image.resize_exact(width,height,cat_engine::image::imageops::FilterType::Gaussian);
    if let cat_engine::image::DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba8()
    }
}