use crate::*;


// Текстовый блок
pub struct TextView<'a>{
    text_view_base:TextViewDependent,
    glyphs:GlyphCache<'a>,
}

impl<'a> TextView<'a>{
    pub fn new(settings:TextViewSettings,mut glyphs:GlyphCache<'a>)->TextView<'a>{
        Self{
            text_view_base:TextViewDependent::new(settings,&mut glyphs),
            glyphs:glyphs
        }
    }
    
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.text_view_base.set_alpha_channel(alpha)
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        self.text_view_base.draw(draw_state,transform,g,&mut self.glyphs);
    }
}

// Зависимый от шрифта текстовый блок
pub struct TextViewDependent{
    x1:f64,
    y1:f64,
    text_base:Text,
    text:String,
}

impl TextViewDependent{
    pub fn new(settings:TextViewSettings,glyphs:&mut GlyphCache)->TextViewDependent{
        let mut text_len=0f64;
        for ch in settings.text.chars(){
            let character=glyphs.character(settings.font_size,ch).unwrap();
            text_len+=character.advance_width();
        }

        let x1=settings.rect[0]+(settings.rect[2]-text_len)/2f64;
        let y1=settings.rect[1]+(settings.rect[3]+settings.font_size as f64)/2f64;

        Self{
            x1:x1,
            y1:y1,
            text_base:Text::new_color(settings.text_color,settings.font_size),
            text:settings.text
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.text_base.color[3]=alpha;
    }

    // pub fn get_text_color_mut(&mut self)->&mut Color{
    //     &mut self.text_base.color
    // }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let x=self.x1;
        let y=self.y1;
        self.text_base.draw(&self.text,glyphs,draw_state,transform.trans(x,y),g);
    }
}

#[derive(Clone)]
pub struct TextViewSettings{
    pub rect:[f64;4], // [x1,y1,width,height] - сюда вписывается текст
    pub text:String,
    pub font_size:u32,
    pub text_color:Color
}

impl TextViewSettings{
    pub fn new()->TextViewSettings{
        Self{
            rect:[0f64;4],
            text:String::new(),
            font_size:20,
            text_color:BLACK
        }
    }

    pub fn rect(mut self,rect:[f64;4])->TextViewSettings{
        self.rect=rect;
        self
    }

    pub fn text(mut self,text:String)->TextViewSettings{
        self.text=text;
        self
    }

    pub fn font_size(mut self,size:u32)->TextViewSettings{
        self.font_size=size;
        self
    }

    pub fn text_color(mut self,color:Color)->TextViewSettings{
        self.text_color=color;
        self
    }
}