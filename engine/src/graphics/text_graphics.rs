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
    // enums
    DrawError,
    // traits
    Surface,
    // structs
    Program,
    Frame,
    DrawParameters,
    VertexBuffer,
    Display,
    index::{NoIndices,PrimitiveType},
};

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
    /// 
    pub fn new(display:&Display,buffer_size:usize,glsl:u16)->TextGraphics{
        let (vertex_shader,fragment_shader)=if glsl==120{(
            include_str!("shaders/120/text_vertex_shader.glsl"),
            include_str!("shaders/120/text_fragment_shader.glsl"),
        )}
        else{(
            include_str!("shaders/text_vertex_shader.glsl"),
            include_str!("shaders/text_fragment_shader.glsl"),
        )};

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,buffer_size).unwrap(),
            program:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
        }
    }

    /// Выводит символ на позицию, которая записана в нём
    pub fn draw_character(
        &self,
        character:&Character,
        colour:Colour,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
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
            )
        }
        else{
            Ok(())
        }
    }
}