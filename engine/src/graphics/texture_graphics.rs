use crate::{
    // statics
    window_center,
    // structs
    image_base::ImageBase,
    game_texture::Texture,
};

use glium::{
    // macroses
    implement_vertex,
    uniform,
    // structs
    VertexBuffer,
    Program,
    Display,
    Frame,
    DrawParameters,
    index::{
        PrimitiveType,
        NoIndices
    },
    Surface,
};

implement_vertex!(TexturedVertex,position,tex_coords);
#[derive(Copy,Clone)]
struct TexturedVertex{
    position:[f32;2],
    tex_coords:[f32;2],
}


pub struct TextureGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex>,
    program:Program,
}

impl TextureGraphics{
    pub fn new(display:&Display)->TextureGraphics{
        let vertex_shader=r#"
            #version 140

            in vec2 position;
            in vec2 tex_coords;

            out vec2 v_tex_coords;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader=r#"
            #version 140

            in vec2 v_tex_coords;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,4).unwrap(),
            program:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
        }
    }

    pub fn draw_texture(&self,image_base:&ImageBase,texture:&Texture,frame:&mut Frame,draw_parameters:&DrawParameters){
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        
        let (x1,y1,x2,y2)=unsafe{(
            image_base.rect[0]/window_center[0]-1f32,
            1f32-image_base.rect[1]/window_center[1],

            (image_base.rect[0]+image_base.rect[2])/window_center[0]-1f32,
            1f32-(image_base.rect[1]+image_base.rect[3])/window_center[1]
        )};

        let rect=[
            TexturedVertex{
                position:[x1,y1],
                tex_coords:[0.0,1.0],
            },
            TexturedVertex{
                position:[x1,y2],
                tex_coords:[0.0,0.0],
            },
            TexturedVertex{
                position:[x2,y1],
                tex_coords:[1.0,1.0],
            },
            TexturedVertex{
                position:[x2,y2],
                tex_coords:[1.0,0.0],
            }
        ];
        
        self.vertex_buffer.write(&rect);

        frame.draw(
            &self.vertex_buffer,
            indices,
            &self.program,
            &uniform!{tex:&texture.0},
            draw_parameters
        );
    }
}