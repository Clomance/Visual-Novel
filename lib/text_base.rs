use crate::*;

// Текстовая основа - с помощью неё выводится текст
// Она содежит начальное положение текста, цвет и размер шрифта
pub struct TextBase{
    pub image:Image,
    pub font_size:u32,
}

impl TextBase{
    pub fn new_color(color:Color,font_size:u32)->TextBase{
        Self{
            image:Image::new_color(color).rect([0f64;4]).src_rect([0f64;4]),
            font_size:font_size,
        }
    }

    pub fn position(mut self,position:[f64;2])->TextBase{
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=position[0];
        rect[1]=position[1];
        self
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha
    }

    pub fn set_x(&mut self,x:f64){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=x;
    }

    pub fn set_position(&mut self,position:[f64;2]){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]=position[0];
        rect[1]=position[1];
    }

    pub fn shift(&mut self,dx:f64,dy:f64){
        let rect=self.image.rectangle.as_mut().unwrap();
        rect[0]+=dx;
        rect[1]+=dy;
    }

    pub fn draw(&mut self,text:&str,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        // Сохранение начального положения
        let (x,y)={
            let image_rect=self.image.rectangle.as_ref().unwrap();
            (image_rect[0],image_rect[1])
        };
        // Перебор символов
        for ch in text.chars(){
            let character=glyphs.character(self.font_size,ch).unwrap();

            { // Установка положения и размер символа
                let image_rect=self.image.rectangle.as_mut().unwrap();
                image_rect[0]+=character.left();
                image_rect[1]-=character.top();
                image_rect[2]=character.atlas_size[0];
                image_rect[3]=character.atlas_size[1];
            }

            { // Обрезка символа
                let image_src_rect=self.image.source_rectangle.as_mut().unwrap();
                image_src_rect[0]=character.atlas_offset[0];
                image_src_rect[1]=character.atlas_offset[1];
                image_src_rect[2]=character.atlas_size[0];
                image_src_rect[3]=character.atlas_size[1];
            }

            self.image.draw(character.texture,&c.draw_state,c.transform,g);

            // Сдвиг дальше по линии и возвращение обратно на линию
            let image_rect=self.image.rectangle.as_mut().unwrap();
            image_rect[0]+=character.advance_width()-character.left();
            image_rect[1]+=character.advance_height()+character.top();
        }
        // Возвращение в начальное положение
        let image_rect=self.image.rectangle.as_mut().unwrap();
        image_rect[0]=x;
        image_rect[1]=y;
    }
}