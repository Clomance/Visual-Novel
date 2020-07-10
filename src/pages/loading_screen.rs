use crate::{
    loading,
    make_screenshot,
    Game,
};

use lib::colours::White;

use cat_engine::{
    // statics
    window_width,
    window_height,
    // structs
    Window,
    image::{ImageBase,Texture},
    // enums
    WindowEvent,
    KeyboardButton,
    graphics::Graphics,
};

use cat_engine::glium::DrawParameters;

pub struct LoadingScreen{
    cat:Texture,
    range:usize,
    cat_eyes_closed:Texture,
    gear:Texture,
    frames:u8,
}

impl LoadingScreen{
    pub fn new(window:&mut Window)->LoadingScreen{
        let mut image_base=ImageBase::new(White,unsafe{[
            (window_width-200f32)/2f32,
            (window_height-200f32)/2f32,
            200f32,
            200f32
        ]});

        // Установка области для быстрой отрисовки иконки загрузки
        let range=window.graphics().bind_image(4..8usize,image_base.clone()).unwrap();

        image_base.set_rect(unsafe{[
            (window_width-400f32)/2f32,
            (window_height-400f32)/2f32,
            400f32,
            400f32
        ]});

        window.graphics().bind_rotating_image(8..12usize,image_base).unwrap();

        Self{
            cat:Texture::from_path("./resources/images/cat.png",window.display()).unwrap(),
            range,
            cat_eyes_closed:Texture::from_path("./resources/images/cat_eyes_closed.png",window.display()).unwrap(),
            gear:Texture::from_path("./resources/images/gear.png",window.display()).unwrap(),
            frames:0u8,
        }
    }

    pub fn start<F,T>(mut self,window:&mut Window,background:F)->Game
            where F:FnOnce()->T,F:Send+'static,T:Send+'static{
        let mut angle=0f32;
        let thread=std::thread::spawn(background);

        'loading:while let Some(event)=window.next_event(){
            if unsafe{!loading}{
                let _result=thread.join();
                break 'loading
            }
            match event{
                WindowEvent::Exit=>{ // Закрытие игры
                    unsafe{loading=false}
                    let _result=thread.join();
                    return Game::Exit
                }

                WindowEvent::Draw=>{
                    self.frames+=1;
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
                        self.draw(image,angle,parameters,graphics)
                    });
                    angle+=0.05f32;
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        let image=if self.frames>=40{
                            &self.cat_eyes_closed
                        }
                        else{
                            &self.cat
                        };

                        if Game::Exit==make_screenshot(window,|parameters,graphics|{
                            self.draw(image,angle,parameters,graphics)
                        }){
                            return Game::Exit
                        }
                    }
                }
                _=>{}
            }
        }

        // Для планого перехода
        self.frames=5;
        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{
                    window.draw(|_context,g|{
                        g.clear_colour(White);
                    });
                    self.frames-=1;
                    if self.frames==0{
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        if Game::Exit==make_screenshot(window,|_,g|{
                            g.clear_colour(White);
                        }){
                            return Game::Exit
                        }
                    }
                }
                _=>{}
            }
        }

        // Удаление области для иконки загрузки
        window.graphics().pop_texture();

        window.graphics().pop_texture();

        Game::MainMenu
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
            angle,
            parameters
        );
    }
}
