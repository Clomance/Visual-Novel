use crate::*;

const focused_resize:f64=4f64;
const focused_movement:f64=focused_resize/2f64;

const margin:f64=focused_resize+1f64;

const movement_scale:f64=8f64;
const focused_movement_scale:f64=10f64;

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
    pub fn shift(&mut self,dx:f64,dy:f64){
        self.image.rect[0]+=dx;
        self.image.rect[1]+=dy;
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

    pub fn add_character(&mut self,character:&RgbaImage,location:CharacterLocation){
        let rect=unsafe{
            let height:f64=character.height() as f64;
            let width:f64=character.width() as f64;

            let y=window_height-height;

            let x=match location{
                CharacterLocation::Left=>margin,

                CharacterLocation::LeftCenter=>(window_center[0]-width)/2f64,

                CharacterLocation::CenterLeft=>window_center[0]-width,

                CharacterLocation::Center=>(window_width-width)/2f64,

                CharacterLocation::CenterRight=>window_center[0],

                CharacterLocation::RightCenter=>window_center[0]+(window_center[0]-width)/2f64,

                CharacterLocation::Right=>window_width-width-margin
            };
            [x,y,width,height]
        };

        let settings=TextureSettings::new();
        let character=Character{
            image:ImageBase::new(White,rect),
            texture:Texture::from_image(character,&settings),
        };

        self.characters.push(character)
    }

    pub fn clear(&mut self){
        self.characters.clear()
    }

    pub fn set_focus(&mut self,index:usize){
        let rect=&mut self.characters[index].image.rect;
        rect[0]-=focused_movement;
        rect[1]-=focused_movement;
        rect[2]+=focused_resize;
        rect[3]+=focused_resize;
    }

    pub fn mouse_shift(&mut self,mut dx:f64,mut dy:f64){
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
            ch.image.color[3]=alpha;
        }
    }

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics){
        for ch in &mut self.characters{
            ch.image.draw(&ch.texture,&context.draw_state,context.transform,graphics);
        }
    }
}