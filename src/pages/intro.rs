use crate::{
    Calibri,
    make_screenshot,
    Game,
};

use super::{
    default_page_smooth
};

use lib::{
    colours::{White,Black},
    TextViewSettings,
    TextViewStaticLined,
};

use engine::{
    // fns
    window_rect,
    // statics
    window_width,
    window_center,
    // types
    Colour,
    // enums
    WindowEvent,
    KeyboardButton,
    // structs
    GameWindow,
    graphics::Rectangle,
};

const page_smooth:f32=default_page_smooth;

const background_color:Colour=Black;

pub struct Intro<'a,'b>{
    text_view:TextViewStaticLined<'b>,
    window:&'a mut GameWindow,
}

impl<'a,'b> Intro<'a,'b>{
    pub fn new(window:&'a mut GameWindow)->Intro<'a,'b>{
        let text="Прогресс сохраняется автоматически";

        let settings=TextViewSettings::new(text,
                unsafe{[
                    0f32,
                    window_center[1]/2f32,
                    window_width,
                    window_center[1]
                ]})
                .font_size(40f32)
                .text_colour(White);

        Self{
            text_view:TextViewStaticLined::new(settings,Calibri!()),
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
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{ // Рендеринг
                    if 1f32<(*window).draw_smooth(|alpha,c,g|{
                        g.clear_colour(background_color);
                        self.text_view.set_alpha_channel(alpha);
                        self.text_view.draw(c,g);
                    }){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(&*window)
                    }
                }
                _=>{}
            }
        }

        self.window.set_smooth(-1f32/128f32);
        while let Some(event)=self.window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{ //Рендеринг
                    if 0f32>(*window).draw_smooth(|alpha,c,g|{
                        g.clear_colour(background_color);
                        self.text_view.set_alpha_channel(alpha);
                        self.text_view.draw(c,g);
                    }){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(&*window)
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

        let mut background=Rectangle::new(window_rect(),background_color);

        while let Some(event)=self.window.next_event(){

            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::Draw=>{
                    if 1f32<(*window).draw_smooth(|alpha,c,g|{
                        background.colour[3]=alpha;
                        background.draw(c,g);
                    }){
                        break
                    }
                }
                
                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        make_screenshot(&*window)
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }
}