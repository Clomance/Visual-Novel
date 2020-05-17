use crate::{
    // types
    Colour,
    // structs
    graphics::GameGraphics
};

use super::{
    Glyphs,
    Character,
};

use glium::DrawParameters;

pub struct TextBase{
    pub position:[f32;2],
    pub font_size:f32,
    pub colour:Colour,
}

impl TextBase{
    pub const fn new(colour:Colour,font_size:f32)->TextBase{
        Self{
            font_size,
            colour,
            position:[0f32;2]
        }
    }

    pub const fn position(mut self,position:[f32;2])->TextBase{
        self.position=position;
        self
    }

    #[inline(always)]
    pub fn set_x(&mut self,x:f32){
        self.position[0]=x
    }

    #[inline(always)]
    pub fn set_position(&mut self,position:[f32;2]){
        self.position=position
    }

    #[inline(always)]
    pub fn shift_x(&mut self,dx:f32){
        self.position[0]+=dx
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.position[0]+=dx;
        self.position[1]+=dy;
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.colour[3]=alpha;
    }

    // Выводит один символ
    pub fn draw_char(&self,character:char,draw_parameters:&DrawParameters,graphics:&mut GameGraphics,glyphs:Glyphs){
        let position=self.position;

        let character=glyphs.character_positioned(character,self.font_size,position);

        graphics.draw_character(self.colour,&character,draw_parameters);
    }

    #[inline(always)] // Выводит уже данный символ
    pub fn draw_character(&self,character:&Character,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_character(self.colour,character,&draw_parameters);
    }

    // Выодит весь текст
    pub fn draw(&self,text:&str,draw_parameters:&DrawParameters,graphics:&mut GameGraphics,glyphs:&Glyphs){
        let mut position=self.position;

        for c in text.chars(){
            let character=glyphs.character_positioned(c,self.font_size,position);
            graphics.draw_character(self.colour,&character,draw_parameters);

            position[0]+=character.width();
        }
    }

    // Выводит часть текста, если выведен весь текста, возвращает true
    pub fn draw_part(&self,text:&str,chars:usize,draw_parameters:&DrawParameters,graphics:&mut GameGraphics,glyphs:&Glyphs)->bool{
        let mut position=self.position;

        let mut whole=true; // Флаг вывода всего текста

        for (i,c) in text.chars().enumerate(){
            if i==chars{
                whole=false;
                break
            }
            let character=glyphs.character_positioned(c,self.font_size,position);

            graphics.draw_character(self.colour,&character,draw_parameters);

            position[0]+=character.width();
        }

        whole
    }
}