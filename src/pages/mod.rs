mod loading_screen;
pub use loading_screen::LoadingScreen;

mod main_menu;
pub use main_menu::MainMenu;

mod settings;
pub use settings::Settings;

use lib::colours::Light_blue;

use cat_engine::Colour;

const button_pressed:Colour=[
    Light_blue[0]-0.05,
    Light_blue[1]-0.05,
    Light_blue[2]-0.05,
    Light_blue[3],
];

pub enum SwipeDirection{
    Up,
    Down,
    Left,
    Right,
}