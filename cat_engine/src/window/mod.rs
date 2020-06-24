mod window;
pub use window::*;

mod settings;
pub use settings::WindowSettings;

mod mouse_cursor;

use glium::glutin::event::ModifiersState;

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