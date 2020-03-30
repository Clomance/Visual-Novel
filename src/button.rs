use crate::*;

pub struct Button<'a>{
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    width:f64,
    height:f64,
    text:String,
    glyphs:GlyphCache<'a>
}

impl<'a> Button<'a>{
    pub fn new(rect:[f64;4],text:String,glyphs:GlyphCache<'a>)->Button<'a>{
        let x2=rect[0]+rect[2];
        let y2=rect[1]+rect[3];

        Self{
            x1:rect[0],
            y1:rect[1],
            x2:x2,
            y2:y2,
            width:rect[2],
            height:rect[3],
            text:text,
            glyphs:glyphs,
        }
    }

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

    pub fn set_text(&mut self,text:String){
        self.text=text;
    }

    pub fn clicked(&mut self)->bool{
        let x=unsafe{mouse_position[0]};
        let y=unsafe{mouse_position[1]};

        self.x1<x && self.x2>x && self.y1<y && self.y2>y
    }
}

impl<'a> Drawable for Button<'a>{
    fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        let rect_pos=[self.x1,self.y1,self.width,self.height];
        let rect=Rectangle::new(Light_blue);
        rect.draw(rect_pos,draw_state,transform,g);

        let text=Text::new_color(BLACK,20);
        let x=self.x1+10f64;
        let y=self.y2-10f64;
        text.draw(&self.text,&mut self.glyphs,draw_state,transform.trans(x,y),g);
    }
}

pub struct MenuButton{
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    width:f64,
    height:f64,
    text:String,
}

impl MenuButton{
    pub fn new(rect:[f64;4],text:String)->MenuButton{
        let x2=rect[0]+rect[2];
        let y2=rect[1]+rect[3];

        Self{
            x1:rect[0],
            y1:rect[1],
            x2:x2,
            y2:y2,
            width:rect[2],
            height:rect[3],
            text:text,
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

        let text=Text::new_color(BLACK,20);
        let x=self.x1+10f64;
        let y=self.y2-10f64;
        text.draw(&self.text,glyphs,draw_state,transform.trans(x,y),g);
    }
}