#![allow(unused_imports)]
use super::{
    graphics::{Graphics2D,Graphics,GraphicsSettings},
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
    Version,
    draw_parameters::{
        DrawParameters,
        Blend,
        BlendingFunction,
        LinearBlendingFactor,
        BackfaceCullingMode,
    },
    texture::RawImage2d,
    backend::glutin::DisplayCreationError
};

use glium::glutin::{
    event::{
        Event,
        WindowEvent as GWindowEvent,
        MouseButton as GMouseButton,
        ElementState,
        ModifiersState,
    },
    event_loop::{ControlFlow,EventLoop},
    window::{WindowBuilder,Fullscreen},
    ContextBuilder,
    platform::desktop::EventLoopExtDesktop,
    NotCurrent,
    monitor::MonitorHandle,
};

use image::{
    ImageFormat,
    ImageBuffer,
    DynamicImage
};



pub static mut mouse_cursor:MouseCursor=MouseCursor::new(); // Положение курсора мыши

pub static mut window_width:f32=0f32;
pub static mut window_height:f32=0f32;
pub static mut window_center:[f32;2]=[0f32;2]; // Центр окна

/// Окно с вписанными в него графическими функциями,
/// а также обработчиками событий.
/// 
/*
    EventLoop - минимум четыре шага для моей схемы с мгновенным закрытием цикла обработки событий:
    1) NewEvent
    2) MainEventsCleared
    (RedrawRequested всегда идет после MainEventsCleared)
    3) RedrawEventsCleared
    4) LoopDestroyed
*/

/// Все события обрабатываются и добавляются в очередь внешней обработки (Window.events)
/// для работы с ними вне структуры окна.
/// 
/// При потере фокуса окно сворачивается,
/// передача событий внешнему управлению прекращается (передаётся только событие о получении фокуса).
/// При получении фокуса окно возвращается в исходное состояние.

pub struct Window{
    display:Display,
    graphics:Graphics2D,

    event_loop:EventLoop<()>,
    events:VecDeque<WindowEvent>,
    events_handler:fn(&mut Self),

    alpha_channel:f32,  // Для плавных
    smooth:f32,         // переходов

    #[cfg(feature="mouse_cursor_icon")]
    mouse_icon:MouseCursorIcon,

    // Поля для отладки
    #[cfg(debug_assertions)]
    focusable_option:bool, // Включение/отключение возможности сворачивания во время отладки
}

/// Внешние события окна
#[derive(Clone)]
pub enum WindowEvent{
    None,
    Draw,

    /// Получение/потеря фокуса, true/false
    /// 
    /// При потере фокуса игра сворачивается
    Hide(bool),

    /// Изменение размера окна
    Resize([u32;2]),

    /// Сдвиг мышки (сдвиг за пределы экрана игнорируется)
    MouseMovementDelta([f32;2]),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),

    KeyboardPressed(KeyboardButton),
    KeyboardReleased(KeyboardButton),
    CharacterInput(char),

    ModifiersChanged(ModifiersState),

    Exit,
}

/// Кнопки мыши, без дополнительных кнопок
#[derive(Clone)]
pub enum MouseButton{
    Left,
    Middle,
    Right,
}

use WindowEvent::*;

