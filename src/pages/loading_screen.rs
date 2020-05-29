use crate::{
    loading,
    make_screenshot,
    Game,
};

use lib::colours::White;

use engine::{
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
    logo_base:ImageBase,
    logo:Texture,
}

impl LoadingScreen{
    pub fn new(window:&mut Window)->LoadingScreen{
        Self{
            logo_base:ImageBase::new(White,unsafe{[
                (window_width-200f32)/2f32,
                (window_height-200f32)/2f32,
                200f32,
                200f32
            ]}),
            logo:Texture::from_path(window.display(),"./resources/images/logo.png").unwrap(),
        }
    }

    pub unsafe fn start<F,T>(self,window:&mut Window,background:F)->Game
            where F:FnOnce()->T,
                F:Send+'static,
                T:Send+'static{

        let mut t=0f32;
        let thread=std::thread::spawn(background);

        'loading:while let Some(event)=window.next_event(){
            if !loading{
                let _result=thread.join();
                break 'loading
            }
            match event{
                WindowEvent::Exit=>{ // Закрытие игры
                    loading=false;
                    let _result=thread.join();
                    return Game::Exit
                }

                WindowEvent::Draw=>{
                    window.draw(|c,g|{
                        g.clear_colour(White);
                        self.logo_base.draw_rotate(&self.logo,t,c,g);
                    });
                    t+=0.05f32;
                    if t>360f32{
                        t=0f32;
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(window,|p,g|{
                            g.clear_colour(White);
                            self.logo_base.draw_rotate(&self.logo,t,p,g);
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

        Game::MainMenu
    }
}
