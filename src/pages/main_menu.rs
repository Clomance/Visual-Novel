use crate::{
    Game,
};

use lib::{
    colours::Gray,
};

use cat_engine::{
    // statics
    mouse_cursor,
    window_center,
    // structs
    Window,
    WindowEvent,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,Texture},
};


const loading_screen_assets:usize=2;

pub struct MainMenu{

}

impl MainMenu{
    pub fn new(window:&Window,graphics:&mut Graphics2D)->MainMenu{
        Self{}
    }

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D)->Game{
        let mut result=Game::Next;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>{
                    result=Game::Exit;
                }

                WindowEvent::Update=>{
                    
                }
                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                    window.draw(graphics,|graphics|{
                        graphics.clear_colour(Gray);

                        // Рендеринг курсора
                        graphics.draw_shift_textured_object(0,[dx,dy]).unwrap();
                    }).unwrap();
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