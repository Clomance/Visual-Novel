#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,unused_imports,dead_code)]
#![cfg_attr(not(debug_assertions),windows_subsystem="windows")]

mod game_settings;
use game_settings::GameSettings;

mod pages;
use pages::*;

mod wallpaper;
use wallpaper::Wallpaper;


use lib::{
    ObjectMap,
    DrawableObject,
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
    Window,
    WindowPage,
    // types
    Colour,
    // structs and else
    MouseScrollDelta,
    ModifiersState,
    DefaultWindow,
    PagedWindow,
    graphics::{
        Graphics,
        DrawType,
        ObjectType,
        ColourFilter
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
    texture::{ImageBase,Texture},
    audio::{Audio,AudioSettings},
    image::RgbaImage,
};

use std::{
    fs::{metadata,read_dir},
    path::PathBuf,
};


// mod page_table;
// use page_table::*;

// mod characters;
// use characters::*;

// mod dialogue;
// use dialogue::*;

// mod textures;
// use textures::Textures;

// mod dialogue_box;
// pub use dialogue_box::DialogueBox;


// Т.к. добавляется в массив объектов самым первым
const mouse_cursor_icon:usize=0;

// Т.к. добавляется в массив объектов вторым
const wallpaper:usize=1;


pub struct Game{
    settings:GameSettings,
    audio:Audio,

    wallpaper:Wallpaper,
    images:Vec<RgbaImage>,

    frames:u64,
    thread:Option<std::thread::JoinHandle<Vec<RgbaImage>>>,

    // Карта объектов
    // Отрисовываемые объекты и зоны активные для клика
    object_map:ObjectMap,
    saved_drawables:Vec<DrawableObject>,


    keyboard_handler:fn(&mut Self,bool,KeyboardButton,&mut PagedWindow),
    //
    prerendering:fn(&mut Self),
    updates:fn(&mut Self,&mut PagedWindow), 
    click_handler:fn(&mut Self,bool,MouseButton,&mut PagedWindow)
}

impl Game{
    pub fn new<F:FnOnce()->Vec<RgbaImage>+Send+'static>(settings:GameSettings,window:&mut PagedWindow,background:F)->Game{
        // Объекты интерфейса
        let mut saved_drawables=Vec::with_capacity(10);
        let mut object_map=ObjectMap::new();

        object_map.add_new_layer();

        let mut image_base=ImageBase::new(White,unsafe{[
            window_center[0]-100f32,
            window_center[1]-100f32,
            200f32,
            200f32
        ]});

        let cat=Texture::from_path("./resources/images/cat.png",window.display()).unwrap();
        let cat=window.graphics2d().add_textured_object(&image_base,cat).unwrap();

        saved_drawables.push(DrawableObject::new(cat,ObjectType::Textured,DrawType::Common));
        object_map.add_raw_simple_drawable_object(0,cat,ObjectType::Textured,DrawType::Common);

        let cat_eyes_closed=Texture::from_path("./resources/images/cat_eyes_closed.png",window.display()).unwrap();
        let cat_eyes_closed=window.graphics2d().add_textured_object(&image_base,cat_eyes_closed).unwrap();

        saved_drawables.push(DrawableObject::new(cat_eyes_closed,ObjectType::Textured,DrawType::Common));

        image_base.set_rect(unsafe{[
            window_center[0]-200f32,
            window_center[1]-200f32,
            400f32,
            400f32
        ]});

        let gear=Texture::from_path("./resources/images/gear.png",window.display()).unwrap();
        let gear=window.graphics2d().add_textured_object(&image_base,gear).unwrap();

        object_map.add_raw_simple_drawable_object(0,gear,ObjectType::Textured,DrawType::Rotating((0f32,unsafe{window_center})));

        // Подключение аудио системы
        let host=cat_engine::audio::cpal::default_host();
        let audio=Audio::new(host,AudioSettings::new()).unwrap();
        audio.add_track("./resources/music/audio.mp3");
        audio.add_track("./resources/music/button.mp3");


        let thread=std::thread::spawn(background);

        Self{
            audio:audio,
            settings:settings,
            wallpaper:Wallpaper::Colour(White),
            images:Vec::new(),

            frames:0u64,
            thread:Some(thread),

            saved_drawables,
            object_map,

            prerendering:Game::empty_prerendering,
            updates:Game::loading_updates,
            click_handler:Game::empty_click_handler,
            keyboard_handler:Game::empty_keyboard_handler,
        }
    }

    pub fn loading_updates(&mut self,window:&mut PagedWindow){
        if unsafe{!loading}{
            if let Some(thread)=self.thread.take(){
                self.images=thread.join().expect("Ошибка начальной загрузки");
            }

            self.saved_drawables.clear();
            // Отчистка слоёв
            self.object_map.clear_layers();

            for _ in 0..3{
                window.graphics2d().delete_last_textured_object();
            }
            self.audio.play_track(0,0);

            return set_main_menu(self,window)
        }

        if let DrawType::Rotating((angle,_))=&mut self.object_map.get_drawable(0,1).draw_type{
            *angle+=0.05f32;
        }

        if self.frames==20{
            self.object_map.set_drawable(0,0,self.saved_drawables[1].clone());
            // self.cat_eyes_closed
        }
        else{
            if self.frames==30{
                // self.cat
                self.object_map.set_drawable(0,0,self.saved_drawables[0].clone());
                self.frames=0;
            }
        };

        self.frames+=1;
    }

    pub fn empty_prerendering(&mut self){

    }

    pub fn empty_updates(&mut self,_window:&mut PagedWindow){

    }

    pub fn empty_click_handler(&mut self,_pressed:bool,_button:MouseButton,_window:&mut PagedWindow){

    }

    pub fn empty_keyboard_handler(&mut self,_:bool,_:KeyboardButton,_:&mut PagedWindow){

    }
}


