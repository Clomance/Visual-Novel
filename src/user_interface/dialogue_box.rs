use crate::*;

const dialog_height:f64=300f64;
const font_size:u32=18;

const text_position_x:f64=80f64;

const name_position_x:f64=20f64;

pub struct DialogueBox<'a,'b>{
    dialog:&'b Dialogue,
    step:usize,
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    text:Text,
    text_position:[f64;2],
    image:Image,
    texture:Texture,
    glyphs:GlyphCache<'a>
}

impl<'a,'b> DialogueBox<'a,'b>{
    pub fn new(texture:Texture,glyph:GlyphCache<'a>,dialogue:&'b Dialogue)->DialogueBox<'a,'b>{
        unsafe{
            let text=Text::new_color(WHITE,font_size);
            let image=Image::new().rect([
                0f64,
                Settings.window_size.height-dialog_height,
                Settings.window_size.width,
                dialog_height
            ]);

            Self{
                dialog:dialogue,
                step:0usize,
                x1:0f64,
                y1:Settings.window_size.height-dialog_height,
                x2:Settings.window_size.width,
                y2:Settings.window_size.height,
                text:text,
                text_position:[text_position_x,Settings.window_size.height-dialog_height/2f64],
                image:image,
                texture:texture,
                glyphs:glyph
            }
        }
    }

    pub fn set_dialogue(&mut self,dialog:&'b Dialogue){
        self.dialog=dialog;
        self.step=0usize;
    }

    // pub fn set_text_color(&mut self,color:Color){
    //     self.text.color=color;
    // }

    // pub fn fit_screen(&mut self){
    //     let x=unsafe{Settings.window_size[0]};
    //     let y=unsafe{Settings.window_size[1]};

    //     let rect=self.image.rectangle.as_mut().unwrap();
    //     self.y1=y-dialog_height;
    //     self.x2=x;
    //     self.y2=y;

    //     rect[1]=y-dialog_height;
    //     rect[2]=x;
    //     self.text_position=[text_position_x,y-dialog_height/2f64];
    // }

    pub fn next(&mut self)->bool{
        if self.step+1<self.dialog.len(){
            self.step+=1;
            true
        }
        else{
            false
        }
    }

    pub fn clicked(&self)->bool{
        unsafe{
            let x=mouse_position[0];
            let y=mouse_position[1];

            self.x1<x && self.x2>x && self.y1<y && self.y2>y
        }
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        let (name,line)=self.dialog.get_line(self.step);
        // Основа
        g.image(&self.image,&self.texture,draw_state,transform);
        // Имя
        self.text
            .draw(name,&mut self.glyphs,draw_state,transform.trans(name_position_x,self.y1+30f64),g);

        // Реплика
        let max_x=self.x2-self.text_position[0]*2f64;
        draw_text(&self.text,line,&mut self.glyphs,draw_state,transform.trans(self.text_position[0],self.text_position[1]),g,max_x);
    }
}

fn draw_text(text_base:&Text,text:&str,cache:&mut GlyphCache,
        draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics,max_x:f64){

    let mut image = Image::new_color(text_base.color);
    let next_line=text_base.font_size as f64+5f64;
    let mut x = 0f64;
    let mut y = 0f64;
    for ch in text.chars() {
        let character = cache.character(text_base.font_size, ch).unwrap();
        let mut ch_x:f64 = x + character.left();
        let mut ch_y:f64 = y - character.top();
        if text_base.round {
            ch_x = ch_x.round();
            ch_y = ch_y.round();
        }
        image = image.src_rect([
            character.atlas_offset[0], character.atlas_offset[1],
            character.atlas_size[0], character.atlas_size[1]
        ]);
        image.draw(character.texture,
            draw_state,
            transform.trans(ch_x, ch_y),
            g);
        x += character.advance_width();
        y += character.advance_height();
        if x>max_x{
            x=0f64;
            y+=next_line;
        }
    }
}