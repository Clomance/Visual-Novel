use crate::{
    loading,
    Game,
};

use lib::colours::White;

use cat_engine::{
    // statics
    window_center,
    // enums
    KeyboardButton,
    MouseButton,
    graphics::Graphics,
    // traits
    Window,
    WindowPage,
    // structs
    PagedWindow,
    image::{ImageBase,Texture},
    glium::glutin::event::MouseScrollDelta,
};

use cat_engine::glium::DrawParameters;

use std::path::PathBuf;
use std::thread::JoinHandle;

pub struct LoadingScreen{
    cat:Texture,
    range:usize,
    cat_eyes_closed:Texture,

    gear:Texture,
    angle:f32,

    frames:u8,

    background:Option<JoinHandle<()>>,

    output:Game,
}

impl LoadingScreen{
    pub fn new<F:FnOnce()+Send+'static>(window:&mut PagedWindow,background:F)->LoadingScreen{
        let mut image_base=ImageBase::new(White,unsafe{[
            window_center[0]-100f32,
            window_center[1]-100f32,
            200f32,
            200f32
        ]});

        // Установка области для быстрой отрисовки иконки загрузки
        let range=window.graphics().bind_image(4..8usize,image_base.clone()).unwrap();

        image_base.set_rect(unsafe{[
            window_center[0]-200f32,
            window_center[1]-200f32,
            400f32,
            400f32
        ]});

        window.graphics().bind_rotating_image(8..12usize,image_base).unwrap();

        Self{
            cat:Texture::from_path("./resources/images/cat.png",window.display()).unwrap(),
            range,
            cat_eyes_closed:Texture::from_path("./resources/images/cat_eyes_closed.png",window.display()).unwrap(),
            
            gear:Texture::from_path("./resources/images/gear.png",window.display()).unwrap(),
            angle:0f32,

            frames:0u8,

            background:Some(std::thread::spawn(background)),

            output:Game::MainMenu,
        }
    }

    fn draw(&self,image:&Texture,angle:f32,parameters:&mut DrawParameters,graphics:&mut Graphics){
        graphics.clear_colour(White);
        graphics.draw_range_image(
            self.range,
            image,
            White,
            parameters
        );

        graphics.draw_rotate_range_image(
            self.range+1,
            &self.gear,
            White,
            unsafe{window_center},
            angle,
            parameters
        );
    }
}


impl WindowPage<'static> for LoadingScreen{
    type Window=PagedWindow;

    type Output=Game;

    fn on_close_requested(&mut self,window:&mut PagedWindow){
        let _=window.stop_events();
        self.output=Game::Exit;
    }

    fn on_update_requested(&mut self,window:&mut PagedWindow){
        if unsafe{!loading}{
            if let Some(thread)=self.background.take(){
                let _result=thread.join();
            }

            window.stop_events();
        }

        self.angle+=0.05f32;
        self.frames+=1;
    }

    fn on_redraw_requested(&mut self,window:&mut PagedWindow){
        let image=if self.frames>=40{
            if self.frames>=50{
                self.frames=0;
            }
            &self.cat_eyes_closed
        }
        else{
            &self.cat
        };

        window.draw(|parameters,graphics|{
            graphics.clear_colour([1.0;4]);
            self.draw(image,self.angle,parameters,graphics)
        }).unwrap();
    }

    fn on_mouse_pressed(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_released(&mut self,_window:&mut PagedWindow,_button:MouseButton){}
    fn on_mouse_moved(&mut self,_window:&mut PagedWindow,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut PagedWindow,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,_window:&mut PagedWindow,_button:KeyboardButton){}

    fn on_keyboard_released(&mut self,_window:&mut PagedWindow,_button:KeyboardButton){}

    fn on_character_recieved(&mut self,_window:&mut PagedWindow,_character:char){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    fn on_file_dropped(&mut self,_:&mut PagedWindow,_:PathBuf){}
    fn on_file_hovered(&mut self,_:&mut PagedWindow,_:PathBuf){}
    fn on_file_hovered_canceled(&mut self,_:&mut PagedWindow){}

    fn on_event_loop_closed(&mut self,window:&mut Self::Window)->Game{
        // Удаление области для иконки загрузки
        window.graphics().pop_texture();

        window.graphics().pop_texture();

        self.output.clone()
    }
}