impl WindowPage<'static> for Game{
    type Window=PagedWindow;

    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut PagedWindow){

    }

    fn on_update_requested(&mut self,window:&mut PagedWindow){
        (self.updates)(self,window)
    }

    fn on_redraw_requested(&mut self,window:&mut PagedWindow){
        (self.prerendering)(self);

        window.draw(|parameters,graphics|{
            let [dx,dy]=unsafe{mouse_cursor.center_radius()};
            // Рендеринг фона
            if let Wallpaper::Colour(colour)=self.wallpaper{
                // Заполнение цветом
                graphics.clear_colour(colour)
            }
            else{
                // Заполнение картинкой
                let shift=[
                    dx/wallpaper_movement_scale,
                    dy/wallpaper_movement_scale,
                ];
                graphics.draw_shift_textured_object(wallpaper,shift,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
            }

            // Рендеринг объектов
            self.object_map.draw(parameters,graphics);

            // Рендеринг курсора
            graphics.draw_shift_textured_object(mouse_cursor_icon,[dx,dy],ColourFilter::new_mul([1f32;4]),parameters).unwrap();
        }).unwrap();
    }

    fn on_mouse_pressed(&mut self,window:&mut PagedWindow,button:MouseButton){
        (self.click_handler)(self,true,button,window)
    }
    fn on_mouse_released(&mut self,window:&mut PagedWindow,button:MouseButton){
        (self.click_handler)(self,false,button,window)
    }
    fn on_mouse_moved(&mut self,_window:&mut PagedWindow,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut PagedWindow,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,window:&mut PagedWindow,button:KeyboardButton){
        (self.keyboard_handler) (self,true,button,window)
    }
    fn on_keyboard_released(&mut self,window:&mut PagedWindow,button:KeyboardButton){
        (self.keyboard_handler) (self,false,button,window)

    }

    fn on_character_recieved(&mut self,_window:&mut PagedWindow,_character:char){}

    fn on_modifiers_changed(&mut self,_window:&mut PagedWindow,_modifiers:ModifiersState){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    fn on_event_loop_closed(&mut self,_window:&mut Self::Window){}
}

// Алфавит для рендеринга текста (остальные символы будут выведены как неопределённые)
const alphabet:&'static str="АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя1234567890AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZzÕõÄäÖöÜü:();[]!.,";

pub const game_name:&'static str="Любимый в УГАТУ";

pub const wallpaper_movement_scale:f32=16f32;

pub static mut loading:bool=true; // Флаг загрузки


fn main(){
    let mut game_settings=GameSettings::load();

    // Коллекция шрифтов
    let mut fonts=Vec::with_capacity(2);

    {
        let main_font_data=FontOwner::load("./resources/fonts/main.font").unwrap();
        fonts.push(main_font_data);
        let dialogue_font_data=FontOwner::load("./resources/fonts/dialogue.font").unwrap();
        fonts.push(dialogue_font_data);
    }


    // Настройка и создание окна и загрузка функций OpenGL
    let mut window:PagedWindow=match PagedWindow::new(|mut monitors,window_settings|{
        // Установка полноэкранного режима для нужного экрана
        let monitor=game_settings.monitor;
        let monitor=if monitor<monitors.len(){
            monitors.remove(monitor)
        }
        else{
            game_settings.monitor=0;
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

    window.display().gl_window().window().set_cursor_visible(false);

    // Размеры для обоев
    let (dx,dy,width,height)=unsafe{
        let dx=window_width/(wallpaper_movement_scale*2f32);
        let dy=window_height/(wallpaper_movement_scale*2f32);
        let width=(window_width+2f32*dx).ceil();
        let height=(window_height+2f32*dy).ceil();

        (dx,dy,width,height)
    };

    let scale=Scale::new(0.2f32,0.2f32);
    for font in fonts{
        let glyph_cache=GlyphCache::new_alphabet(font.face(),alphabet,scale,window.display());
        let cached_font = CachedFont::raw(font, glyph_cache);
        window.graphics2d().add_font(cached_font).unwrap();
    }

    let loading_resources_thread=move||{
        let mut wallpapers=Vec::with_capacity(1);
        let menu_wallpaper=load_image("./resources/images/wallpapers/main_menu_wallpaper.png",width as u32,height as u32);
        wallpapers.push(menu_wallpaper);
        unsafe{loading=false};
        wallpapers
    };


    let mut image_base=ImageBase::new(White,unsafe{[
        window_center[0]-15f32,
        window_center[1]-15f32,
        30f32,
        30f32
    ]});

    // Загрузка иконки курсора мыши
    let mouse_texture=Texture::from_path("./resources/images/mouse_icon.png",window.display()).unwrap();
    window.graphics2d().add_textured_object(&image_base,mouse_texture).unwrap();

    // Создание тектуры для обоев
    let rect=[
        -dx,
        -dy,
        width,
        height,
    ];

    image_base.set_rect(rect);

    // Создание текстуры чуть больше размера экрана
    let wallpaper_texture=Texture::empty([width as u32,height as u32],window.display()).unwrap();
    window.graphics2d().add_textured_object(&image_base,wallpaper_texture).unwrap();


    let mut page=Game::new(game_settings,&mut window,loading_resources_thread);

    window.run_page(&mut page)
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
        image.into_rgba()
    }
}