#[cfg(feature="mouse_cursor_icon")]
use crate::mouse_cursor::MouseCursorIcon;

use crate::{
    graphics::{Graphics2D,Graphics},
    mouse_cursor::MouseCursor,
};

use super::{
    WindowEvent,
    WindowSettings,
    MouseButton,
    KeyboardButton,
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
    },
    window::Fullscreen,
    platform::desktop::EventLoopExtDesktop,
    dpi::Size,
};

use image::{
    ImageFormat,
    ImageBuffer,
    DynamicImage
};

use std::{
    collections::VecDeque,
    path::Path,
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
/// All events are handled and added to the outer handling queue (Window.events)
/// to work with them outside of the window structure.
/// 
/// # feature = "mouse_cursor_icon"
/// 
/// Замена обычного курсора своим.
/// Путь для картинки по умолчанию - `./mouse_cursor_icon.png`.
/// 
/// Replaces default mouse cursor with users one.
/// It uses image to draw the cursor.
/// 
/// The cursor points to the center of cursor image.
/// The default path to the image is `./mouse_cursor_icon.png`.
/// 
/// # feature = "auto_hide"
/// 
/// При потере фокуса окно сворачивается,
/// передача событий внешнему управлению прекращается (передаётся только событие о получении фокуса).
/// При получении фокуса окно возвращается в исходное состояние.
/// 
/// The window gets minimized when loses focus and
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
        #[cfg(feature="mouse_cursor_icon")]
        let mouse_icon_path=window_settings.mouse_cursor_icon_path.clone();


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
            mouse_icon:MouseCursorIcon::new(&display,mouse_icon_path),

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
}

impl Window{
    pub fn set_inner_size<S:Into<Size>>(&self,size:S){
        self.display.gl_window().window().set_inner_size(size)
    }

    pub fn set_min_inner_size<S:Into<Size>>(&self,size:Option<S>){
        self.display.gl_window().window().set_min_inner_size(size)
    }

    pub fn set_max_inner_size<S:Into<Size>>(&self,size:Option<S>){
        self.display.gl_window().window().set_max_inner_size(size)
    }

    pub fn set_title(&self,title:&str){
        self.display.gl_window().window().set_title(title)
    }

    pub fn set_visible(&self,visible:bool){
        self.display.gl_window().window().set_visible(visible)
    }

    pub fn set_resizable(&self,resizable:bool){
        self.display.gl_window().window().set_resizable(resizable)
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
        self.display.gl_window().window().set_minimized(minimized)
    }

    /// Делает окно максимального размера.
    /// 
    /// Maximizes the window.
    #[inline(always)]
    pub fn set_maximized(&self,maximized:bool){
        self.display.gl_window().window().set_maximized(maximized)
    }

    pub fn set_decorations(&self,decorations:bool){
        self.display.gl_window().window().set_decorations(decorations)
    }

    pub fn set_always_on_top(&self,always_on_top:bool){
        self.display.gl_window().window().set_always_on_top(always_on_top)
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