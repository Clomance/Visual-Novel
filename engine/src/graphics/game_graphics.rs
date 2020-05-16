use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    game_texture::Texture,
    image_base::ImageBase,
    text::{Character,TextBase},
};

use super::{
    SimpleGraphics,
    SimpleObject,
    TextureGraphics,
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


implement_vertex!(TexturedVertex,pos,uv);
#[derive(Copy,Clone)]
struct TexturedVertex{
    pos:[f32;2],
    uv:[f32;2],
}


pub struct Graphics2D{
    texture:TextureGraphics,
    simple:SimpleGraphics,
    text:TextGraphics,
}

impl Graphics2D{
    pub fn new(window:&Display)->Graphics2D{
        Self{
            texture:TextureGraphics::new(window),
            simple:SimpleGraphics::new(window),
            text:TextGraphics::new(window),
        }
    }

    #[inline(always)] // Рисует один символ
    pub fn draw_character(
        &mut self,
        text_base:&TextBase,
        character:&Character,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    ){
        self.text.draw_character(character,text_base.colour,frame,draw_parameters);
    }

    pub fn draw_simple<O:SimpleObject>(
        &mut self,
        object:&O,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    ){
        self.simple.draw(object,frame,draw_parameters);
    }
}

pub struct GameGraphics<'d,'s>{
    system: &'d mut Graphics2D,
    surface: &'s mut Frame,
}

impl<'d,'s> GameGraphics<'d,'s>{
    pub fn new(system:&'d mut Graphics2D,surface:&'s mut Frame)->GameGraphics<'d,'s>{
        GameGraphics{
            system,
            surface
        }
    }

    pub fn frame(&mut self)->&mut Frame{
        &mut self.surface
    }

    // Заполнение цветом
    pub fn clear_colour(&mut self,colour:[f32;4]){
        let (r,g,b,a)=(colour[0],colour[1],colour[2],colour[3]);
        self.surface.clear_color(r,g,b,a);
    }

    // fn clear_stencil(&mut self, value: u8) {
    //     self.surface.clear_stencil(value as i32);
    // }

    pub fn draw_simple<O:SimpleObject>(&mut self,object:&O,draw_parameters:&DrawParameters){
        self.system.draw_simple(object,draw_parameters,self.surface)
    }

    #[inline(always)] // Рисует один символ
    pub fn draw_character(&mut self,colour:Colour,character:&Character,draw_parameters:&DrawParameters){
        self.system.text.draw_character(character,colour,self.surface,draw_parameters);
    }

    pub fn draw_texture(&mut self,image_base:&ImageBase,texture:&Texture,draw_parameters:&DrawParameters){
        self.system.texture.draw_texture(image_base,texture,self.surface,draw_parameters)
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
                &uniform!{colour:colour},
                draw_parameters,
            );
        }
    }
}