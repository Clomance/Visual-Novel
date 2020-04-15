use graphics::types::Color;

// Простые цвета

pub const White:Color=[1.0;4];
pub const Black:Color=[0.0,0.0,0.0,1.0];


// Оттенки синего \\

pub const Light_blue_1:Color=[0.4,0.5,1.0,1.0];
pub const Light_blue:Color=[0.3,0.4,1.0,1.0];
pub const Light_blue_0:Color=[0.2,0.3,1.0,1.0];
pub const Blue:Color=[0.0,0.0,1.0,1.0]; // Истинный синий, выше - светлее, ниже - темнее

// Другие оттенки синего
pub const Purple:Color=[0.85,0.4,0.8,1.0];

pub const Dark_purple:Color=[0.6,0.4,0.7,1.0];

// Оттенки красного \\

pub const Red:Color=[1.0,0.0,0.0,1.0];

// Оттенки зелёного \\

pub const Cyan:Color=[0.2,0.8,0.8,1.0]; // Бирюзовый

// Оттенки серого \\
pub const Gray:Color=[0.5,0.5,0.5,1.0]; // Истинный серый, выше - светлее, ниже - темнее
pub const Dark_gray:Color=[0.2,0.2,0.2,1.0];


// Специальные \\
pub const Head_main_menu_color:Color=White;

pub const Settings_page_color:Color=Dark_gray;

pub const Pause_menu_background_color:Color=[0.1,0.2,0.3,1.0]; // Серо-синий