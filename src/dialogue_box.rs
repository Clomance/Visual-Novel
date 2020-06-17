use crate::{
    Dialogue,
    DialogueFormatted,
    Settings,
};

use lib::{
    AlignX,
    AlignY,
    colours::{White,Black},
    TextViewLined,
    TextViewSettings
};

use cat_engine::{
    // statics
    window_height,
    window_width,
    // structs
    image::{ImageBase,Texture,image::RgbaImage},
    text::{TextBase,Glyphs},
    graphics::{Graphics,Point2D,MonoColourPolygon,Line},
    glium::{Display,DrawParameters},
};

const k:f32=4f32; // Отношение размера окна игры к диалоговому окну

const font_size:f32=24f32;

const image_border_width:f32=4f32;
const dibw:f32=image_border_width/2f32;

pub struct DialogueBox<'b,'c>{
    name_box:MonoColourPolygon,
    name_base:TextBase,

    lines:TextViewLined<'c>, // Текстовый блок для диалогов
    image_border:Line,
    image:ImageBase,
    texture:Texture,

    dialogue:DialogueFormatted<'b>,
    whole_text:bool, // Флаг вывода всего текста
    chars:f32, // Количесво выводимых в данный момент символов диалога
    dialogue_step:usize,
    glyphs:&'c Glyphs
}

impl<'b,'c> DialogueBox<'b,'c>{
    pub fn new(texture:&RgbaImage,display:&Display,glyphs:&'c Glyphs)->DialogueBox<'b,'c>{
        unsafe{
            let height=window_height/k; // Высота диалогового окна
            let y1=window_height-height; // Верхняя граница диалогового окна

            let rect=[
                0f32,
                y1,
                window_width,
                height,
            ];

            let polygon=[
                Point2D::new(0f32,y1-60f32-dibw),
                Point2D::new(400f32,y1-60f32-dibw),
                Point2D::new(0f32,y1-dibw),
                Point2D::new(460f32,y1-dibw),
            ];

            let texture=Texture::from_image(texture,display).unwrap();

            // Позиция имени
            let name_position=[
                20f32,
                y1-18f32,
            ];

            let line_position=[
                60f32,
                y1+40f32,
            ];

            let line_settings=TextViewSettings::new("",[
                        line_position[0],
                        line_position[1],
                        (window_width-2f32*line_position[0]),
                        height-80f32,
                    ])
                    .font_size(font_size)
                    .align_x(AlignX::Left)
                    .align_y(AlignY::Up)
                    .text_colour([0.0, 0.0, 1.0, 1.0]);

            Self{
                // Имя
                name_box:MonoColourPolygon::new(&polygon,Black),
                name_base:TextBase::new(White,font_size)
                        .position([name_position[0],name_position[1]]),

                lines:TextViewLined::new(line_settings,&glyphs),
                image_border:Line::new([0f32,y1,window_width,y1],image_border_width,Black),
                image:ImageBase::new(White,rect),
                texture:texture,

                dialogue:DialogueFormatted::empty(),
                whole_text:false,
                chars:0f32,
                dialogue_step:Settings.saved_dialogue,
                glyphs:glyphs
            }
        }
    }

    #[inline(always)]
    pub fn set_step(&mut self,step:usize){
        self.dialogue_step=step;
    }

    #[inline(always)]
    pub fn current_step(&self)->usize{
        self.dialogue_step
    }

    // Установка нового диалога
    // Шаг обнулён заранее, при переходе к новой странице (next_page)
    pub fn set_dialogue(&mut self,dialogue:&'b Dialogue){
        unsafe{
            self.dialogue=dialogue.format(&Settings.user_name);
            self.lines.set_text(self.dialogue.get_line(self.dialogue_step));
        }
        self.chars=0f32;
        self.whole_text=false;
    }

    // false - cлeдующая часть диалога или вывод всего текста,
    // true - следующая страница
    pub fn next_page(&mut self)->bool{
        if self.whole_text{ // Если выведен весь текст
            self.whole_text=false; // Установка флага для отключения вывода всего текста
            self.chars=0f32; // Обнуление количества выводимых символов
            self.dialogue_step+=1; // Следующая часть диалога
            if self.dialogue_step<self.dialogue.len(){ // Проверка есть ли следующая часть диалога
                self.lines.set_text(self.dialogue.get_line(self.dialogue_step));
                false
            }
            else{
                self.dialogue_step=0; // Переход к новой странице, обнуление шага
                true
            }
        }
        else{
            self.whole_text=true; // Установка флага для вывода всего текста
            false
        }
    }

    #[inline(always)]
    pub fn draw_without_text(&self,draw_parameters:&mut DrawParameters,g:&mut Graphics){
        self.image.draw(&self.texture,draw_parameters,g);
        self.image_border.draw(draw_parameters,g);
        self.name_box.draw(draw_parameters,g);
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.image.colour_filter[3]=alpha;
        self.image_border.colour[3]=alpha;
        self.name_box.colour[3]=alpha;
        self.lines.set_alpha_channel(alpha);
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,g:&mut Graphics){
        let name=self.dialogue.get_name(self.dialogue_step);

        self.image.draw(&self.texture,draw_parameters,g); // Основа

        self.image_border.draw(draw_parameters,g);

        self.name_box.draw(draw_parameters,g);
        self.name_base.draw(name,draw_parameters,g,&self.glyphs); // Имя

        // Реплика
        if self.whole_text{
            self.lines.draw(draw_parameters,g) // Вывод всего текста
        }
        else{
            unsafe{
                self.chars+=Settings.signs_per_frame; // Количество выводимых символов
            }
            // Вывод части текста
            self.whole_text=self.lines.draw_part(self.chars as usize,draw_parameters,g);
        }
    }
}