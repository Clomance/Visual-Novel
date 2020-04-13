use crate::*;

// Загрузочный экран
pub unsafe fn loading_screen(window:&mut GameWindow,gl:&mut GlGraphics)->Game{
    let texture_settings=TextureSettings::new();
    let half_size=100f64;
    let logo=Image::new().rect([
        0f64,
        0f64,
        200f64,
        200f64
    ]);
    let logo_texture=Texture::from_path("images/logo.png",&texture_settings).unwrap();

    let (x,y)=(window_width/2f64,window_height/2f64);
    let mut rotation=0f64;

    'loading:while let Some(event)=window.next_event(){
        if !loading{
            break 'loading
        }
        match event{
            GameWindowEvent::Exit=>{loading=false;return Game::Exit} // Закрытие игры

            GameWindowEvent::Draw(viewport)=>{
                gl.draw(viewport,|c,g|{
                    g.clear_color(White);
                    logo.draw(&logo_texture,&c.draw_state,c.transform.trans(x,y).rot_rad(rotation).trans(-half_size,-half_size),g);
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

            GameWindowEvent::Draw(viewport)=>{
                gl.draw(viewport,|_context,g|{
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