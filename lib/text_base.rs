use crate::*;

use image_base::ImageBaseSrc;

// Текстовая основа - с помощью неё выводится текст
// Она содежит начальное положение текста, цвет и размер шрифта
pub struct TextBase{
    pub image:ImageBaseSrc,
    pub font_size:u32,
}

impl TextBase{
    pub fn new_color(color:Color,font_size:u32)->TextBase{
        Self{
            image:ImageBaseSrc::new(color,[0f64;4],[0f64;4]),
            font_size:font_size,
        }
    }

    pub fn position(mut self,position:[f64;2])->TextBase{
        self.image.rect[0]=position[0];
        self.image.rect[1]=position[1];
        self
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color[3]=alpha
    }

    pub fn set_x(&mut self,x:f64){
        self.image.rect[0]=x;
    }

    pub fn set_position(&mut self,position:[f64;2]){
        self.image.rect[0]=position[0];
        self.image.rect[1]=position[1];
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        self.image.rect[0]+=dx;
        self.image.rect[1]+=dy;
    }

    pub fn draw(&mut self,text:&str,c:&Context,g:&mut GameGraphics,glyphs:&mut GlyphCache){
        // Сохранение начального положения
        let (x,y)=(self.image.rect[0],self.image.rect[1]);

        // Перебор символов
        for ch in text.chars(){
            let character=glyphs.character(self.font_size,ch).unwrap();

            { // Установка положения и размер символа
                self.image.rect[0]+=character.left();
                self.image.rect[1]-=character.top();
                self.image.rect[2]=character.atlas_size[0];
                self.image.rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                self.image.src_rect[0]=character.atlas_offset[0];
                self.image.src_rect[1]=character.atlas_offset[1];
                self.image.src_rect[2]=character.atlas_size[0];
                self.image.src_rect[3]=character.atlas_size[1];
            }

            self.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше по линии и возвращение обратно на линию
            self.image.rect[0]+=character.advance_width()-character.left();
            self.image.rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        self.image.rect[0]=x;
        self.image.rect[1]=y;
    }
}