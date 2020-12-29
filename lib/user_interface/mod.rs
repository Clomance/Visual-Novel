mod button;
pub use button::{
    Button,
    ButtonSettings
};

mod text_view;
pub use text_view::{
    TextView,
    TextViewSettings
};

mod edit_text_view;
pub use edit_text_view::{
    EditTextView,
    EditTextViewSettings
};

mod menu;
pub use menu::{
    Menu,
    MenuSettings
};

#[derive(Clone)]
pub struct GeneralSettings{
    /// Область для вставки объекта
    /// [x,y,width,height]
    layout:[f32;4],
}

impl GeneralSettings{
    pub fn new(layout:[f32;4])->GeneralSettings{
        Self{
            layout,
        }
    }
}