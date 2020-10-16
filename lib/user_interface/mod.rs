use crate::{
    //
    colours::*,
    Align,
    AlignX,
    AlignY,
};

use cat_engine::graphics::DrawType;

mod text_view;
pub use text_view::{
    TextView,
    TextViewSettings
};

// mod edit_text_view;
// pub use edit_text_view::*;

mod button;
pub use button::{
    Button,
    ButtonSettings
};

mod menu;
pub use menu::{
    Menu,
    MenuSettings,
};

// mod slider;
// pub use slider::*;

// mod list;
// pub use list::*;
#[derive(Clone)]
pub struct GeneralSettings{
    draw_type:DrawType,
}

impl GeneralSettings{
    pub fn new()->GeneralSettings{
        Self{
            draw_type:DrawType::Common,
        }
    }

    pub const fn draw_type(mut self,draw_type:DrawType)->GeneralSettings{
        self.draw_type=draw_type;
        self
    }
}