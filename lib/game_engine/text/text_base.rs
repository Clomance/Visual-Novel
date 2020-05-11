use crate::{
    draw_state::convert_draw_state,
    // statics
    window_center,
    // structs
    GameGraphics
};

use super::{
    Glyphs,
    Character,
};

use glium::{
    DrawParameters,
    Display,
    Frame,
    Program,
    index::NoIndices,
    index::PrimitiveType,
    VertexBuffer,
    implement_vertex,
    uniform,
    Surface,
};

use graphics::{
    Context,
    types::Color
};

pub struct TextBase{
    pub position:[f32;2],
    pub font_size:f32,
    pub color:Color,
}

impl TextBase{
    pub const fn new(color:Color,font_size:f32)->TextBase{
        Self{
            font_size,
            color,
            position:[0f32;2]
        }
    }

    pub const fn position(mut self,position:[f32;2])->TextBase{
        self.position=position;
        self
    }

    #[inline(always)]
    pub fn set_x(&mut self,x:f32){
        self.position[0]=x
    }

    #[inline(always)]
    pub fn set_position(&mut self,position:[f32;2]){
        self.position=position
    }

    #[inline(always)]
    pub fn shift_x(&mut self,dx:f32){
        self.position[0]+=dx
    }

    #[inline(always)]
    pub fn shift(&mut self,dx:f32,dy:f32){
        self.position[0]+=dx;
        self.position[1]+=dy;
    }

    #[inline(always)]
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.color[3]=alpha;
    }

    // Выводит один символ
    pub fn draw_char(&self,character:char,context:&Context,graphics:&mut GameGraphics,glyphs:Glyphs){
        let draw_parameters=convert_draw_state(&context.draw_state);

        let position=[self.position[0],self.position[1]];

        let character=glyphs.character_positioned(character,self.font_size,position);

        graphics.draw_character(self,&character,&draw_parameters);
    }

    #[inline(always)] // Выводит уже данный символ
    pub fn draw_character(&self,character:&Character,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_character(self,character,&draw_parameters);
    }

    // Выодит весь текст
    pub fn draw(&self,text:&str,context:&Context,graphics:&mut GameGraphics,glyphs:&Glyphs){
        let mut position=[self.position[0],self.position[1]];

        let draw_parameters=convert_draw_state(&context.draw_state);

        for c in text.chars(){
            let character=glyphs.character_positioned(c,self.font_size,position);
            graphics.draw_character(self,&character,&draw_parameters);

            position[0]+=character.width();
        }
    }

    // Выводит часть текста, если выведен весь текста, возвращает true
    pub fn draw_part(&self,text:&str,chars:usize,context:&Context,graphics:&mut GameGraphics,glyphs:&Glyphs)->bool{
        let mut position=[self.position[0],self.position[1]];
        let draw_parameters=convert_draw_state(&context.draw_state);

        let mut whole=true; // Флаг вывода всего текста

        for (i,c) in text.chars().enumerate(){
            if i==chars{
                whole=false;
                break
            }
            let character=glyphs.character_positioned(c,self.font_size,position);

            graphics.draw_character(self,&character,&draw_parameters);

            position[0]+=character.width();
        }

        whole
    }
}

// Максимальное количество пикселей на символ (считаются только точки самого символа, прозрачные символы пропускаются)
const Character_pixel_limit:usize=2000;

// Пиксель для текста
// Позиция и альфа-канал каждой точки
// Цвет передаётся отдельно - для экономии места
//
implement_vertex!(TextPoint,p);
#[derive(Clone,Copy)]
struct TextPoint{
    p:[f32;3], // position + alpha channel
}
pub struct TextGraphics{
    vertex_buffer:VertexBuffer<TextPoint>,
    program:Program
}

impl TextGraphics{
    // 
    pub fn new(display:&Display)->TextGraphics{
        let vertex_shader=r#"
            #version 140

            in vec3 p;

            out float alpha;

            void main() {
                alpha = p.z;
                gl_Position = vec4(p.x, p.y, 0.0, 1.0);
            }
        "#;

        let fragment_shader=r#"
            #version 140

            in float alpha;
            out vec4 color;

            uniform vec4 colour;

            void main() {{

                color = vec4(colour.xyz, colour.w * alpha);
            }}
        "#;

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,Character_pixel_limit).unwrap(),
            program:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
        }
    }

    // Выводит символ на позицию, которая записана в нём
    pub fn draw_character(
        &mut self,
        character:&Character,
        color:Color,
        frame:&mut Frame,
        draw_parameters:&DrawParameters
    ){
        // Если у символа есть размерная область (не является пробелом)
        if let Some(rect)=character.pixel_bounding_box(){
            let mut len=(rect.width()*rect.height()) as usize;
            self.vertex_buffer.invalidate();

            let mut vec=Vec::with_capacity(len);

            character.draw(|x,y,alpha|unsafe{
                // Пропуск прозрачных пикселей
                if alpha!=0f32{
                    let x=(rect.min.x+x as i32) as f32/window_center[0] as f32-1f32;
                    let y=1f32-(rect.min.y+y as i32)as f32/window_center[1] as f32;

                    let point=TextPoint{
                        p:[x,y,alpha],
                    };

                    vec.push(point);
                }
                else{
                    len-=1;
                }
            });

            let slice=self.vertex_buffer.slice(0..len).unwrap();
            slice.write(&vec);

            frame.draw(
                slice,
                NoIndices(PrimitiveType::Points),
                &self.program,
                &uniform!{colour:color},
                draw_parameters,
            );
        }
    }
}