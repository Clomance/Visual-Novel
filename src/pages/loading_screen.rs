use crate::{
    loading,
    make_screenshot,
    Game,
};

use lib::White;

use engine::{
    // statics
    window_width,
    window_height,
    // structs
    GameWindow,
    image_base::ImageBase,
    game_texture::Texture,
    // enums
    GameWindowEvent,
    KeyboardButton,
};

pub struct LoadingScreen{
    logo_base:ImageBase,
    logo:Texture,
}

impl LoadingScreen{
    pub fn new(window:&mut GameWindow)->LoadingScreen{
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

    pub unsafe fn start<F,T>(&mut self,window:&mut GameWindow,background:F)->Game
            where F:FnOnce()->T,
                F:Send+'static,
                T:Send+'static{

        let thead=std::thread::spawn(background);

        'loading:while let Some(event)=window.next_event(){
            if !loading{
                let _result=thead.join();
                break 'loading
            }
            match event{
                GameWindowEvent::Exit=>{ // Закрытие игры
                    loading=false;
                    let _result=thead.join();
                    return Game::Exit
                }

                GameWindowEvent::Draw=>{
                    window.draw(|c,g|{
                        g.clear_colour(White);
                        self.logo_base.draw(&self.logo,c,g);
                    });
                }

                GameWindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(window)
                    }
                }
                _=>{}
            }
        }

        // Для планого перехода
        let mut frames=5;
        while let Some(event)=window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{
                    window.draw(|_context,g|{
                        g.clear_colour(White);
                    });
                    frames-=1;
                    if frames==0{
                        break
                    }
                }

                GameWindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(window)
                    }
                }
                _=>{}
            }
        }

        Game::MainMenu
    }
}
