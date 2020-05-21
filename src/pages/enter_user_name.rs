use crate::{
    Game,
    Settings,
};

use super::MainMenu;

use lib::{
    colours::{
        Blue,
        Light_blue,
    },
    Drawable,
    EditTextView,
    EditTextViewSettings,
    TextViewSettings,
    TextViewStaticLineDependent,
};

use engine::{
    // statics
    window_width,
    window_height,
    // structs
    text::Glyphs,
    GameWindow,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
};

const page_smooth:f32=1f32/8f32;

pub struct EnterUserName<'a,'b,'c,'d,'e>{
    head:TextViewStaticLineDependent,
    glyphs:Glyphs<'a>,
    input:EditTextView<'b>,
    main_menu:&'c mut MainMenu<'e,'d>,
    window:*mut GameWindow,
}

impl<'a,'b,'c,'d,'e> EnterUserName<'a,'b,'c,'d,'e>{
    pub unsafe fn new(main_menu:&'c mut MainMenu<'e,'d>,window:&mut GameWindow)->EnterUserName<'a,'b,'c,'d,'e>{

        // Загрузка шрифта
        let head_glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");

        let head_settings=TextViewSettings::new("Введите своё имя",[
                    window_width/2f32-150f32,
                    window_height/2f32-150f32,
                    300f32,
                    70f32,
                ]);

        let glyphs=Glyphs::load("./resources/fonts/CALIBRI.TTF");

        let settings=EditTextViewSettings::new("",[
                    window_width/2f32-150f32,
                    window_height/2f32-150f32,
                    300f32,
                    150f32,
                ])
                .background_colour(Light_blue)
                .border_colour(Some(Blue));

        Self{
            head:TextViewStaticLineDependent::new(head_settings,&head_glyphs),
            glyphs:head_glyphs,
            input:EditTextView::new(settings,glyphs),
            main_menu:main_menu,
            window:window as *mut GameWindow,
        }
    }

    pub unsafe fn start(&mut self)->Game{
        match self.smooth(){
            Game::Exit=>return Game::Exit,
            Game::Back=>return Game::Back,
            _=>{}
        }

        // Полная отрисовка
        while let Some(event)=(*self.window).next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::MouseMovementDelta((dx,dy))=>{
                    self.main_menu.wallpaper.mouse_shift(dx,dy);
                    self.main_menu.menu.mouse_shift(dx,dy)
                }

                WindowEvent::MouseReleased(button)=>{
                    match button{
                        MouseButton::Left=>{
                            if !self.input.clicked(){
                                return Game::Back
                            }
                        }
                        _=>{}
                    }
                }

                WindowEvent::Draw=>{ // Рендеринг
                    (*self.window).draw(|c,g|{
                        self.main_menu.draw(c,g);
                        self.input.draw(c,g);
                        self.head.draw(c,g,&self.glyphs);
                    })
                }

                WindowEvent::CharacterInput(character)=>self.input.push_char(character),

                WindowEvent::KeyboardPressed(button)=>{
                    match button{
                        KeyboardButton::Backspace=>self.input.pop_char(), // Удаление
                        _=>{}
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::Escape=>return Game::Back,
                        
                        KeyboardButton::Enter=>{
                            let name=self.input.text().clone();
                            if !name.is_empty(){
                                Settings.user_name=name;
                                return Game::NewGamePlay
                            }
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        }
        Game::Exit
    }

    // Сглаживание перехода к странице (открытие)
    pub unsafe fn smooth(&mut self)->Game{
        (*self.window).set_new_smooth(page_smooth);

        while let Some(event)=(*self.window).next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::MouseMovementDelta((dx,dy))=>self.main_menu.menu.mouse_shift(dx,dy),

                WindowEvent::Draw=>{ // Рендеринг
                    if 1f32<(*self.window).draw_smooth(|alpha,c,g|{
                        self.main_menu.draw(c,g);

                        self.input.draw_smooth(alpha,c,g);
                        self.head.draw_smooth(alpha,c,g,&self.glyphs);
                    }){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>{
                    match button{
                        KeyboardButton::Escape=>return Game::Back,
                        _=>{}
                    }
                }
                _=>{}
            }
        }
        Game::Current
    }
}