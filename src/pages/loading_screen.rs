use crate::*;

pub struct LoadingScreen{
    logo_base:Image,
    logo:Texture,
}

impl LoadingScreen{
    #[inline(always)]
    pub fn new()->LoadingScreen{
        let texture_settings=TextureSettings::new();

        Self{
            logo_base:Image::new().rect([
                0f64,
                0f64,
                200f64,
                200f64
            ]),
            logo:Texture::from_path("images/logo.png",&texture_settings).unwrap(),
        }
    }

    pub unsafe fn start(&mut self,window:&mut GameWindow)->Game{
        let half_size=100f64;
        let (x,y)=(window_width/2f64,window_height/2f64);
        let mut rotation=0f64;

        'loading:while let Some(event)=window.next_event(){
            if !loading{
                break 'loading
            }
            match event{
                GameWindowEvent::Exit=>{loading=false;return Game::Exit} // Закрытие игры
    
                GameWindowEvent::Draw=>{
                    window.draw(|c,g|{
                        g.clear_color(White);
                        self.logo_base.draw(&self.logo,&c.draw_state,c.transform.trans(x,y).rot_rad(rotation).trans(-half_size,-half_size),g);
                    });
                    rotation+=0.1f64;
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
                _=>{}
            }
        }
    
        Game::MainMenu
    }
}
