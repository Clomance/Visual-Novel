use super::{
    graphics::{Graphics2D,GameGraphics},
    mouse_cursor::{MouseCursor,MouseCursorIcon},
};

use std::{
    collections::VecDeque,
    path::Path,
};

use glium::{
    Display,
    Surface,
    Frame,
    draw_parameters::{
        DrawParameters,
        Blend,
        BlendingFunction,
        LinearBlendingFactor,
        BackfaceCullingMode,
    },
};

use glium::glutin::{
    event::{
        Event,
        WindowEvent as GWindowEvent,
        MouseButton as GMouseButton,
        ElementState,
    },
    event_loop::{ControlFlow,EventLoop},
    window::{WindowBuilder,Fullscreen},
    ContextBuilder,
    window::Window,
    platform::desktop::EventLoopExtDesktop,
    window::Icon
};

use image::GenericImageView;


/*
    EventLoop - минимум четыре шага для моей схемы с мгновенным закрытием цикла обработки событий:
    1) NewEvent
    2) MainEventsCleared
    (RedrawRequested всегда идет после MainEventsCleared)
    3) RedrawEventsCleared
    4) LoopDestroyed


    Все события обрабатываются и добавляются в очередь внешней обработки (GameWindow.events)
        для работы с ними вне структуры окна

    При потере фокуса игра сворачивается, передача событий внешнему управлению прекращается
    При получении фокуса игра возвращается в исходное состояние
*/

pub static mut mouse_cursor:MouseCursor=MouseCursor::new();

pub static mut window_width:f32=0f32;
pub static mut window_height:f32=0f32;
pub static mut window_center:[f32;2]=[0f32;2];

pub struct GameWindow{
    event_loop:EventLoop<()>,
    display:Display,
    graphics:Graphics2D,
    mouse_icon:MouseCursorIcon,
    events:VecDeque<WindowEvent>,
    events_handler:fn(&mut Self),
    width:u32,
    height:u32,
    alpha_channel:f32,
    smooth:f32,
}

#[derive(Clone)]
pub enum WindowEvent{
    None,
    Draw,

    Hide(bool),

    MouseMovementDelta((f32,f32)),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),

    KeyboardPressed(KeyboardButton),
    KeyboardReleased(KeyboardButton),
    CharacterInput(char),

    Exit,
}

#[derive(Clone)]
pub enum MouseButton{
    Left,
    Middle,
    Right,
}

use WindowEvent::*;

impl GameWindow{
    #[inline(always)]
    pub fn new(title:&String)->GameWindow{
        let event_loop=EventLoop::new();
        let monitor=event_loop.primary_monitor();
        let size=monitor.size();

        let fullscreen=Fullscreen::Borderless(monitor);

        unsafe{
            window_width=size.width as f32;
            window_height=size.height as f32;
            window_center=[window_width/2f32,window_height/2f32];
        }

        let icon=load_window_icon();

        let window_builder=WindowBuilder::new()
            .with_inner_size(size)
            .with_decorations(false)
            .with_resizable(false)
            .with_always_on_top(true)
            .with_title(title)
            .with_window_icon(Some(icon))
            .with_fullscreen(Some(fullscreen));


        let context_builder=ContextBuilder::new()
            .with_vsync(true)
            .with_srgb(true);

        // Создание окна
        let display=Display::new(window_builder,context_builder,&event_loop).unwrap();

        let mut frame=display.draw();       //
        frame.clear_color(1.0,1.0,1.0,1.0); // Заполнение окна
        frame.finish().unwrap();            //


        display.gl_window().window().set_cursor_visible(false);

        Self{
            event_loop,
            graphics:Graphics2D::new(&display),
            mouse_icon:MouseCursorIcon::new(&display),
            display:display,
            events:VecDeque::with_capacity(32),
            events_handler:GameWindow::event_listener,
            width:size.width,
            height:size.height,
            alpha_channel:0f32,
            smooth:0f32,
        }
    }

    #[inline(always)]
    pub fn display(&mut self)->&mut Display{
        &mut self.display
    }

    #[inline(always)]
    pub fn size(&self)->[f32;2]{
        [self.width as f32,self.height as f32]
    }

    // Получение событий
    pub fn next_event(&mut self)->Option<WindowEvent>{
        if self.events.is_empty(){
            (self.events_handler)(self); // Вызов функции обработки событий
        }
        self.events.pop_front()
    }

