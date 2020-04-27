use crate::*;

pub trait Drawable{
    fn set_alpha_channel(&mut self,alpha:f32);

    fn draw(&mut self,context:&Context,graphics:&mut GlGraphics);

    fn draw_smooth(&mut self,alpha:f32,context:&Context,graphics:&mut GlGraphics){
        self.set_alpha_channel(alpha);
        self.draw(context,graphics);
    }
}

pub trait Text{
    // Вывод всего текста
    fn draw(&self,text_base:&mut TextBase,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache);

    // Вывод части текста, если текст выведен полностью - true, в ином случае - false
    fn draw_part(&self,chars:usize,text_base:&mut TextBase,c:&Context,g:&mut GlGraphics,glyphs:&mut GlyphCache)->bool;
}