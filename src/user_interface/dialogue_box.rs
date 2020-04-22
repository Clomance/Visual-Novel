use crate::*;

const k:f64=3.3f64; // Отношение размера окна игры к диалоговому окну

const font_size:u32=dialogues_font_size;

const text_position_x:f64=80f64;

const name_position_x:f64=20f64;
const name_position_y:f64=40f64;

pub struct DialogueBox<'a,'b>{
    dialogue:DialogueFormatted<'b>,
    whole_text:bool, // Флаг вывода всего текста
    chars:f64, // Количесво выводимых в данный момент символов диалога
    dialogue_step:usize,
    y1:f64, // Граница нижней трети экрана, где находится диалоговое окно
    text_base:TextBase,
    name_base:TextBase,
    image:Image,
    texture:Texture,
    glyphs:GlyphCache<'a>
}

impl<'a,'b> DialogueBox<'a,'b>{
    pub fn new(texture:&RgbaImage)->DialogueBox<'a,'b>{
        let texture_settings=TextureSettings::new();
        let glyphs=GlyphCache::new("fonts/CALIBRI.TTF",(),texture_settings).unwrap();
        let texture=Texture::from_image(texture,&texture_settings);
        unsafe{
            let height=window_height/k; // Высота диалогового окна
            let y1=window_height-height; // Верхняя граница диалогового окна

            Self{
                dialogue:DialogueFormatted::empty(),
                whole_text:false,
                chars:0f64,
                dialogue_step:Settings.saved_dialogue,
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
                glyphs:glyphs
            }
        }
    }

    pub fn set_step(&mut self,step:usize){
        self.dialogue_step=step;
    }

    pub fn current_step(&self)->usize{
        self.dialogue_step
    }

    // Установка нового диалога, шаг обнулён заранее, при переходе к новой странице (next_page)
    pub fn set_dialogue(&mut self,dialogue:&'b Dialogue){
        unsafe{
            let line_length=window_width-2f64*text_position_x;
            self.dialogue=dialogue.format(&Settings.user_name,line_length,self.text_base.font_size,&mut self.glyphs);
        }
        self.chars=0f64;
        self.whole_text=false;
    }

    // false - cлeдующая часть диалога или мгновенный вывод текста, true - следующая страница
    pub fn next_page(&mut self)->bool{
        if self.whole_text{ // Если выведен весь текст
            self.whole_text=false; // Установка флага вывода всего текста
            self.chars=0f64; // Обнуление количества выводимых символов
            self.dialogue_step+=1; // Слудующая часть диалога
            if self.dialogue_step<self.dialogue.len(){ // Проверка есть ли следующая часть диалога
                false
            }
            else{
                self.dialogue_step=0; // Переход к новой странице, обнуление шага
                true
            }
        }
        else{
            self.whole_text=true; // Установка флага вывода всего текста
            false
        }
    }

    pub fn clicked(&mut self)->bool{
        unsafe{
            self.y1<mouse_cursor.position()[1] // Если курсор в нижней трети экрана
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
        let (name,lines)=self.dialogue.get_line(self.dialogue_step);
        // Основа
        g.image(&self.image,&self.texture,&c.draw_state,c.transform);

        // Имя
        self.name_base.draw(name,c,g,&mut self.glyphs);

        // Реплика
        if self.whole_text{
            self.text_base.draw_lined_text(lines,c,g,&mut self.glyphs) // Вывод всего текста
        }
        else{
            unsafe{
                self.chars+=Settings.signs_per_frame;
            }
            // Вывод части текста
            self.whole_text=self.text_base.draw_lined_text_part(lines,self.chars as usize,c,g,&mut self.glyphs);
        }
    }
}