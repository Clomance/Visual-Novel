use super::{
    graphics::{Graphics2D,Graphics},
    mouse_cursor::MouseCursor,
};

#[cfg(feature="mouse_cursor_icon")]
use super::mouse_cursor::MouseCursorIcon;

mod settings;
pub use settings::WindowSettings;

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
    monitor::MonitorHandle,
    event_loop::{ControlFlow,EventLoop},
    event::{
        Event,
        WindowEvent as GWindowEvent,
        MouseButton as GMouseButton,
        ElementState,
        ModifiersState,
    },
    window::Fullscreen,
    platform::desktop::EventLoopExtDesktop,
};

use image::{
    ImageFormat,
    ImageBuffer,
    DynamicImage
};


/// Положение курсора мыши. The mouse cursor position.
pub static mut mouse_cursor:MouseCursor=MouseCursor::new();

/// Ширина окна. The window width.
pub static mut window_width:f32=0f32;
/// Высота окна. The window height.
pub static mut window_height:f32=0f32;
/// Центр окна. The window center. [x, y]
pub static mut window_center:[f32;2]=[0f32;2];

/// Окно, включает в себя графические функциями
/// и обработчик событий.
/// Window with graphic functions
/// and an event listener included.
/// 
/*
    EventLoop - минимум четыре шага:
    1) NewEvent
    (Основные события)
    2) MainEventsCleared
    (RedrawRequested)
    3) RedrawEventsCleared
    4) LoopDestroyed
*/

/// Все события обрабатываются и добавляются в очередь внешней обработки (Window.events)
/// для работы с ними вне структуры окна.
/// 
/// # feature = "auto_hide"
/// При потере фокуса окно сворачивается,
/// передача событий внешнему управлению прекращается (передаётся только событие о получении фокуса).
/// При получении фокуса окно возвращается в исходное состояние.
///
/// All events are handled and added to the outer handling queue (Window.events)
/// to work with them outside of the window structure.
/// 
/// # feature = "auto_hide"
/// The window gets minimized when it loses focus and
/// it stops sending outer events, except gained focus event.
/// The window gets back when it gains focus.

pub struct Window{
    display:Display,
    graphics:Graphics2D,

    event_loop:EventLoop<()>,
    events:VecDeque<WindowEvent>,

    #[cfg(feature="auto_hide")]
    events_handler:fn(&mut Self),

    alpha_channel:f32,  // Для плавных
    smooth:f32,         // переходов

    #[cfg(feature="mouse_cursor_icon")]
    mouse_icon:MouseCursorIcon,
}

/// Внешние события окна.
/// 
/// Outer window events.
#[derive(Clone)]
pub enum WindowEvent{
    None,
    Draw,

    /// Окно свёрнуто.
    /// 
    /// The window minimized.
    /// 
    /// feature = "auto_hide"
    #[cfg(feature="auto_hide")]
    Hide(bool),


    /// Окно получило или потеряло фокус.
    /// 
    /// True - получило, false - потеряло.
    /// 
    /// The window gained or lost focus.
    /// The parameter is true, if the window has gained focus,
    /// and false, if it has lost focus.
    /// 
    /// feature != "auto_hide"
    #[cfg(not(feature="auto_hide"))]
    Focused(bool),

    /// Размера окна изменён.
    /// 
    /// The window resized.
    Resize([u32;2]),

    /// Сдвиг мышки (сдвиг за пределы экрана игнорируется).
    /// 
    /// Mouse movement (moving beyond the window border is ignored).
    MouseMovementDelta([f32;2]),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),

    KeyboardPressed(KeyboardButton),
    KeyboardReleased(KeyboardButton),
    CharacterInput(char),

    /// Shift, Ctrl, Alt или Logo нажаты.
    /// 
    /// Shift, Ctrl, Alt or Logo pressed.
    ModifiersChanged(ModifiersState),

    Exit,
}

/// Кнопки мыши, без дополнительных кнопок.
/// 
/// Mouse buttons without additional buttons.
#[derive(Clone)]
pub enum MouseButton{
    Left,
    Middle,
    Right,
}

use WindowEvent::*;

impl Window{
    //pub fn new_settings(settigs:WindowSettings)->Result<Window,DisplayCreationError>{}

