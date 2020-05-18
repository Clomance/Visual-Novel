use crate::*;

use engine::{
    image::{ImageBase,Texture},
    graphics::GameGraphics,
    glium::{Display,DrawParameters},
};

const focused_resize:f32=4f32;
const focused_movement:f32=focused_resize/2f32;

const margin:f32=focused_resize+1f32;

const movement_scale:f32=8f32;
const focused_movement_scale:f32=10f32;

// Позиция персонажа на сцене
#[derive(Clone)]
pub enum CharacterLocation{
    Left, // Слева с краю
    LeftCenter, // Центр левой половины
    CenterLeft, // Левее центра
    Center, // Центр
    CenterRight,
    RightCenter,
    Right
}

struct Character{
    image:ImageBase,
    texture:Texture
}

impl Character{
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.image.x1+=dx/movement_scale;
        self.image.y1+=dy/movement_scale;
        self.image.x2+=dx/movement_scale;
        self.image.y2+=dy/movement_scale;
    }
}

pub struct CharactersView{
    characters:Vec<Character>,
}

impl CharactersView{
    pub fn new()->CharactersView{
        Self{
            characters:Vec::new(),
        }
    }

    pub fn add_character(&mut self,character:&RgbaImage,location:CharacterLocation,display:&mut Display){
        let rect=unsafe{
            let height=character.height() as f32;
            let width=character.width() as f32;

            let y=window_height-height;

            let x=match location{
                CharacterLocation::Left=>margin,

                CharacterLocation::LeftCenter=>(window_center[0]-width)/2f32,

                CharacterLocation::CenterLeft=>window_center[0]-width,

                CharacterLocation::Center=>(window_width-width)/2f32,

                CharacterLocation::CenterRight=>window_center[0],

                CharacterLocation::RightCenter=>window_center[0]+(window_center[0]-width)/2f32,

                CharacterLocation::Right=>window_width-width-margin
            };
            [x,y,width,height]
        };

        let character=Character{
            image:ImageBase::new(White,rect),
            texture:Texture::from_image(display,character).unwrap(),
        };

        self.characters.push(character)
    }

    pub fn clear(&mut self){
        self.characters.clear()
    }

    pub fn mouse_shift(&mut self,mut dx:f32,mut dy:f32){
        dx/=movement_scale;
        dy/=movement_scale;
        for character in &mut self.characters{
            character.shift(dx,dy)
        }
    }
}

impl Drawable for CharactersView{
    fn set_alpha_channel(&mut self,alpha:f32){
        for ch in &mut self.characters{
            ch.image.colour[3]=alpha;
        }
    }

    fn draw(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut GameGraphics){
        for ch in &mut self.characters{
            ch.image.draw(&ch.texture,draw_parameters,graphics);
        }
    }
}