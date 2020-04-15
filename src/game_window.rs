#![allow(unused_must_use)]

use crate::{
    Settings,
    MouseCursor,
};

use opengl_graphics::{
    GlGraphics,
    OpenGL
};

use glutin::{
    event::{
        Event,
        WindowEvent,
        MouseButton as GMouseButton,
        ElementState,
    },
    event_loop::{ControlFlow,EventLoop},
    window::{Window,WindowBuilder,Fullscreen},
    ContextBuilder,
    ContextWrapper,
    PossiblyCurrent,
    GlRequest,
    dpi::PhysicalPosition,
};

use winit::platform::desktop::EventLoopExtDesktop;

use graphics::{
    Viewport,
    Context,
    Graphics
};

use std::collections::VecDeque;
/*
    EventLoop - минимум четыре шага для моей схемы с мгновенным закрытием цикла обработки событий:
    1) NewEvent
    2) MainEventsCleared
    (RedrawRequested всегда идет после MainEventsCleared)
    3) RedrawEventsCleared
    4) LoopDestroyed


    Все события обрабатываются и добавляются в очередь внешней обработки (GameWindow.events)
        для работы с ними вне структуры окна
*/

pub const openGL:OpenGL=OpenGL::V3_2;

pub static mut window_width:f64=0f64;
pub static mut window_height:f64=0f64;
pub static mut window_center:[f64;2]=[0f64;2];

pub static mut mouse_cursor:MouseCursor=MouseCursor::new();

pub struct GameWindow{
    event_loop:EventLoop<()>,
    context:ContextWrapper<PossiblyCurrent,Window>,
    graphics:GlGraphics,
    events:VecDeque<GameWindowEvent>,
    width:u32,
    height:u32,
}

#[derive(Clone)]
pub enum GameWindowEvent{
    None,
    Update,
    Draw,
    MouseMovement((f64,f64)),

    KeyboardPressed(KeyboardButton),
    KeyboardReleased(KeyboardButton),

    MousePressed(MouseButton),
    MouseReleased(MouseButton),

    CharacterInput(char),
    Exit,
}

#[derive(Clone)]
pub enum MouseButton{
    Left,
    Middle,
    Right,
}

use GameWindowEvent::*;

impl GameWindow{
    pub fn new()->GameWindow{
        let event_loop=EventLoop::new();
        let monitor=event_loop.primary_monitor();
        let size=monitor.size();

        unsafe{
            window_width=size.width as f64;
            window_height=size.height as f64;
            window_center=[window_width/2f64,window_height/2f64];
        }
        let mut inner=size;
        inner.width=0;
        inner.height=0;

        let window_builder=WindowBuilder::new()
            .with_inner_size(inner) // Чтобы не вылезало не дорисованное окно
            .with_decorations(false)
            .with_resizable(false)
            .with_always_on_top(false)
            .with_title(unsafe{&Settings.game_name})
            .with_fullscreen(Some(Fullscreen::Borderless(monitor)));

        let api=openGL.get_major_minor();

        let builder=ContextBuilder::new()
            .with_gl(GlRequest::GlThenGles{
                opengl_version:(api.0 as u8,api.1 as u8),
                opengles_version:(api.0 as u8,api.1 as u8),
            })
            .with_vsync(true)
            .with_srgb(true);

        let ctx=builder.build_windowed(window_builder,&event_loop).unwrap();
        let ctx=unsafe{ctx.make_current().unwrap()};

        ctx.window().set_cursor_visible(false);

        gl::load_with(|s|ctx.get_proc_address(s) as *const _);

        let mut gl=GlGraphics::new(openGL);

        let width=unsafe{window_width as u32};
        let height=unsafe{window_height as u32};
        let viewport=Viewport{
            rect:[0,0,width as i32,height as i32],
            draw_size:[width,height],
            window_size:unsafe{[window_width,window_height]},
        };

        gl.draw(viewport,|_,g|{
            g.clear_color([1.0;4])
        });

        Self{
            event_loop:event_loop,
            context:ctx,
            graphics:gl,
            events:VecDeque::with_capacity(6),
            width:width,
            height:height,
        }
    }

    pub fn window(&self)->&Window{
        self.context.window()
    }

    pub fn next_event(&mut self)->Option<GameWindowEvent>{
        if self.events.is_empty(){
            let vec=&mut self.events as *mut VecDeque<GameWindowEvent>;

            let window=self as *mut GameWindow;

            self.event_loop.run_return(|event,_,control_flow|{
                *control_flow=ControlFlow::Wait;

                let next_event=match event{
                    Event::NewEvents(_)=>None, // Игнорирование

                    // События окна
                    Event::WindowEvent{event,..}=>{
                        match event{
                            // Закрытие окна
                            WindowEvent::CloseRequested=>Exit,

                            // Движение мыши (конечное положение)
                            WindowEvent::CursorMoved{position,..}=>MouseMovement((position.x,position.y)),
                            
                            // Обработка действий с кнопками мыши (только стандартные кнопки)
                            WindowEvent::MouseInput{button,state,..}=>{
                                if state==ElementState::Pressed{
                                    match button{
                                        GMouseButton::Left=>MousePressed(MouseButton::Left),
                                        GMouseButton::Middle=>MousePressed(MouseButton::Middle),
                                        GMouseButton::Right=>MousePressed(MouseButton::Right),
                                        GMouseButton::Other(_)=>None
                                    }
                                }
                                else{
                                    match button{
                                        GMouseButton::Left=>MouseReleased(MouseButton::Left),
                                        GMouseButton::Middle=>MouseReleased(MouseButton::Middle),
                                        GMouseButton::Right=>MouseReleased(MouseButton::Right),
                                        GMouseButton::Other(_)=>None
                                    }
                                }
                            }

                            WindowEvent::KeyboardInput{input,..}=>{
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
                            WindowEvent::ReceivedCharacter(character)=>{
                                match character{
                                    '\u{7f}' | // Delete
                                    '\u{1b}' | // Escape
                                    '\u{8}'  | // Backspace
                                    '\r' | '\n' | '\t'=>None,
                                    ch=>CharacterInput(ch),
                                }
                            }

                            WindowEvent::Focused(_focused)=>{
                                None
                            }
                            _=>None // Игнорирование остальных событий
                        }
                    }

                    // Создание кадра и запрос на его вывод на окно
                    Event::MainEventsCleared=>{
                        unsafe{
                            (*window).request_redraw()
                        }
                        None
                    }

                    Event::Suspended=>{
                        None
                    }

                    // Вывод кадра на окно
                    Event::RedrawRequested(_)=>{
                        unsafe{(*window).redraw()};
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

            self.events.pop_front()
        }
        else{
            self.events.pop_front()
        }
    }

    pub fn draw<F>(&mut self,f:F)
            where F: FnOnce(Context,&mut GlGraphics){

        let viewport=Viewport{
            rect:[0,0,self.width as i32,self.height as i32],
            draw_size:[self.width,self.height],
            window_size:unsafe{[window_width,window_height]},
        };

        let context=self.graphics.draw_begin(viewport);

        f(context,&mut self.graphics);

        self.graphics.draw_end();
    }

    pub fn request_redraw(&self){
        self.context.window().request_redraw();
    }

    pub fn redraw(&self){
        self.context.swap_buffers().unwrap();
    }

    pub fn set_cursor_position(&self,position:[f64;2]){
        let position=PhysicalPosition{
            x:position[0],
            y:position[1]
        };
        self.context.window().set_cursor_position(position);
    }
}

#[derive(Clone)]
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