use crate::*;

use glium::Display;

const k:f64=3.3f64; // Отношение размера окна игры к диалоговому окну

const font_size:u32=24u32;

const text_position_x:f64=80f64;

const name_position_x:f64=20f64;
const name_position_y:f64=45f64;

pub struct DialogueBox<'a,'b>{
    dialogue:DialogueFormatted<'b>,
    whole_text:bool, // Флаг вывода всего текста
    chars:f64, // Количесво выводимых в данный момент символов диалога
    dialogue_step:usize,
    y1:f64, // Граница нижней трети экрана, где находится диалоговое окно
    name_base:TextBase,
    lines:TextViewLinedDependent, // Текстовый блок для диалогов
    image:ImageBase,
    texture:Texture,
    glyphs:GlyphCache<'a>
}

impl<'a,'b> DialogueBox<'a,'b>{
    pub fn new(texture:&RgbaImage,display:&mut Display)->DialogueBox<'a,'b>{
        let texture_settings=TextureSettings::new();
        let mut glyphs=GlyphCache::new("./resources/fonts/CALIBRI.TTF",display.clone(),texture_settings).unwrap();
        let texture=Texture::from_image(display,texture,&texture_settings).unwrap();
        unsafe{
            let height=window_height/k; // Высота диалогового окна
            let y1=window_height-height; // Верхняя граница диалогового окна

            let line_settings=TextViewSettings::new("",[
                        text_position_x,
                        window_height-height+name_position_y*3f64,
                        window_width-2f64*text_position_x,
                        height/2f64,
                    ])
                    .font_size(24)
                    .align_x(AlignX::Left)
                    .align_y(AlignY::Up)
                    .text_color(White);

            Self{
                dialogue:DialogueFormatted::empty(),
                whole_text:false,
                chars:0f64,
                dialogue_step:Settings.saved_dialogue,
                y1:y1,
                lines:TextViewLinedDependent::new(line_settings,&mut glyphs),

                name_base:TextBase::new_color(White,font_size)
                        .position([name_position_x,y1+name_position_y]),

                image:ImageBase::new(White,[
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
            self.dialogue=dialogue.format(&Settings.user_name);
            self.lines.set_text(self.dialogue.get_line(self.dialogue_step),&mut self.glyphs);
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
                self.lines.set_text(self.dialogue.get_line(self.dialogue_step),&mut self.glyphs);
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

    pub fn draw_without_text(&mut self,c:&Context,g:&mut GameGraphics){
        self.image.draw(&self.texture,&c.draw_state,c.transform,g);
    }
}

impl<'a,'b> Drawable for DialogueBox<'a,'b>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.image.color[3]=alpha;
        //self.lines.set_alpha_channel(alpha);
    }

    fn draw(&mut self,c:&Context,g:&mut GameGraphics){
        let name=self.dialogue.get_name(self.dialogue_step);

        self.image.draw(&self.texture,&c.draw_state,c.transform,g); // Основа

        self.name_base.draw(name,c,g,&mut self.glyphs); // Имя

        // Реплика
        if self.whole_text{
            self.lines.draw(c,g,&mut self.glyphs) // Вывод всего текста
        }
        else{
            unsafe{
                self.chars+=Settings.signs_per_frame; // Количество выводимых символов
            }
            // Вывод части текста
            self.whole_text=self.lines.draw_part(self.chars as usize,c,g,&mut self.glyphs);
        }
    }
}