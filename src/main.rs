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
    texture::{
        ImageBase,
        ImageObject,
        Texture},
    audio::{
        Audio,
        AudioSettings,
        AudioWrapper,
        ChanneledTrack,
    },
    image::{
        DynamicImage,
        RgbaImage
    },
};

use std::{
    fs::{metadata,read_dir},
    path::{PathBuf,Path},
};

pub enum Game{
    MainMenu,
    Pause,
    Next,
    Exit,
}

// Индексы главных текстурных объектов
const mouse_cursor_icon_index:usize=0;

const wallpaper_index:usize=1;

// Пути ресурсов
const audio_tracks_paths:&[&'static str]=&[
    "./resources/audio/audio.mp3",
];

const fonts_paths:&[&'static str]=&[
    "./resources/fonts/main.font",
    "./resources/fonts/dialogue.font",
];

const main_menu_wallpaper_path:&'static str="./resources/images/wallpapers/main_menu_wallpaper.png";

const decoration_image_paths:&[&'static str]=&[
    "resources/images/rose.png",
];

// Названия для аудио треков
const audio_tracks_names:&[&'static str]=&[
    "main_theme",
];


// Алфавит для рендеринга текста (остальные символы будут выведены как неопределённые)
const alphabet:&'static str="АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя1234567890AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz:();[]!.,";

pub const game_name:&'static str="A Visual Novel by Clomance";

pub const wallpaper_movement_scale:f32=16f32;

pub static mut game_settings:GameSettings=GameSettings::new();

fn main(){
    // unsafe{
    //     game_settings=GameSettings::load();
    // }

    // Подключение аудио системы
    let audio=Audio::default(AudioSettings::new()).unwrap();
    let mut audio=AudioWrapper::new(audio);

    // Настройка и создание окна и загрузка функций OpenGL
    let (mut window,mut graphics)=match Window::new(|mut monitors,window_settings|{
        // Установка полноэкранного режима для нужного экрана
        let monitor=unsafe{game_settings.monitor};
        let monitor=if monitor<monitors.len(){
            monitors.remove(monitor)
        }
        else{
            //game_settings.monitor=0;
            monitors.remove(0)
        };

        //let size=monitor.size();

        let fullscreen=cat_engine::glium::glutin::window::Fullscreen::Borderless(Some(monitor));

        let icon=load_window_icon();

        window_settings.general.initial_colour=Some(White);

        window_settings.general.updates_per_second=50;

        //window_settings.window_attributes.inner_size=Some(Size::Physical(size));
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
    { // Загрузка иконки курсора мыши
        
        let mouse_texture=Texture::from_path("./resources/images/mouse_icon.png",window.display()).unwrap();
        let mouse_texture_index=graphics.add_texture(mouse_texture);
        let _mouse_cursor_icon=graphics.add_textured_object(&image_base,mouse_texture_index).unwrap();
    }
    { // Создание текстуры чуть больше размера экрана
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

        let wallpaper_texture=Texture::empty([width as u32,height as u32],window.display()).unwrap();
        let wallpaper_texture_index=graphics.add_texture(wallpaper_texture);
        let _wallpaper=graphics.add_textured_object(&image_base,wallpaper_texture_index).unwrap();
    }

    // Данные для начальной загрузки
    let mut main_data=LoadingMainData::new();

    // Создание и запуск страницы загрузки
    if let Game::Exit=LoadingScreen::new(&window,&mut graphics).run(&mut window,&mut graphics,&mut main_data){
        return
    }

    // Кэширование шрифтов
    let scale=Scale::new(0.1f32,0.1f32);
    for font in main_data.fonts{
        let glyph_cache=GlyphCache::new_alphabet(font.face(),alphabet,scale,window.display());
        let cached_font=CachedFont::raw(font,glyph_cache);
        graphics.add_font(cached_font);
    }

    // Загрузка треков в хранилище
    for (track,name) in main_data.audio.into_iter().zip(audio_tracks_names.iter()){
        audio.push_track(track,name.to_string());
    }

    // Запуск мелодии главной темы (повторять бесконечно)
    audio.play_track("main_theme",0u32);

    let images=main_data.textures;

    // Цикл игры
    'game:loop{
        // Главное меню
        match MainMenu::new(&window,&mut graphics,&images[0..2]).run(&mut window,&mut graphics){
            Game::Exit=>break 'game,
            _=>{}
        }
    }
}

/// Данные при начальной загрузке.
pub struct LoadingMainData{
    pub fonts:Vec<FontOwner>,
    pub audio:Vec<ChanneledTrack>,
    pub textures:Vec<RgbaImage>,
}

impl LoadingMainData{
    pub fn new()->LoadingMainData{
        Self{
            fonts:Vec::new(),
            audio:Vec::new(),
            textures:Vec::new(),
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
fn load_image<P:AsRef<Path>>(path:P,size:Option<[u32;2]>)->RgbaImage{
    let mut image=cat_engine::image::open(path).unwrap();

    if let Some([width,height])=size{
        image=image.resize_exact(width,height,cat_engine::image::imageops::FilterType::Gaussian);
    }

    if let cat_engine::image::DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba8()
    }
}

// Загрузка фонов
// fn load_wallpapers_textures_paths<P:AsRef<Path>+Clone>(path:P)->Vec<PathBuf>{
//     let meta=metadata(path.clone()).unwrap();
//     let mut textures=Vec::with_capacity(meta.len() as usize);
//     let dir=read_dir(path).unwrap();

//     for r in dir{
//         let file=r.unwrap();
//         let path=file.path();
//         textures.push(path)
//     }

//     textures
// }

// fn load_characters_textures(height:f32)->Vec<RgbaImage>{
//     let path="./resources/images/characters";
//     let meta=metadata(path).unwrap();

//     let mut char_textures=Vec::with_capacity(meta.len() as usize);

//     let dir=read_dir(path).unwrap();

//     for r in dir{
//         let file=r.unwrap();
//         let _name=file.file_name();
//         let path=file.path();
//         let image=load_character_image(path,height);
//         char_textures.push(image)
//     }

//     char_textures
// }

// Загрузка изображений
// fn load_character_image<P:AsRef<Path>>(path:P,height:f32)->RgbaImage{
//     let mut image=image::open(path).unwrap();
//     let image_height=image.height() as f32;
//     let image_width=image.width() as f32;

//     let width=image_width*height/image_height;

//     image=image.resize_exact(width as u32,height as u32,FilterType::Gaussian);
//     if let DynamicImage::ImageRgba8(image)=image{
//         image
//     }
//     else{
//         image.into_rgba()
//     }
// }