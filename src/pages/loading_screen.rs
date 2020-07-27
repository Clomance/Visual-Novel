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
    MouseScrollDelta,
    ModifiersState,
};

use cat_engine::glium::DrawParameters;

use std::thread::JoinHandle;

pub struct LoadingScreen{
    cat:usize,
    cat_eyes_closed:usize,
    gear:usize,

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

        let cat=Texture::from_path("./resources/images/cat.png",window.display()).unwrap();

        let cat=window.graphics2d().add_textured_object(&image_base,cat).unwrap();

        let cat_eyes_closed=Texture::from_path("./resources/images/cat_eyes_closed.png",window.display()).unwrap();

        let cat_eyes_closed=window.graphics2d().add_textured_object(&image_base,cat_eyes_closed).unwrap();

        image_base.set_rect(unsafe{[
            window_center[0]-200f32,
            window_center[1]-200f32,
            400f32,
            400f32
        ]});

        let gear=Texture::from_path("./resources/images/gear.png",window.display()).unwrap();

        let gear=window.graphics2d().add_textured_object(&image_base,gear).unwrap();

        Self{
            cat,
            cat_eyes_closed,
            gear,
            angle:0f32,
            frames:0u8,
            background:Some(std::thread::spawn(background)),
            output:Game::MainMenu,
        }
    }

    fn draw(&self,index:usize,angle:f32,parameters:&DrawParameters,graphics:&mut Graphics){
        graphics.clear_colour(White);
        graphics.draw_textured_object(index,parameters);

        graphics.draw_rotate_textured_object(
            self.gear,
            unsafe{window_center},
            angle,
            parameters
        );
    }
}


impl WindowPage<'static> for LoadingScreen{
    type Window=PagedWindow;

    type Output=Game;

    fn on_window_close_requested(&mut self,window:&mut PagedWindow){
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
        let image=if self.frames>=20{
            if self.frames>=30{
                self.frames=0;
            }
            self.cat_eyes_closed
        }
        else{
            self.cat
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

    fn on_modifiers_changed(&mut self,_window:&mut PagedWindow,_modifiers:ModifiersState){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    fn on_event_loop_closed(&mut self,window:&mut Self::Window)->Game{
        window.graphics2d().clear_textured_object_array();
        self.output.clone()
    }
}