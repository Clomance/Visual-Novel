use crate::*;

const k:f64=3.3f64; // Отношение размера окна игры к диалоговому окну

const font_size:u32=24;

const text_position_x:f64=80f64;

const name_position_x:f64=20f64;
const name_position_y:f64=40f64;

const sign_per_frame:usize=1; // Знаков за один кадр (для постепенного вывода текста)

pub struct DialogueBox<'a>{
    dialogue:*const Dialogue,
    line_step:usize,
    step:usize,
    y1:f64, // Граница нижней трети экрана, где находится диалоговое окно
    text_base:TextBase,
    text_y:f64,
    image:Image,
    texture:Texture,
    glyphs:GlyphCache<'a>
}

impl<'a> DialogueBox<'a>{
    pub fn new(texture:Texture,glyph:GlyphCache<'a>)->DialogueBox<'a>{
        unsafe{
            let height=window_height/k; // Высота диалогового окна
            let y1=window_height-height; // Верхняя граница диалогового окна

            let image=Image::new_color([1.0;4]).rect([
                0f64,
                y1,
                window_width,
                height
            ]);

            Self{
                dialogue:std::ptr::null(),
                line_step:0,
                step:Settings.saved_dialogue,
                y1:y1,
                text_base:TextBase::new_color(font_size,window_width-2f64*text_position_x,White),
                text_y:window_height-height/2f64,
                image:image,
                texture:texture,
                glyphs:glyph
            }
        }
    }

    pub fn current_step(&self)->usize{
        self.step
    }

    pub fn set_dialogue(&mut self,dialogue:&Dialogue){

        self.dialogue=dialogue as *const Dialogue;
        self.step=0usize;
    }



    // pub fn set_text_color(&mut self,color:Color){
    //     self.text.color=color;
    // }

    pub fn next(&mut self)->bool{
        self.line_step=0;
        if self.step+1<unsafe{(*self.dialogue).len()}{
            self.step+=1;
            true
        }
        else{
            false
        }
    }

    pub fn clicked(&self)->bool{
        unsafe{
            self.y1<mouse_position[1] // Если курсор в нижней трети экрана
        }
    }

    pub fn draw_without_text(&mut self,c:&Context,g:&mut GlGraphics){
        g.image(&self.image,&self.texture,&c.draw_state,c.transform);
    }
}

impl<'a> Drawable for DialogueBox<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha;
        self.text_base.color[3]=alpha;
    }

    fn draw(&mut self,c:&Context,g:&mut GlGraphics){
        let (name,line)=unsafe{(*self.dialogue).get_line(self.step)};
        // Основа
        g.image(&self.image,&self.texture,&c.draw_state,c.transform);

        // Имя
        self.text_base.draw(name,[name_position_x,self.y1+name_position_y],c,g,&mut self.glyphs);
        
        self.line_step+=sign_per_frame;

        self.text_base.draw_slowly(line,[text_position_x,self.text_y],self.line_step,c,g,&mut self.glyphs);
    }
}