use cat_engine::Colour;

// Простые цвета

pub const White:Colour=[1.0;4];
pub const Black:Colour=[0.0,0.0,0.0,1.0];


// Оттенки синего \\

pub const Light_blue_1:Colour=[0.4,0.5,1.0,1.0];
pub const Light_blue:Colour=[0.1,0.2,0.85,1.0];
pub const Blue:Colour=[0.0,0.0,1.0,1.0]; // Истинный синий, выше - светлее, ниже - темнее

// Другие оттенки синего
pub const Purple:Colour=[0.85,0.4,0.8,1.0];

pub const Dark_purple:Colour=[0.6,0.4,0.7,1.0];

// Оттенки красного \\

pub const Red:Colour=[1.0,0.0,0.0,1.0];

// Оттенки зелёного \\

pub const Cyan:Colour=[0.2,0.8,0.8,1.0]; // Бирюзовый

// Оттенки серого \\
pub const Gray:Colour=[0.5,0.5,0.5,1.0]; // Истинный серый, выше - светлее, ниже - темнее
pub const Dark_gray:Colour=[0.09,0.09,0.09,1.0];

// Bleak orange \\
pub const Bleak_orange:Colour=[1.0, 0.545, 0.349, 1.0];
// Специальные \\
pub const Head_main_menu_colour:Colour=White;

pub const Pause_menu_background_colour:Colour=[0.1,0.2,0.3,1.0]; // Серо-синий