use crate::{
    // statics
    window_center,
    // structs
    image::{ImageBase,Texture},
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
        PrimitiveType, // enum
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
    draw:Program,
    draw_rotate:Program,
}

impl TextureGraphics{
    pub fn new(display:&Display)->TextureGraphics{
        let draw_rotate=include_str!("shaders/texture_rotation_vertex_shader.glsl");

        let vertex_shader=include_str!("shaders/texture_vertex_shader.glsl");

        let fragment_shader=include_str!("shaders/texture_fragment_shader.glsl");

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,4).unwrap(),
            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,draw_rotate,fragment_shader,None).unwrap(),
        }
    }

    pub fn draw_texture(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        frame:&mut Frame,
        draw_parameters:&DrawParameters
    ){
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        
        let (x1,y1,x2,y2)=unsafe{(
            image_base.x1/window_center[0]-1f32,
            1f32-image_base.y1/window_center[1],

            image_base.x2/window_center[0]-1f32,
            1f32-image_base.y2/window_center[1]
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
            &self.draw,
            &uniform!{
                tex:&texture.0,
                colour_filter:image_base.colour_filter
            },
            draw_parameters
        );
    }

    pub fn draw_rotate_texture(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        angle:f32,
        frame:&mut Frame,
        draw_parameters:&mut DrawParameters
    ){
        let indices=NoIndices(PrimitiveType::TriangleStrip);
        
        let (x1,y1,x2,y2)=(
            image_base.x1,
            image_base.y1,
            image_base.x2,
            image_base.y2
        );

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
            &self.draw_rotate,
            &uniform!{
                tex:&texture.0,
                angle:angle,
                window_center:unsafe{window_center},
                colour_filter:image_base.colour_filter,
            },
            draw_parameters
        );
    }
}