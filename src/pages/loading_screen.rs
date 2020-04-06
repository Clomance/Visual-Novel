use crate::*;

// Загрузочный экран
pub unsafe fn loading_screen(events:&mut Events,window:&mut GlutinWindow,gl:&mut GlGraphics)->Game{
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

    'loading:while let Some(e)=events.next(window){
        if !loading{
            break 'loading
        }
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            loading=false;
            return Game::Exit
        } 
        // Рендеринг
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|c,g|{
                g.clear_color(White);
                logo.draw(&logo_texture,&c.draw_state,c.transform.trans(x,y).rot_rad(rotation).trans(-half_size,-half_size),g);
            });
            rotation+=0.1f64;
        }
    }

    // Для планого перехода
    let mut frames=5;
    while let Some(e)=events.next(window){
        // Закрытие игры
        if let Some(_close)=e.close_args(){
            loading=false;
            return Game::Exit
        }
        if let Some(r)=e.render_args(){
            gl.draw(r.viewport(),|_c,g|{
                g.clear_color(White);
            });
            frames-=1;
            if frames==0{
                break
            }
        }
    }

    return Game::MainMenu
}