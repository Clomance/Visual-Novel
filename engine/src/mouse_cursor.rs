use super::{
    // statics
    window_center,
    window_width,
    window_height,
};

use glium::{
    Frame,
    implement_vertex,
    uniform,
    Display,
    Program,
    Surface,
    VertexBuffer,
    index::{NoIndices,PrimitiveType},
    DrawParameters,
};

pub struct MouseCursor{
    position:[f32;2],
    saved_position:[f32;2],
}

impl MouseCursor{
    pub const fn new()->MouseCursor{
        Self{
            position:[0f32;2],
            saved_position:[0f32;2],
        }
    }

    #[inline(always)]
    pub fn position(&self)->[f32;2]{
        self.position
    }

    // Расстояние от курсора до центра экрана
    pub fn center_radius(&self)->[f32;2]{
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
    pub fn saved_movement(&self)->(f32,f32){
        (
            self.position[0]-self.saved_position[0],
            self.position[1]-self.saved_position[1]
        )
    }

    #[inline(always)]
    pub fn set_position(&mut self,position:[f32;2]){
        self.position=position;
    }
}


// Иконка курсора мыши
// Сделана для прямого вывода на кадр
// Полностью ручкая настройка
// Требуется доработка

const radius:f32=30f32;

const points:usize=16usize; // Количество точек для иконки

//const d_angle:f32=(std::f32::consts::PI)/(2f32*points as f32);

const mouse_icon_color:(f32,f32,f32,f32)=(0.15,0.25,0.9,0.85);

implement_vertex!(Vertex2DPoint,position);
#[derive(Clone,Copy)]
struct Vertex2DPoint{
    position:[f32;2]
}

pub struct MouseCursorIcon{
    vertex_buffer:VertexBuffer<Vertex2DPoint>,
    vertex_buffer_pressed:VertexBuffer<Vertex2DPoint>,
    program:Program,
    draw_function:fn(&Self,(f32,f32),&mut DrawParameters,&mut Frame)
}

impl MouseCursorIcon{
    pub fn new(display:&Display)->MouseCursorIcon{
        let (k,mut r_x, mut r_y)=unsafe{(
                window_width/window_height,
                radius/window_width,
                radius/window_height
        )};

        let mut shape=[Vertex2DPoint{position:[0f32;2]};4*points+2];
        let mut pressed_shape=[Vertex2DPoint{position:[0f32;2]};4*points+2];

        let dx=r_x/points as f32;
        let mut x=dx;

        for c in 1..points{
            let y=((r_x-x)*(r_x+x)).sqrt()*k;
            shape[c].position=[x,y];

            shape[2*points-c].position=[x,-y];

            shape[2*points+c].position=[-x,-y];

            shape[4*points-c].position=[-x,y];

            let (x_p,y_p)=(x*0.8f32,y*0.8f32);

            pressed_shape[c].position=[x_p,y_p];

            pressed_shape[2*points-c].position=[x_p,-y_p];

            pressed_shape[2*points+c].position=[-x_p,-y_p];

            pressed_shape[4*points-c].position=[-x_p,y_p];

            x+=dx;
        }

        shape[1].position=[0f32,r_y];
        shape[points].position=[r_x,0f32];
        shape[2*points].position=[0f32,-r_y];
        shape[3*points].position=[-r_x,0f32];
        shape[4*points].position=[0f32,r_y];

        r_x*=0.8f32;
        r_y*=0.8f32;

        pressed_shape[1].position=[0f32,r_y];
        pressed_shape[points].position=[r_x,0f32];
        pressed_shape[2*points].position=[0f32,-r_y];
        pressed_shape[3*points].position=[-r_x,0f32];
        pressed_shape[4*points].position=[0f32,r_y];

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

        Self{
            vertex_buffer,
            vertex_buffer_pressed,
            program:program,
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
}


// Функции отрисовки
impl MouseCursorIcon{
    pub fn draw(&self,position:(f32,f32),draw_parameters:&mut DrawParameters,frame:&mut Frame){
        (self.draw_function)(self,position,draw_parameters,frame)
    }

    fn draw_common(&self,position:(f32,f32),draw_parameters:&mut DrawParameters,frame:&mut Frame){
        frame.draw(
            &self.vertex_buffer,
            &NoIndices(PrimitiveType::TriangleFan),
            &self.program,
            &uniform!{dx:position.0,dy:position.1},
            draw_parameters
        ).unwrap();
    }

    fn draw_pressed(&self,position:(f32,f32),draw_parameters:&mut DrawParameters,frame:&mut Frame){
        frame.draw(
            &self.vertex_buffer_pressed,
            &NoIndices(PrimitiveType::TriangleFan),
            &self.program,
            &uniform!{dx:position.0,dy:position.1},
            draw_parameters
        ).unwrap();
    }
}