impl Window{
    /// Создание окна с функцией настройки
    ///
    /// Create new window with setting function
    pub fn new<F>(setting:F)->Result<Window,DisplayCreationError>
        where
            F:FnOnce(Vec<MonitorHandle>,&mut WindowBuilder,&mut ContextBuilder<NotCurrent>,&mut GraphicsSettings){
        let event_loop=EventLoop::new();
        let monitors=event_loop.available_monitors().collect();

        let mut graphics_settings=GraphicsSettings::new();
        let mut window_builder=WindowBuilder::new();
        let mut context_builder=ContextBuilder::new();

        // Настройка
        setting(monitors,&mut window_builder,&mut context_builder,&mut graphics_settings);

        // Создание окна и привязывание графической библиотеки
        let display=Display::new(window_builder,context_builder,&event_loop)?;

        let size=display.gl_window().window().inner_size();
        unsafe{
            window_width=size.width as f32;
            window_height=size.height as f32;
            window_center=[window_width/2f32,window_height/2f32];
        }

        // Опреление поддерживаемой версии GLSL
        let Version(..,m,l)=display.get_supported_glsl_version();
        let glsl=match m{
            1 if l<3 =>{
                120
            }
            _=>{
                140
            }
        };

        let mut frame=display.draw();       //
        frame.clear_color(1.0,1.0,1.0,1.0); // Заполнение окна
        frame.finish().unwrap();            //

        // Отлючение курсора системы
        // Замена его собственным
        #[cfg(feature="mouse_cursor_icon")]
        display.gl_window().window().set_cursor_visible(false);

        Ok(Self{
            #[cfg(feature="mouse_cursor_icon")]
            mouse_icon:MouseCursorIcon::new(&display),

            graphics:Graphics2D::new(&display,graphics_settings,glsl),
            display:display,

            event_loop,
            events:VecDeque::with_capacity(32),
            events_handler:Window::event_listener,

            alpha_channel:0f32,
            smooth:0f32,

            #[cfg(debug_assertions)]
            focusable_option:true,
        })
    }

    #[inline(always)]
    pub fn display(&self)->&Display{
        &self.display
    }

    #[inline(always)]
    pub fn graphics(&mut self)->&mut Graphics2D{
        &mut self.graphics
    }

    #[inline(always)]
    pub fn available_monitors(&self)->impl std::iter::Iterator<Item=MonitorHandle>{
        self.event_loop.available_monitors()
    }

    /// Следующее событие окна
    ///
    /// Next window event
    pub fn next_event(&mut self)->Option<WindowEvent>{
        if self.events.is_empty(){
            (self.events_handler)(self); // Вызов функции обработки событий
        }
        self.events.pop_front()
    }

    pub fn choose_fullscreen_monitor(&self,monitor:usize)->Result<(),()>{
        if let Some(m)=self.available_monitors().nth(monitor){
            self.display.gl_window().window().set_fullscreen(Some(Fullscreen::Borderless(m)));
            Ok(())
        }
        else{
            Err(())
        }
        
    }

    pub fn disable_fullscreen(&self){
        self.display.gl_window().window().set_fullscreen(Option::None)
    }

    pub fn set_fullscreen(&self,fullscreen:Fullscreen){
        self.display.gl_window().window().set_fullscreen(Some(fullscreen))
    }

    /// Спрятать окно
    ///
    /// Hide the window
    #[inline(always)]
    pub fn set_hide(&self,hide:bool){
        self.display.gl_window().window().set_minimized(hide);
    }

    #[inline(always)]
    pub fn set_cursor_visible(&mut self,visible:bool){
        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.set_visible(visible);
        #[cfg(not(feature="mouse_cursor_icon"))]
        self.display.gl_window().window().set_cursor_visible(visible);
    }

    #[cfg(feature="mouse_cursor_icon")]
    #[inline(always)]
    pub fn switch_cursor_visibility(&mut self){
        self.mouse_icon.switch_visibility()
    }
}

// Связанное с версиями OpenGL
impl Window{
    #[inline(always)]
    pub fn get_supported_glsl_version(&self)->Version{
        self.display.get_supported_glsl_version()
    }
    #[inline(always)]
    pub fn get_opengl_version(&self)->&Version{
        self.display.get_opengl_version()
    }
}

// Функции для сглаживания
impl Window{
    /// Set alpha channel for smooth drawing
    pub fn set_alpha(&mut self,alpha:f32){
        self.alpha_channel=alpha;
    }

    /// Set smooth for smooth drawing
    pub fn set_smooth(&mut self,smooth:f32){
        self.smooth=smooth
    }

    /// Set smooth and zero alpha channel
    /// for smooth drawing
    pub fn set_new_smooth(&mut self,smooth:f32){
        self.alpha_channel=0f32;
        self.smooth=smooth
    }
}

// Функции для рисования
impl Window{
    /// Даёт прямое управление буфером кадра
    pub fn draw_raw<F:FnOnce(&mut Frame)>(&self,f:F){
        let mut frame=self.display().draw();
        f(&mut frame);
        frame.finish();
    }

