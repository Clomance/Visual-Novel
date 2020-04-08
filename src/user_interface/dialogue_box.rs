use crate::*;

const k:f64=3.3f64; // Отношение размера окна игры к диалоговому окну

const font_size:u32=dialogues_font_size;

const text_position_x:f64=80f64;

const name_position_x:f64=20f64;
const name_position_y:f64=40f64;

const sign_per_frame:usize=1; // Знаков за один кадр (для постепенного вывода текста)

pub struct DialogueBox<'a,'b>{
    dialogue:DialogueFormatted<'b>,
    line_step:usize,
    step:usize,
    y1:f64, // Граница нижней трети экрана, где находится диалоговое окно
    text_base:TextBase,
    name_base:TextBase,
    image:Image,
    texture:Texture,
    glyphs:GlyphCache<'a>
}

impl<'a,'b> DialogueBox<'a,'b>{
    pub fn new(texture:Texture,glyph:GlyphCache<'a>)->DialogueBox<'a,'b>{
        unsafe{
            let height=window_height/k; // Высота диалогового окна
            let y1=window_height-height; // Верхняя граница диалогового окна

            Self{
                dialogue:DialogueFormatted::empty(),
                line_step:0,
                step:Settings.saved_dialogue,
                y1:y1,
                text_base:TextBase::new_color(White,font_size)
                        .position([text_position_x,window_height-height/2f64]),

                name_base:TextBase::new_color(White,font_size)
                        .position([name_position_x,y1+name_position_y]),

                image:Image::new_color([1.0;4]).rect([
                    0f64,
                    y1,
                    window_width,
                    height
                ]),
                
                texture:texture,
                glyphs:glyph
            }
        }
    }

    pub fn current_step(&self)->usize{
        self.step
    }

    pub fn set_dialogue(&mut self,dialogue:&'b Dialogue){
        unsafe{
            let line_length=window_width-2f64*text_position_x;
            self.dialogue=dialogue.format(&Settings.user_name,line_length,self.text_base.font_size,&mut self.glyphs);
        }
        self.step=0usize;
    }

    pub fn next(&mut self)->bool{
        self.line_step=0;
        if self.step+1<self.dialogue.len(){
            self.step+=1;
            true
        }
        else{
            false
        }
    }

    pub fn clicked(&mut self)->bool{
        unsafe{
            if self.y1<mouse_position[1]{ // Если курсор в нижней трети экрана
                self.line_step=!0;
                true
            }
            else{
                false
            }
        }
    }

    pub fn draw_without_text(&mut self,c:&Context,g:&mut GlGraphics){
        g.image(&self.image,&self.texture,&c.draw_state,c.transform);
    }
}

impl<'a,'b> Drawable for DialogueBox<'a,'b>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color.as_mut().unwrap()[3]=alpha;
        self.text_base.color[3]=alpha;
    }

    fn draw(&mut self,c:&Context,g:&mut GlGraphics){
        let (name,lines)=self.dialogue.get_line(self.step);
        // Основа
        g.image(&self.image,&self.texture,&c.draw_state,c.transform);

        // Имя
        self.name_base.draw(name,c,g,&mut self.glyphs);

        // Реплика
        self.line_step+=sign_per_frame;
        self.text_base.draw_lined_text_slowly(lines,self.line_step,c,g,&mut self.glyphs);

    }
}