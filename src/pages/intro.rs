use crate::*;

const page_smooth:f32=Intro_smooth;

const background_color:Color=Black;

pub struct Intro<'a,'b>{
    text_view:TextView<'a,TextLine>,
    window:&'b mut GameWindow,
}

impl<'a,'b> Intro<'a,'b>{
    #[inline(always)]
    pub unsafe fn new(window:&'b mut GameWindow)->Intro<'a,'b>{
        let texture_settings=TextureSettings::new();

        let glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",(),texture_settings).unwrap();

        let settings=TextViewSettings::new()
                .rect([
                    window_center[0],
                    window_center[1],
                    0f64,
                    0f64
                ])
                .text(Settings.game_name.clone())
                .font_size(40)
                .text_color(Head_main_menu_color);

        Self{
            text_view:TextView::new(settings,glyphs), // Создание меню
            window:window
        }
    }

    #[inline(always)]
    pub unsafe fn start(&mut self)->Game{

        if self.smooth()==Game::Exit{
            return Game::Exit
        }

        let window=self.window as *mut GameWindow;

        self.window.set_new_smooth(1f32/128f32);

        while let Some(event)=self.window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ //Рендеринг
                    if !(*window).draw_smooth(|alpha,c,g|{
                        g.clear_color(background_color);
                        self.text_view.draw_smooth(alpha,c,g);
                    }){
                        break
                    }
                }
                _=>{}
            }
        }

        let mut alpha=1.0;
        let smooth=1f32/128f32;

        while let Some(event)=self.window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ //Рендеринг
                    (*window).draw(|c,g|{
                        g.clear_color(background_color);
                        self.text_view.draw_smooth(alpha,c,g);
                    });

                    alpha-=smooth;
                    if alpha<0.0{
                        break
                    }
                }
                _=>{}
            }
        }

        Game::ContinueGamePlay
    }

    #[inline(always)]
    pub unsafe fn smooth(&mut self)->Game{
        self.window.set_new_smooth(page_smooth);
        let window=self.window as *mut GameWindow;

        let mut background=Background::new(background_color,[
            0f64,
            0f64,
            window_width,
            window_height
        ]);

        while let Some(event)=self.window.next_event(){
            
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{
                    if !(*window).draw_smooth(|alpha,c,g|{
                        background.draw_smooth(alpha,c,g);
                    }){
                        break
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }
}