use crate::*;

use lib::game_engine::text::Glyphs;

const page_smooth:f32=Intro_smooth;

const background_color:Color=Black;

pub struct Intro<'a,'b>{
    text_view:TextViewStaticLinedDependent,
    glyphs:Glyphs<'a>,
    window:&'b mut GameWindow,
}

impl<'a,'b> Intro<'a,'b>{
    pub unsafe fn new(window:&'b mut GameWindow)->Intro<'a,'b>{
        let mut glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");

        let text="Прогресс сохраняется автоматически";

        let settings=TextViewSettings::new(text,
                [
                    0f64,
                    window_center[1]/2f64,
                    window_width,
                    window_center[1]
                ])
                .font_size(40f32)
                .text_color(White);

        Self{
            text_view:TextViewStaticLinedDependent::new(settings,&mut glyphs),
            glyphs:glyphs,
            window:window
        }
    }

    pub unsafe fn start(&mut self)->Game{
        if self.smooth()==Game::Exit{
            return Game::Exit
        }

        let window=self.window as *mut GameWindow;

        self.window.set_new_smooth(1f32/128f32);

        while let Some(event)=self.window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ // Рендеринг
                    if 1f32<(*window).draw_smooth(|alpha,c,g|{
                        g.clear_color(background_color);
                        self.text_view.set_alpha_channel(alpha);
                        self.text_view.draw(c,g,&mut self.glyphs);
                    }){
                        break
                    }
                }
                _=>{}
            }
        }

        self.window.set_smooth(-1f32/128f32);
        while let Some(event)=self.window.next_event(){
            match event{
                GameWindowEvent::Exit=>return Game::Exit, // Закрытие игры

                GameWindowEvent::Draw=>{ //Рендеринг
                    if 0f32>(*window).draw_smooth(|alpha,c,g|{
                        g.clear_color(background_color);
                        self.text_view.set_alpha_channel(alpha);
                        self.text_view.draw(c,g,&mut self.glyphs);
                    }){
                        break
                    }
                }
                _=>{}
            }
        }

        Game::ContinueGamePlay
    }

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
                    if 1f32<(*window).draw_smooth(|alpha,c,g|{
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