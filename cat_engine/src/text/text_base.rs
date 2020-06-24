use crate::{
    // types
    Colour,
    // structs
    graphics::Graphics
};

use super::{
    Glyphs,
    Character,
};

use glium::DrawParameters;

const text_pixel_size:f32=1f32; // Размер одной точки (можно сделать текст жирнее)

/// Основа для текста с установленным
/// цветом и размером шрифта.
/// Сам шрифт задаётся отдельно во время вывода.
/// 
/// A base for text with set colour and font_size.
/// The font is set at drawing.
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
    pub fn set_y(&mut self,y:f32){
        self.position[1]=y
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
    pub fn shift_y(&mut self,dy:f32){
        self.position[1]+=dy
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.position[0]+=dx;
        self.position[1]+=dy;
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.colour[3]=alpha
    }

    #[inline(always)]
    pub fn set_colour(&mut self,colour:Colour){
        self.colour=colour
    }

    /// Выводит уже готовый символ.
    /// 
    /// Draws already built character.
    #[inline(always)] 
    pub fn draw_character(&self,character:&Character,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        graphics.draw_character(self.colour,character,draw_parameters);
    }

    /// Строит и выводит один символ.
    /// 
    /// Builds and draws a character.
    pub fn draw_char(&self,character:char,draw_parameters:&mut DrawParameters,graphics:&mut Graphics,glyphs:&Glyphs){
        let position=self.position;

        let character=glyphs.character_positioned(character,self.font_size,position);

        draw_parameters.point_size=Some(text_pixel_size);
        graphics.draw_character(self.colour,&character,draw_parameters);
    }

    /// Выводит весь текст в строчку.
    /// 
    /// Draws a string.
    pub fn draw(&self,text:&str,draw_parameters:&mut DrawParameters,graphics:&mut Graphics,glyphs:&Glyphs){
        let mut position=self.position;
        draw_parameters.point_size=Some(text_pixel_size);
        for c in text.chars(){
            let character=glyphs.character_positioned(c,self.font_size,position);
            graphics.draw_character(self.colour,&character,draw_parameters);

            position[0]+=character.width();
        }
    }

    /// Выводит часть текста в строчку.
    /// Если текст выведен полностью, возвращает true.
    /// 
    /// Draws a part of the string.
    /// Returns true, if the whole string was drawn.
    pub fn draw_part(&self,text:&str,chars:usize,draw_parameters:&mut DrawParameters,graphics:&mut Graphics,glyphs:&Glyphs)->bool{
        let mut position=self.position;
        draw_parameters.point_size=Some(text_pixel_size);

        let mut whole=true; // Флаг вывода всего текста

        for (i,c) in text.chars().enumerate(){
            if i==chars{
                whole=false;
                break
            }

            // Создание символа с заданной позицией
            let character=glyphs.character_positioned(c,self.font_size,position);

            graphics.draw_character(self.colour,&character,draw_parameters);

            position[0]+=character.width(); // Сдвиг дальше по линии
        }

        whole
    }
}