use crate::*;

pub struct Button<'a>{
    button_base:ButtonDependent,
    glyphs:GlyphCache<'a>
}

impl<'a> Button<'a>{
    // pub fn new(rect:[f64;4],text:String,font_size:u32,mut glyphs:GlyphCache<'a>)->Button<'a>{
    //     Self{
    //         button_base:ButtonDependent::new(rect,text,font_size,&mut glyphs),
    //         glyphs:glyphs,
    //     }
    // }

    // pub fn set_rect(&mut self,rect:[f64;4]){
    //     let x2=rect[0]+rect[2];
    //     let y2=rect[1]+rect[3];

    //     self.x1=rect[0];
    //     self.y1=rect[1];
    //     self.x2=x2;
    //     self.y2=y2;
    //     self.width=rect[2];
    //     self.height=rect[3];
    // }

    // pub fn set_text(&mut self,text:String){
    //     self.buton_base.text=text;
    // }

    pub fn clicked(&mut self)->bool{
        self.button_base.clicked()
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        self.button_base.draw(draw_state,transform,g,&mut self.glyphs)
    }
}

// Зависимая от шрифта кнопка (должно быть больше зависимостей)
pub struct ButtonDependent{
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    width:f64,
    height:f64,
    rectanlge:Rectangle,
    text:TextViewDependent, // Зависимый от шрифта текстовый блок
}

impl ButtonDependent{
    pub fn new(settings:ButtonSettings,glyphs:&mut GlyphCache)->ButtonDependent{
        let text_view_settings=TextViewSettings::new()
                .rect(settings.rect)
                .text_color(settings.text_color)
                .text(settings.text)
                .font_size(settings.font_size);

        let x2=settings.rect[0]+settings.rect[2];
        let y2=settings.rect[1]+settings.rect[3];

        Self{
            x1:settings.rect[0],
            y1:settings.rect[1],
            x2:x2,
            y2:y2,
            width:settings.rect[2],
            height:settings.rect[3],
            rectanlge:Rectangle::new(settings.background_color),
            text:TextViewDependent::new(text_view_settings,glyphs),
        }
    }

    pub fn clicked(&mut self)->bool{
        let x=unsafe{mouse_position[0]};
        let y=unsafe{mouse_position[1]};

        self.x1<x && self.x2>x && self.y1<y && self.y2>y
    }
    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics,glyphs:&mut GlyphCache){
        let rect_pos=[self.x1,self.y1,self.width,self.height];
        let rect=Rectangle::new(Light_blue);
        rect.draw(rect_pos,draw_state,transform,g);

        self.text.draw(draw_state,transform,g,glyphs);
    }
}

#[derive(Clone)]
pub struct ButtonSettings{
    pub rect:[f64;4],
    pub background_color:Color,
    pub text:String,
    pub font_size:u32,
    pub text_color:Color
}

impl ButtonSettings{
    pub fn new()->ButtonSettings{
        Self{
            rect:[0f64;4],
            background_color:Gray,
            text:String::new(),
            font_size:20,
            text_color:BLACK,
        }
    }

    pub fn rect(mut self,rect:[f64;4])->ButtonSettings{
        self.rect=rect;
        self
    }

    pub fn background_color(mut self,color:Color)->ButtonSettings{
        self.background_color=color;
        self
    }

    pub fn text(mut self,text:String)->ButtonSettings{
        self.text=text;
        self
    }

    pub fn font_size(mut self,size:u32)->ButtonSettings{
        self.font_size=size;
        self
    }
    
    pub fn text_color(mut self,color:Color)->ButtonSettings{
        self.text_color=color;
        self
    }
}