    /// Выполняет замыкание и рисует курсор
    pub fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F){
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=Graphics::new(&self.graphics,&mut frame);

        f(&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish();
    }

    /// Выполняет замыкание и рисует курсор
    /// 
    /// Нужна для правных переходов с помощью альфа-канала
    /// 
    /// Выдаёт изменяющийся альфа-канал для рисования, возвращает следующее значение альфа-канала
    pub fn draw_smooth<F:FnOnce(f32,&mut DrawParameters,&mut Graphics)>(&mut self,f:F)->f32{
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=Graphics::new(&mut self.graphics,&mut frame);

        f(self.alpha_channel,&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish();

        self.alpha_channel+=self.smooth;
        self.alpha_channel
    }

    /// Игнорирует все события, кроме рендеринга и закрытия окна
    /// 
    /// Рисует один кадр
    pub fn draw_event_once<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&mut self,f:F)->WindowEvent{
        while let Some(event)=self.next_event(){
            match event{
                WindowEvent::Exit=>return WindowEvent::Exit, // Закрытие игры
                WindowEvent::Draw=>{ //Рендеринг
                    self.draw(f);
                    break
                }
                _=>{}
            }
        }
        WindowEvent::None
    }
}

// Дополнительные функции
impl Window{
    /// Сохраняет скриншот в формате png
    /// 
    /// Save screenshot in png format
    pub fn screenshot<P:AsRef<Path>>(&self,path:P){
        // Копирование буфера окна
        let image:RawImage2d<u8>=match self.display.read_front_buffer(){
            Ok(t)=>t,
            Err(_)=>return
        };
        // Перевод в буфер изображения
        let image=match ImageBuffer::from_raw(image.width,image.height,image.data.into_owned()){
            Option::Some(i)=>i,
            Option::None=>return
        };
        // Перевод в изображение
        let image=DynamicImage::ImageRgba8(image).flipv();
        // Сохранение
        if let Err(_)=image.save_with_format(path,ImageFormat::Png){
            return
        }
    }
}

//                         \\
//    ЛОКАЛЬНЫЕ ФУНКЦИИ    \\
//                         \\

impl Window{
    #[inline(always)]
    fn request_redraw(&self){
        self.display.gl_window().window().request_redraw();
    }
}

