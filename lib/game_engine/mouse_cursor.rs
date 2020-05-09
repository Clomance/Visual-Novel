use super::window_center; // statics

use glium::{
    BlendingFunction,
    LinearBlendingFactor,
    Blend,
    Frame,
    implement_vertex,
    uniform,
    Display,
    Program,
    Surface,
    VertexBuffer,
    IndexBuffer,
    index::PrimitiveType,
    DrawParameters,
};

pub struct MouseCursor{
    position:[f64;2],
    saved_position:[f64;2],
}

impl MouseCursor{
    pub const fn new()->MouseCursor{
        Self{
            position:[0f64;2],
            saved_position:[0f64;2],
        }
    }

    #[inline(always)]
    pub fn position(&self)->[f64;2]{
        self.position
    }

    // Расстояние от курсора до центра экрана
    pub fn center_radius(&self)->[f64;2]{
        unsafe{[
            self.position[0]-window_center[0],
            self.position[1]-window_center[1]
        ]}
    }

    #[inline(always)] // Сохранение текущей позиции
    pub fn save_position(&mut self){
        self.saved_position=self.position;
    }

    // Сдвиг с сохранённого места
    pub fn saved_movement(&self)->(f64,f64){
        (
            self.position[0]-self.saved_position[0],
            self.position[1]-self.saved_position[1]
        )
    }

    #[inline(always)]
    pub fn set_position(&mut self,position:[f64;2]){
        self.position=position;
    }
}


// Иконка курсора мыши
// Сделана для прямого вывода на кадр
// Полностью ручкая настройка
// Требуется доработка

const radius:f32=30f32;

const points:usize=60usize; // Количество точек для иконки

const d_angle:f32=2f32*std::f32::consts::PI/points as f32;

const mouse_icon_color:(f32,f32,f32,f32)=(0.15,0.25,0.9,0.85);

implement_vertex!(Vertex2DPoint,position);
#[derive(Clone,Copy)]
struct Vertex2DPoint{
    position:[f32;2]
}

pub struct MouseCursorIcon{
    vertex_buffer:VertexBuffer<Vertex2DPoint>,
    vertex_buffer_pressed:VertexBuffer<Vertex2DPoint>,
    indices:IndexBuffer<u16>,
    program:Program,
    draw_parameters:DrawParameters<'static>,
    draw_function:fn(&Self,&mut Frame,(f32,f32))
}

impl MouseCursorIcon{
    pub fn new(display:&Display,window_size:[f32;2])->MouseCursorIcon{
        let r_x=radius/window_size[0];
        let r_y=radius/window_size[1];

        let mut shape=[Vertex2DPoint{position:[0f32;2]};points+1];
        let mut pressed_shape=[Vertex2DPoint{position:[0f32;2]};points+1];
        let mut indices=[0u16;3*(points+1)];
        let mut a=0f32;

        for c in 1..points+1{
            let (sin,cos)=a.sin_cos();
            let x=cos*r_x;
            let y=sin*r_y;
            shape[c].position=[x,y];
            pressed_shape[c].position=[x*0.8f32,y*0.8f32];
            a+=d_angle
        }

        for c in 0..points{
            let i=c as u16;
            let index=c*3;
            indices[index]=0u16;
            indices[index+1]=i;
            indices[index+2]=i+1;
        }

        indices[3*points]=0u16;
        indices[3*(points+1)-2]=1;
        indices[3*(points+1)-1]=points as u16;

        let vertex_buffer=VertexBuffer::new(display,&shape).unwrap();

        let vertex_buffer_pressed=VertexBuffer::new(display,&pressed_shape).unwrap();

        let vertex_shader_src=r#"
                #version 140

                in vec2 position;

                uniform float dx;
                uniform float dy;

                void main() {
                    vec2 pos = position;
                    pos.x += dx;
                    pos.y += dy;
                    gl_Position = vec4(pos, 0.0, 1.0);
                }
            "#;

        let fragment_shader_src=&format!(r#"
                #version 140

                out vec4 color;

                void main() {{
                    color = vec4{:?};
                }}
            "#,mouse_icon_color);

        let program=glium::Program::from_source(display,vertex_shader_src,fragment_shader_src,None).unwrap();

        let indices=IndexBuffer::new(display,PrimitiveType::TrianglesList,&indices).unwrap();

        let mut draw_parameters:DrawParameters=Default::default();

        draw_parameters.blend=Blend{
            color:BlendingFunction::Addition{
                source:LinearBlendingFactor::SourceAlpha,
                destination:LinearBlendingFactor::OneMinusSourceAlpha,
            },
            alpha:BlendingFunction::Addition{
                source:LinearBlendingFactor::One,
                destination:LinearBlendingFactor::One,
            },
            constant_value:(0.0,0.0,0.0,0.0),
        };
        Self{
            vertex_buffer,
            vertex_buffer_pressed,
            indices,
            program:program,
            draw_parameters,
            draw_function:Self::draw_common
        }
    }

    // При нажатии левой кнопки мыши
    pub fn pressed(&mut self){
        self.draw_function=Self::draw_pressed;
    }

    // При освобождении левой кнопки мыши
    pub fn released(&mut self){
        self.draw_function=Self::draw_common;
    }

    pub fn draw(&self,frame:&mut Frame,position:(f32,f32)){
        (self.draw_function)(self,frame,position)
    }

    fn draw_common(&self,frame:&mut Frame,position:(f32,f32)){
        frame.draw(&self.vertex_buffer,&self.indices,&self.program,
                &uniform!{dx:position.0,dy:position.1},&self.draw_parameters).unwrap();
    }

    fn draw_pressed(&self,frame:&mut Frame,position:(f32,f32)){
        frame.draw(&self.vertex_buffer_pressed,&self.indices,&self.program,
                &uniform!{dx:position.0,dy:position.1},&self.draw_parameters).unwrap();
    }
}