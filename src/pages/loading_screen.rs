use crate::*;

pub struct LoadingScreen{
    logo_base:ImageBase,
    logo:Texture,
}

impl LoadingScreen{
    pub fn new(window:&mut GameWindow)->LoadingScreen{
        let texture_settings=TextureSettings::new();

        Self{
            logo_base:ImageBase::new(White,[
                0f32,
                0f32,
                200f32,
                200f32
            ]),
            logo:Texture::from_path(window.display(),"./resources/images/logo.png",&texture_settings).unwrap(),
        }
    }

    pub unsafe fn start<F,T>(&mut self,window:&mut GameWindow,background:F)->Game
            where F:FnOnce()->T,
                F:Send+'static,
                T:Send+'static{
        let half_size=100f64;
        let (x,y)=(window_width as f64/2f64,window_height as f64/2f64);
        let mut rotation=0f64;

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
                        g.clear_color(White);
                        self.logo_base.draw(&self.logo,&c.draw_state,c.transform.trans(x,y).rot_rad(rotation).trans(-half_size,-half_size),g);
                    });
                    rotation+=0.1f64;
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
                        g.clear_color(White);
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
