use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    text::Character,
};

use glium::{
    // macroses
    implement_vertex,
    uniform,
    // structs
    Program,
    Frame,
    DrawParameters,
    Surface,
    VertexBuffer,
    Display,
    index::{NoIndices,PrimitiveType},
};

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

            void main() {
                color = vec4(colour.xyz, colour.w * alpha);
            }
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
        colour:Colour,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
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
                &uniform!{colour:colour},
                draw_parameters,
            );
        }
    }
}