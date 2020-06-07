use crate::{
    Main_font,
    Game,
    Settings,
};

use super::MainMenu;

use lib::{
    colours::{
        Blue,
        Light_blue,
    },
    AlignY,
    Drawable,
    EditTextView,
    EditTextViewSettings,
    TextViewSettings,
    TextViewStaticLine,
};

use engine::{
    // statics
    window_width,
    window_height,
    // enums
    WindowEvent,
    MouseButton,
    KeyboardButton,
    // structs
    Window,
};

// Шаг альфа-канала, для плавного перехода
const page_smooth:f32=1f32/8f32; // 1 к количеству кадров перехода

pub struct EnterUserName<'a,'c,'e>{
    head:TextViewStaticLine<'a>,
    input:EditTextView<'a>,
    main_menu:&'c mut MainMenu<'a,'e>,
}

impl<'a,'c,'e> EnterUserName<'a,'c,'e>{
    pub fn new(main_menu:&'c mut MainMenu<'a,'e>)->EnterUserName<'a,'c,'e>{
        // Область для поля ввода
        let mut rect=unsafe{[
            window_width/2f32-150f32,
            window_height/2f32-170f32,
            300f32,
            150f32,
        ]};

        // Настройка заголовка
        let head_settings=TextViewSettings::new("Введите своё имя",rect).align_y(AlignY::Up);

        // Настройка поля ввода
        rect[1]-=20f32;
        let settings=EditTextViewSettings::new("",rect)
                .background_colour(Light_blue)
                .border_colour(Blue);

        Self{
            head:TextViewStaticLine::new(head_settings,Main_font!()),
            input:EditTextView::new(settings,Main_font!()),
            main_menu:main_menu,
        }
    }

    pub fn start(mut self,window:&mut Window)->Game{
        match self.smooth(window){
            Game::Exit=>return Game::Exit,
            Game::Back=>return Game::Back,
            _=>{}
        }

        // Главный цикл
        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                // Движение мыши
                WindowEvent::MouseMovementDelta(shift)=>self.main_menu.mouse_shift(shift),

                WindowEvent::MouseReleased(button)=>match button{
                    MouseButton::Left=>{
                        if !self.input.clicked(){
                            return Game::Back
                        }
                    }
                    _=>{}
                }

                // Рендеринг
                WindowEvent::Draw=>window.draw(|c,g|{
                    self.main_menu.draw(c,g);
                    self.input.draw(c,g);
                    self.head.draw(c,g);
                }),

                // Ввод символов
                WindowEvent::CharacterInput(character)=>self.input.push_char(character),

                WindowEvent::KeyboardPressed(button)=>match button{
                    KeyboardButton::Backspace=>self.input.pop_char(), // Удаление
                    _=>{}
                }

                WindowEvent::KeyboardReleased(button)=>match button{
                    KeyboardButton::Escape=>return Game::Back,

                    KeyboardButton::Enter=>unsafe{
                        let name=self.input.text().clone();
                        if !name.is_empty(){
                            Settings.user_name=name;
                            return Game::NewGamePlay
                        }
                    }
                    _=>{}
                }
                _=>{}
            }
        }
        Game::Exit
    }

    // Сглаживание перехода к странице (открытие)
    pub fn smooth(&mut self,window:&mut Window)->Game{
        window.set_new_smooth(page_smooth);

        while let Some(event)=window.next_event(){
            match event{
                WindowEvent::Exit=>return Game::Exit, // Закрытие игры

                WindowEvent::MouseMovementDelta(shift)=>self.main_menu.mouse_shift(shift),
                // Рендеринг
                WindowEvent::Draw=>{
                    if 1f32<window.draw_smooth(|alpha,c,g|{
                        self.main_menu.draw(c,g);

                        self.input.draw_smooth(alpha,c,g);
                        self.head.draw_smooth(alpha,c,g);
                    }){
                        break
                    }
                }

                WindowEvent::KeyboardReleased(button)=>match button{
                    KeyboardButton::Escape=>return Game::Back,
                    _=>{}
                }

                _=>{}
            }
        }
        Game::Current
    }
}