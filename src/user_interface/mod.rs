mod text_view;
pub use text_view::*;

mod dialogue_box;
pub use dialogue_box::DialogueBox;

mod button;
pub use button::*;

mod menu;
pub use menu::{
    Menu,
    MenuSettings,
};

mod slider;
pub use slider::*;

// Выравнивание
#[derive(Clone)]
pub struct Align{
    x:AlignX,
    y:AlignY
}

impl Align{
    #[inline(always)]
    pub fn center()->Align{
        Self{
            x:AlignX::Center,
            y:AlignY::Center,
        }
    }

    #[inline(always)]
    pub fn position(&self,location:[f64;4],size:[f64;2])->(f64,f64){
        // Выравнивание по x
        let x=match self.x{
            AlignX::Left=>location[0],
            AlignX::Center=>location[0]+(location[2]-size[0])/2f64,
            AlignX::Right=>location[0]+location[2]-size[0],
        };
        
        // Выравнивание по y
        let y=match self.y{
            AlignY::Up=>location[1]+size[1],
            AlignY::Center=>location[1]+(location[3]+size[1])/2f64,
            AlignY::Down=>location[1]+location[3],
        };

        (x,y)
    }
}

// Тип выравнивания по x
#[derive(Clone)]
pub enum AlignX{
    Left,
    Center,
    Right
}

// Тип выравнивания по y
#[derive(Clone)]
pub enum AlignY{
    Up,
    Center,
    Down
}