    #[inline(always)]
    pub fn request_redraw(&self){
        self.display.gl_window().window().request_redraw();
    }

    #[inline(always)]
    pub fn set_hide(&self,hide:bool){
        self.display.gl_window().window().set_minimized(hide);
    }
}

// Функции обработки событий
impl GameWindow{
    // Обычная функция обработки событий
    fn event_listener(&mut self){
        let vec=&mut self.events as *mut VecDeque<WindowEvent>;

        let game_window=self as *mut GameWindow;

        let display=self.display.gl_window();

        let window:&Window=display.window();

        self.event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            let next_event=match event{
                Event::NewEvents(_)=>None, // Игнорирование

                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>Exit,

                        // Движение мыши (конечное положение)
                        GWindowEvent::CursorMoved{position,..}=>unsafe{
                            let last_position=mouse_cursor.position();

                            let position=[position.x as f32,position.y as f32];

                            let dx=position[0]-last_position[0];
                            let dy=position[1]-last_position[1];
                            mouse_cursor.set_position([position[0],position[1]]);
                            MouseMovementDelta((dx,dy))
                        }
                        
                        // Обработка действий с кнопками мыши (только стандартные кнопки)
                        GWindowEvent::MouseInput{button,state,..}=>{
                            if state==ElementState::Pressed{
                                match button{
                                    GMouseButton::Left=>unsafe{
                                        (*game_window).mouse_icon.pressed();
                                        MousePressed(MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>MousePressed(MouseButton::Middle),
                                    GMouseButton::Right=>MousePressed(MouseButton::Right),
                                    GMouseButton::Other(_)=>None
                                }
                            }
                            else{
                                match button{
                                    GMouseButton::Left=>unsafe{
                                        (*game_window).mouse_icon.released();
                                        MouseReleased(MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>MouseReleased(MouseButton::Middle),
                                    GMouseButton::Right=>MouseReleased(MouseButton::Right),
                                    GMouseButton::Other(_)=>None
                                }
                            }
                        }

                        GWindowEvent::KeyboardInput{input,..}=>{
                            let key=if let Some(key)=input.virtual_keycode{
                                unsafe{std::mem::transmute(key)}
                            }
                            else{
                                KeyboardButton::Unknown
                            };
                            if input.state==ElementState::Pressed{
                                KeyboardPressed(key)
                            }
                            else{
                                KeyboardReleased(key)
                            }
                        }

                        // Получение вводимых букв
                        GWindowEvent::ReceivedCharacter(character)=>{
                            match character{
                                '\u{7f}' | // Delete
                                '\u{1b}' | // Escape
                                '\u{8}'  | // Backspace
                                '\r' | '\n' | '\t'=>None,
                                ch=>CharacterInput(ch),
                            }
                        }

                        GWindowEvent::Focused(_)=>unsafe{
                            window.set_minimized(true); // Сворацивание окна

                            (*game_window).events_handler=GameWindow::wait_until_focused; // Смена фукции обработки событий
                            *control_flow=ControlFlow::Exit; // Флаг завершения цикла обработки событий

                            WindowEvent::Hide(true) // Передача события во внешнее управление
                        }
                        _=>None // Игнорирование остальных событий
                    }
                }

                // Создание кадра и запрос на его вывод на окно
                Event::MainEventsCleared=>{
                    window.request_redraw();
                    None
                }

                // Вывод кадра на окно
                Event::RedrawRequested(_)=>{
                    Draw
                }

                // После вывода кадра
                Event::RedrawEventsCleared=>{
                    *control_flow=ControlFlow::Exit;
                    None
                } // Игнорирование

                // Закрытия цикла обработки событий
                Event::LoopDestroyed=>None, // Игнорирование

                _=>None  // Игнорирование остальных событий
            };

            unsafe{(*vec).push_back(next_event)}
        });
    }

    // Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    fn wait_until_focused(&mut self){
        let vec=&mut self.events as *mut VecDeque<WindowEvent>;

        let game_window=self as *mut GameWindow;

        let display=self.display.gl_window();

        let window:&Window=display.window();

