use crate::{Colour,window_center};

use glium::{
    uniform,
    implement_vertex,
    Program,
    Display,
    Frame,
    DrawParameters,
    DrawError,
    index::{NoIndices,PrimitiveType,IndicesSource},
    Surface,
    vertex::{VertexBuffer,VertexBufferSlice},
};

use core::ops::Range;

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

    pub fn convert(&mut self){
        unsafe{
            self.position[0]=self.position[0]/window_center[0]-1f32;
            self.position[1]=1f32-self.position[1]/window_center[1];
        }
    }
}

/// Графическая основа для простых одноцветных объектов.
pub struct SimpleGraphics{
    vertex_buffer:VertexBuffer<Point2D>,
    vertex_buffer_ranges:Vec<Range<usize>>,
    draw:Program,
    draw_move:Program,
}

impl SimpleGraphics{
    pub fn new(display:&Display,buffer_size:usize,glsl:u16)->SimpleGraphics{
        let (movement,vertex_shader,fragment_shader)=if glsl==120{(
            include_str!("shaders/120/simple_movement_vertex_shader.glsl"),
            include_str!("shaders/120/simple_vertex_shader.glsl"),
            include_str!("shaders/120/simple_fragment_shader.glsl"),
        )}
        else{(
            include_str!("shaders/simple_movement_vertex_shader.glsl"),
            include_str!("shaders/simple_vertex_shader.glsl"),
            include_str!("shaders/simple_fragment_shader.glsl"),
        )};

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,buffer_size).unwrap(),
            vertex_buffer_ranges:Vec::<Range<usize>>::with_capacity(buffer_size),
            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_move:Program::from_source(display,movement,fragment_shader,None).unwrap(),
        }
    }

    /// Вписывает в буфер данные, начиная с начала.
    pub fn write_vertex(&self,data:&[Point2D])->Option<VertexBufferSlice<Point2D>>{
        let slice=self.vertex_buffer.slice(0..data.len())?;
        slice.write(&data);
        Some(slice)
    }

    pub fn draw<'a,O:SimpleObject<'a>>(
        &self,
        object:&O,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let mut points=object.point_buffer();

        for point in points.iter_mut(){
            point.convert();
        }

        let slice=self.write_vertex(&points).unwrap();
        let indices:O::Indices=object.indices();
        let uni=uniform!{
            colour:object.colour()
        };

        frame.draw(slice,indices,&self.draw,&uni,draw_parameters)
    }

    pub fn draw_move<'a,O:SimpleObject<'a>>(
        &self,
        object:&O,
        [dx,dy]:[f32;2],
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let mut points=object.point_buffer();

        for point in points.iter_mut(){
            point.convert();
        }

        let movement=unsafe{[
            dx/window_center[0],
            -dy/window_center[1]
        ]};

        let slice=self.write_vertex(&points).unwrap();
        let indices:O::Indices=object.indices();
        let uni=uniform!{
            colour:object.colour(),
            movement:movement,
        };

        frame.draw(slice,indices,&self.draw_move,&uni,draw_parameters)
    }
}

// Функции для работы с областями
impl SimpleGraphics{
    /// Добавляет область и записывает в неё данные.
    /// 
    /// Возвращает номер (индекс) области.
    /// 
    /// Области могут пересекаться.
    pub fn bind_range(&mut self,range:Range<usize>,data:&[Point2D])->Option<usize>{
        let i=self.vertex_buffer_ranges.len();

        let slice=self.vertex_buffer.slice(range.clone())?;
        slice.write(&data);

        self.vertex_buffer_ranges.push(range);

        Some(i)
    }

    /// Удаляет выбранную область, без проверки.
    pub fn unbind(&mut self,index:usize){
        self.vertex_buffer_ranges.remove(index);
    }

    /// Рисует выбранную область, без проверки.
    pub fn draw_range(
        &self,
        index:usize,
        colour:Colour,
        draw_type:PrimitiveType,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let range=self.vertex_buffer_ranges[index].clone();
        let slice=self.vertex_buffer.slice(range).unwrap();
        let indices=NoIndices(draw_type);
        let uni=uniform!{
            colour:colour,
        };

        frame.draw(
            slice,
            indices,
            &self.draw,
            &uni,
            draw_parameters
        )
    }
}

/// Типаж для создания собственных простых одноцветных объектов
pub trait SimpleObject<'a>{
    type Indices:Into<IndicesSource<'a>>;

    /// Цвет объекта.
    /// 
    /// Object's colour.
    fn colour(&self)->Colour;

    /// Точки объекта в оконных координатах (без приведению к формату OpenGL).
    fn point_buffer(&self)->Vec<Point2D>;

    /// Индексы для построения объекта.
    fn indices(&self)->Self::Indices;
}