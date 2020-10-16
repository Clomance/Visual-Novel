use crate::{
    Main_font,
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

use cat_engine::{
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
    // traits
    Window,
    // structs
    DefaultWindow,
    shapes::Rectangle,
};

const page_smooth:f32=default_page_smooth;

const background_color:Colour=Black;

pub struct Intro<'b>{
    text_view:TextViewStaticLined<'b>,
}

impl<'b> Intro<'b>{
    pub fn new()->Intro<'b>{
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
            text_view:TextViewStaticLined::new(settings,Main_font!()),
        }
    }

    pub unsafe fn start(mut self,window:&mut DefaultWindow)->Game{
        if self.smooth(window)==Game::Exit{
            return Game::Exit
        }

        window.set_new_smooth(1f32/128f32);

        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::CloseRequested=>return Game::Exit, // Закрытие игры

                WindowEvent::RedrawRequested=>{ // Рендеринг
                    if 1f32<(*window).draw_smooth(|alpha,c,g|{
                        g.clear_colour(background_color);
                        self.text_view.set_alpha_channel(alpha);
                        self.text_view.draw(c,g);
                    }).unwrap(){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        if Game::Exit==make_screenshot(&mut (*window),|p,g|{
                            g.clear_colour(background_color);
                            self.text_view.draw(p,g);
                        }){
                            return Game::Exit
                        }
                    }
                }
                _=>{}
            }
        }

        window.set_smooth(-1f32/128f32);
        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::CloseRequested=>return Game::Exit, // Закрытие игры

                WindowEvent::RedrawRequested=>{ //Рендеринг
                    if 0f32>(*window).draw_smooth(|alpha,c,g|{
                        g.clear_colour(background_color);
                        self.text_view.set_alpha_channel(alpha);
                        self.text_view.draw(c,g);
                    }).unwrap(){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        if Game::Exit==make_screenshot(&mut (*window),|p,g|{
                            g.clear_colour(background_color);
                            self.text_view.draw(p,g);
                        }){
                            return Game::Exit
                        }
                    }
                }
                _=>{}
            }
        }

        Game::ContinueGamePlay
    }

    pub unsafe fn smooth(&mut self,window:&mut DefaultWindow)->Game{
        window.set_new_smooth(page_smooth);

        let mut background=Rectangle::new(window_rect(),background_color);

        while let Some(event)=window.next_event(){

            match event{
                WindowEvent::CloseRequested=>return Game::Exit, // Закрытие игры

                WindowEvent::RedrawRequested=>{
                    if 1f32<(*window).draw_smooth(|alpha,p,g|{
                        background.colour[3]=alpha;
                        background.draw(p,g);
                    }).unwrap(){
                        break
                    }
                }
                
                WindowEvent::KeyboardReleased(button)=>{
                    if button==KeyboardButton::F5{
                        if Game::Exit==make_screenshot(window,|p,g|{
                            background.draw(p,g);
                        }){
                            return Game::Exit
                        }
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }
}