        self.event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            match event{
                Event::WindowEvent{event,..}=>{
                    match event{
                        GWindowEvent::CloseRequested=>unsafe{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            (*vec).push_back(Exit); // Передача события во внешнее управление
                        }

                        GWindowEvent::Focused(_)=>unsafe{
                            (*game_window).events_handler=GameWindow::event_listener; // Смена фукции обработки событий
                            window.set_minimized(false);

                            *control_flow=ControlFlow::Exit; // Остановка цикла обработки событий

                            (*vec).push_back(Hide(false)); // Передача события во внешнее управление
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        })
    }
}

// Функции для сглаживания
impl GameWindow{
    pub fn set_alpha(&mut self,alpha:f32){
        self.alpha_channel=alpha;
    }

    pub fn set_smooth(&mut self,smooth:f32){
        self.smooth=smooth
    }

    pub fn set_new_smooth(&mut self,smooth:f32){
        self.alpha_channel=0f32;
        self.smooth=smooth
    }
}

// Функции для рисования
impl GameWindow{
    // Даёт прямое управление буфером кадра
    pub fn draw_raw<F:FnOnce(&mut Frame)>(&mut self,f:F){
        let mut frame=self.display().draw();
        f(&mut frame);
        frame.finish();
    }

    pub fn draw<F:FnOnce(&mut DrawParameters,&mut GameGraphics)>(&mut self,f:F){
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=GameGraphics::new(&mut self.graphics,&mut frame);

        f(&mut draw_parameters,&mut g);

        unsafe{
            let mouse_position=mouse_cursor.position();
            let dx=(mouse_position[0]/window_center[0])-1f32;
            let dy=1f32-(mouse_position[1]/window_center[1]);
            self.mouse_icon.draw((dx,dy),&mut draw_parameters,g.frame());
        }

        frame.finish();
    }

    // Рисует курсор и выполняет замыкание f
    // Выдаёт изменяющийся альфа-канал для рисования, возвращает следующее значение альфа-канала
    pub fn draw_smooth<F:FnOnce(f32,&mut DrawParameters,&mut GameGraphics)>(&mut self,f:F)->f32{
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=GameGraphics::new(&mut self.graphics,&mut frame);

        f(self.alpha_channel,&mut draw_parameters,&mut g);

        unsafe{
            let mouse_position=mouse_cursor.position();
            let dx=(mouse_position[0]/window_center[0])-1f32;
            let dy=1f32-(mouse_position[1]/window_center[1]);
            self.mouse_icon.draw((dx,dy),&mut draw_parameters,g.frame());
        }

        frame.finish();

        self.alpha_channel+=self.smooth;
        self.alpha_channel
    }
}

impl GameWindow{
    pub fn screenshot<P:AsRef<Path>>(&self,path:P){
        let image:glium::texture::RawImage2d<u8>=self.display.read_front_buffer().unwrap();
        let image=image::ImageBuffer::from_raw(image.width,image.height,image.data.into_owned()).unwrap();
        let image=image::DynamicImage::ImageRgba8(image).flipv();
        image.save(path).unwrap();
    }
}

#[derive(Clone,PartialEq)]
#[repr(u32)]
pub enum KeyboardButton{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Screenshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Backspace,
    Enter,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LeftAlt,
    LeftBracket,
    LeftControl,
    LeftShift,
    LeftWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RightAlt,
    RightBracket,
    RightControl,
    RightShift,
    RightWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
    Unknown,
}

// Загрузка иконки окна
pub fn load_window_icon()->Icon{
    let image=image::open("./resources/images/window_icon.png").unwrap();
    let vec=image.to_bytes();
    let width=image.width();
    let height=image.height();

    Icon::from_rgba(vec,width,height).unwrap()
}

// Обычные параметры для рисования
pub fn default_draw_parameters<'a>()->DrawParameters<'a>{
    let mut draw_parameters=DrawParameters::default();

    draw_parameters.blend=Blend{
        color:BlendingFunction::Addition{
            source:LinearBlendingFactor::SourceAlpha,
            destination:LinearBlendingFactor::OneMinusSourceAlpha,
        },
        alpha:BlendingFunction::Addition{
            source:LinearBlendingFactor::One,
            destination:LinearBlendingFactor::One,
        },
        constant_value:(0.0,0.0,0.0,0.0),
    };

    draw_parameters.backface_culling=BackfaceCullingMode::CullingDisabled;

    draw_parameters
}