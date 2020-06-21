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
};

pub struct LoadingScreen{
    logo:Texture,
    range:usize,
}

impl LoadingScreen{
    pub fn new(window:&mut Window)->LoadingScreen{
        let image_base=ImageBase::new(White,unsafe{[
            (window_width-400f32)/2f32,
            (window_height-400f32)/2f32,
            400f32,
            400f32
        ]});

        // Установка области для быстрой отрисовки иконки загрузки
        let range=window.graphics().bind_rotating_image(4..8usize,image_base).unwrap();

        Self{
            range,
            logo:Texture::from_path("./resources/images/logo.png",window.display()).unwrap(),
        }
    }

    pub fn start<F,T>(self,window:&mut Window,background:F)->Game
            where F:FnOnce()->T,F:Send+'static,T:Send+'static{
        let mut t=0f32;
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
                    window.draw(|parameters,graphics|{
                        graphics.clear_colour(White);
                        graphics.draw_rotate_range_image(
                            self.range,
                            &self.logo,
                            White,
                            t,
                            parameters
                        );
                    });
                    t+=0.05f32;
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(window,|parameters,graphics|{
                            graphics.clear_colour(White);
                            graphics.draw_rotate_range_image(
                                self.range,
                                &self.logo,
                                White,
                                t,
                                parameters
                            );
                        })
                    }
                }
                _=>{}
            }
        }

        // Для планого перехода
        let mut frames=5;
        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{
                    window.draw(|_context,g|{
                        g.clear_colour(White);
                    });
                    frames-=1;
                    if frames==0{
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(window,|_,g|{
                            g.clear_colour(White);
                        })
                    }
                }
                _=>{}
            }
        }

        // Удаление области для иконки загрузки
        window.graphics().unbind_texture(self.range);

        Game::MainMenu
    }
}