    /// Создаёт окно. Принимает функцию для настройки.
    ///
    /// Creates the window.
    pub fn new<F>(setting:F)->Result<Window,DisplayCreationError>
        where
            F:FnOnce(Vec<MonitorHandle>,&mut WindowSettings){
        let event_loop=EventLoop::new();
        let monitors=event_loop.available_monitors().collect();

        let mut window_settings=WindowSettings::new();
        

        // Настройка
        setting(monitors,&mut window_settings);

        let initial_colour=window_settings.initial_colour;

        let (window_builder,context_builder,graphics_settings)
                =window_settings.devide();

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

        if let Some([r,g,b,a])=initial_colour{
            let mut frame=display.draw();   //
            frame.clear_color(r,g,b,a);     // Заполнение окна
            frame.finish().unwrap();        //
        }

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

            #[cfg(feature="auto_hide")]
            events_handler:Window::event_listener,

            alpha_channel:0f32,
            smooth:0f32,
        })
    }

    #[inline(always)]
    pub fn display(&self)->&Display{
        &self.display
    }

    /// Возвращает графическую основу.
    /// 
    /// Returns graphic base.
    #[inline(always)]
    pub fn graphics(&mut self)->&mut Graphics2D{
        &mut self.graphics
    }

    #[inline(always)]
    pub fn available_monitors(&self)->impl std::iter::Iterator<Item=MonitorHandle>{
        self.event_loop.available_monitors()
    }

    /// Возвращает следующее событие окна.
    /// 
    /// Returns next window event.
    pub fn next_event(&mut self)->Option<WindowEvent>{
        if self.events.is_empty(){
            #[cfg(feature="auto_hide")]
            (self.events_handler)(self); // Вызов функции обработки событий

            #[cfg(not(feature="auto_hide"))]
            self.event_listener();
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


    pub fn set_fullscreen(&self,fullscreen:Option<Fullscreen>){
        self.display.gl_window().window().set_fullscreen(fullscreen)
    }

    /// Сворачивает окно.
    /// 
    /// Minimizes the window.
    #[inline(always)]
    pub fn set_minimized(&self,minimized:bool){
        self.display.gl_window().window().set_minimized(minimized);
    }

    /// Делает окно максимального размера.
    /// 
    /// Maximizes the window.
    #[inline(always)]
    pub fn set_maximized(&self,maximized:bool){
        self.display.gl_window().window().set_maximized(maximized);
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

/// Связанное с версиями OpenGL.
/// 
/// OpenGL versions.
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

/// Функции для сглаживания.
/// 
/// Functions for smoothing.
impl Window{
    /// Set alpha channel for smooth drawing.
    pub fn set_alpha(&mut self,alpha:f32){
        self.alpha_channel=alpha;
    }

    /// Set smooth for smooth drawing.
    pub fn set_smooth(&mut self,smooth:f32){
        self.smooth=smooth
    }

    /// Set smooth and zero alpha channel
    /// for smooth drawing.
    pub fn set_new_smooth(&mut self,smooth:f32){
        self.alpha_channel=0f32;
        self.smooth=smooth
    }
}

/// Функции для рисования.
/// Drawing functions.
impl Window{
    /// Даёт прямое управление буфером кадра.
    /// 
    /// Gives frame to raw drawing.
    pub fn draw_raw<F:FnOnce(&mut DrawParameters,&mut Frame)>(&self,f:F){
        let mut frame=self.display().draw();
        let mut draw_parameters=default_draw_parameters();
        f(&mut draw_parameters,&mut frame);
        frame.finish();
    }

    /// Выполняет замыкание (и рисует курсор, если `feature="mouse_cursor_icon"`).
    /// 
    /// Executes closure (and draws the mouse cursor, if `feature="mouse_cursor_icon"`).
    pub fn draw<F:FnOnce(&mut DrawParameters,&mut Graphics)>(&self,f:F){
        let mut draw_parameters=default_draw_parameters();

        let mut frame=self.display().draw();

        let mut g=Graphics::new(&self.graphics,&mut frame);

        f(&mut draw_parameters,&mut g);

        #[cfg(feature="mouse_cursor_icon")]
        self.mouse_icon.draw(&mut draw_parameters,&mut g);

        frame.finish();
    }

    /// Выполняет замыкание (и рисует курсор, если `feature="mouse_cursor_icon"`).
    /// Выдаёт альфа-канал для рисования, возвращает следующее значение канала.
    /// 
    /// Нужна для плавных переходов или размытия с помощью альфа-канала.
    /// 
    /// Executes closure (and draws the mouse cursor, if `feature="mouse_cursor_icon"`).
    /// Gives alpha channel for drawing, returns next value of the channel.
    /// 
    /// Needed for smooth drawing or smoothing with alpha channel.
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

    /// Игнорирует все события, кроме рендеринга и закрытия окна, и рисует один кадр.
    /// 
    /// Ignores all events, except rendering and closing the window, and draws one frame.
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

/// Дополнительные функции.
/// 
/// Additional functions.
impl Window{
    /// Возвращает скриншот.
    /// 
    /// Returns a screenshot.
    pub fn screenshot(&self)->Option<DynamicImage>{
        // Копирование буфера окна
        let image:RawImage2d<u8>=match self.display.read_front_buffer(){
            Ok(t)=>t,
            Err(_)=>return Option::None
        };
        // Перевод в буфер изображения
        let image=match ImageBuffer::from_raw(image.width,image.height,image.data.into_owned()){
            Option::Some(i)=>i,
            Option::None=>return Option::None
        };
        // Перевод в изображение
        Some(DynamicImage::ImageRgba8(image).flipv())
    }
    /// Сохраняет скриншот в формате png.
    /// 
    /// Saves a screenshot in png format.
    pub fn save_screenshot<P:AsRef<Path>>(&self,path:P){
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

/// Функции обработки событий.
/// 
/// Event handlers.
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
                        #[cfg(feature="auto_hide")]
                        GWindowEvent::Focused(f)=>if !f{
                            self.lost_focus(control_flow)
                        }
                        else{
                            WindowEvent::Hide(false) // Передача события во внешнее управление
                        }

                        #[cfg(not(feature="auto_hide"))]
                        GWindowEvent::Focused(f)=>WindowEvent::Focused(f),

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
    #[cfg(feature="auto_hide")]
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

/// Функции внутренней обработки событий.
/// 
/// Inner event handling functions.
impl Window{
    #[cfg(feature="auto_hide")]
    fn lost_focus(&mut self,control_flow:&mut ControlFlow)->WindowEvent{
        self.display.gl_window().window().set_minimized(true); // Сворацивание окна
        self.events_handler=Window::wait_until_focused; // Смена фукции обработки событий

        *control_flow=ControlFlow::Exit; // Флаг завершения цикла обработки событий

        WindowEvent::Hide(true) // Передача события во внешнее управление
    }

    /// При получении фокуса
    #[cfg(feature="auto_hide")]
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