use glium::{
    implement_vertex,
    Program,
    VertexBuffer,
    Display,
    Frame,
    DrawParameters,
};

const Points_limit:usize=100; // Максимальное количество точек для одного объекта

implement_vertex!(Point2D,position);
#[derive(Copy,Clone)]
pub struct Point2D{
    pub position:[f32;2],
}

impl Point2D{
    pub fn new(x:f32,y:f32)->Point2D{
        Self{
            position:[x,y]
        }
    }
}

// Для простых одноцветных объектов,
// состоящих из менее, чем Points_limit точек
// 
pub struct SimpleGraphics{
    pub vertex_buffer:VertexBuffer<Point2D>,
    pub program:Program,
}

impl SimpleGraphics{
    pub fn new(display:&Display,glsl:u16)->SimpleGraphics{
        let (vertex_shader,fragment_shader)=if glsl==120{(
            include_str!("shaders/120/simple_vertex_shader_120.glsl"),
            include_str!("shaders/120/simple_fragment_shader_120.glsl"),
        )}
        else{(
            include_str!("shaders/simple_vertex_shader.glsl"),
            include_str!("shaders/simple_fragment_shader.glsl"),
        )};

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,Points_limit).unwrap(),
            program:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
        }
    }

    #[inline(always)]
    pub fn draw<O:SimpleObject>(
        &self,
        object:&O,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
        object.draw_simple(draw_parameters,frame,self)
    }
}

pub trait SimpleObject{
    fn draw_simple(&self,draw_parameters:&mut DrawParameters,frame:&mut Frame,graphics:&SimpleGraphics);
}