// Функции обработки событий
impl Window{
    /// Обычная функция обработки событий
    fn event_listener(&mut self){
        let el=&mut self.event_loop as *mut EventLoop<()>;
        let event_loop=unsafe{&mut *el};

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;
            let next_event=match event{
                Event::NewEvents(_)=>None, // Игнорирование

                // События окна
                Event::WindowEvent{event,..}=>{
                    match event{
                        // Закрытие окна
                        GWindowEvent::CloseRequested=>Exit,

                        GWindowEvent::Resized(size)=>unsafe{
                            window_width=size.width as f32;
                            window_height=size.height as f32;
                            window_center=[window_width/2f32,window_height/2f32];
                            Resize([size.width,size.height])
                        }

                        // Сдвиг мыши (сдвиг за пределы окна игнорируется)
                        GWindowEvent::CursorMoved{position,..}=>unsafe{
                            let last_position=mouse_cursor.position();

                            let position=[position.x as f32,position.y as f32];

                            let dx=position[0]-last_position[0];
                            let dy=position[1]-last_position[1];

                            mouse_cursor.set_position(position);

                            #[cfg(feature="mouse_cursor_icon")]
                            self.mouse_icon.set_position(position);

                            MouseMovementDelta([dx,dy])
                        }

                        // Обработка действий с кнопками мыши (только стандартные кнопки)
                        GWindowEvent::MouseInput{button,state,..}=>{
                            if state==ElementState::Pressed{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.mouse_icon.pressed();

                                        MousePressed(MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>MousePressed(MouseButton::Middle),
                                    GMouseButton::Right=>MousePressed(MouseButton::Right),
                                    GMouseButton::Other(_)=>None
                                }
                            }
                            else{
                                match button{
                                    GMouseButton::Left=>{
                                        #[cfg(feature="mouse_cursor_icon")]
                                        self.mouse_icon.released();

                                        MouseReleased(MouseButton::Left)
                                    }
                                    GMouseButton::Middle=>MouseReleased(MouseButton::Middle),
                                    GMouseButton::Right=>MouseReleased(MouseButton::Right),
                                    GMouseButton::Other(_)=>None
                                }
                            }
                        }

                        // Обработка действий с клавишами клавиатуры
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
                                #[cfg(feature="mouse_cursor_icon")]
                                if key==KeyboardButton::F8{
                                    self.switch_cursor_visibility()
                                }

                                // Отключение/включение возможности сворачивания окна
                                #[cfg(debug_assertions)]{
                                    if key==KeyboardButton::F10{
                                        self.focusable_option=!self.focusable_option;
                                    }
                                }
                                KeyboardReleased(key)
                            }
                        }

                        // Получение вводимых букв
                        GWindowEvent::ReceivedCharacter(character)=>if character.is_ascii_control(){
                            None
                        }
                        else{
                            CharacterInput(character)
                        }

                        // При потере фокуса
                        GWindowEvent::Focused(f)=>if !f{
                            self.lost_focus(control_flow)
                        }
                        else{
                            WindowEvent::Hide(false) // Передача события во внешнее управление
                        }
                        _=>None // Игнорирование остальных событий
                    }
                }

                // Запрос на рендеринг
                Event::MainEventsCleared=>{
                    self.request_redraw();
                    None
                }

                // Рендеринг
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

            self.events.push_back(next_event)
        });
    }

    /// Функция ожидания получения фокуса - перехватывает управление до получения окном фокуса
    fn wait_until_focused(&mut self){
        let el=&mut self.event_loop as *mut EventLoop<()>;
        let event_loop=unsafe{&mut *el};

        event_loop.run_return(|event,_,control_flow|{
            *control_flow=ControlFlow::Wait;

            let event=match event{
                Event::WindowEvent{event,..}=>{
                    match event{
                        GWindowEvent::CloseRequested=>{ // Остановка цикла обработки событий,
                            *control_flow=ControlFlow::Exit;
                            Exit // Передача события во внешнее управление
                        }

                        // При получении фокуса
                        GWindowEvent::Focused(_)=>self.gained_focus(control_flow),

                        // Изменение флагов модификаторов (alt, shift, ctrl, logo)
                        GWindowEvent::ModifiersChanged(state)=>ModifiersChanged(state),

                        _=>None
                    }
                }
                _=>None
            };
            self.events.push_back(event);
        })
    }
}

// Функции внутренней обработки событий
impl Window{
    /// При потере фокуса - для отладки
    #[cfg(debug_assertions)]
    fn lost_focus(&mut self,control_flow:&mut ControlFlow)->WindowEvent{

        // Если функция сворачивания включена
        if self.focusable_option{
            self.display.gl_window().window().set_minimized(true); // Сворацивание окна
            self.events_handler=Window::wait_until_focused; // Смена фукции обработки событий
        }

        *control_flow=ControlFlow::Exit; // Флаг завершения цикла обработки событий

        WindowEvent::Hide(true) // Передача события во внешнее управление
    }

    /// При потере фокуса - релиз
    #[cfg(not(debug_assertions))]
    fn lost_focus(&mut self,control_flow:&mut ControlFlow)->WindowEvent{
        self.display.gl_window().window().set_minimized(true); // Сворацивание окна
        self.events_handler=Window::wait_until_focused; // Смена фукции обработки событий

        *control_flow=ControlFlow::Exit; // Флаг завершения цикла обработки событий

        WindowEvent::Hide(true) // Передача события во внешнее управление
    }

    /// При получении фокуса
    fn gained_focus(&mut self,control_flow:&mut ControlFlow)->WindowEvent{
        self.events_handler=Window::event_listener; // Смена фукции обработки событий
        self.display.gl_window().window().set_minimized(false);

        let size=self.display.gl_window().window().inner_size();
        unsafe{
            window_width=size.width as f32;
            window_height=size.height as f32;
            window_center=[window_width/2f32,window_height/2f32];
        }

        *control_flow=ControlFlow::Exit; // Остановка цикла обработки событий

        Hide(false) // Передача события во внешнее управление
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

// Обычные параметры для рисования
fn default_draw_parameters<'a>()->DrawParameters<'a>{
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