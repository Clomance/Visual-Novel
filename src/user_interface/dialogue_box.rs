use crate::*;

const k:f64=3.3f64; // Отношение размера окна игры к диалоговому окну

const font_size:u32=24;

const text_position_x:f64=80f64;

const name_position_x:f64=20f64;

pub struct DialogueBox<'a>{
    dialogue:*const Dialogue,
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

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha;
        self.text_base.color[3]=alpha;
    }

    // pub fn set_text_color(&mut self,color:Color){
    //     self.text.color=color;
    // }

    pub fn next(&mut self)->bool{
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

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        let (name,line)=unsafe{(*self.dialogue).get_line(self.step)};
        // Основа
        g.image(&self.image,&self.texture,draw_state,transform);

        // Имя
        self.text_base.draw(name,[name_position_x,self.y1+30f64],draw_state,transform,g,&mut self.glyphs);


        self.text_base.draw(line,[text_position_x,self.text_y],draw_state,transform,g,&mut self.glyphs);
    }
}