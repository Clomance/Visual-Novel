use crate::{
    // statics
    window_center,
    // types
    Colour,
    // structs
    image::{ImageBase,Texture},
};

use glium::{
    // macroses
    implement_vertex,
    uniform,
    // enums
    DrawError,
    // structs
    VertexBuffer,
    Program,
    Display,
    Frame,
    DrawParameters,
    index::{
        PrimitiveType, // enum
        NoIndices,
        IndicesSource, // enum
    },
    Surface, // trait
};

use core::ops::Range;

implement_vertex!(TexturedVertex,position,tex_coords);
#[derive(Copy,Clone)]
pub struct TexturedVertex{
    position:[f32;2],
    tex_coords:[f32;2],
}

impl TexturedVertex{
    pub const fn new(position:[f32;2],tex_coords:[f32;2])->TexturedVertex{
        Self{
            position,
            tex_coords,
        }
    }
}

/// Графическая основа для отрисовки текстур (изображений).
/// 
/// Размер буферов регулируется вручную при создании.
/// 
/// Чтобы постоянно не загружать координаты для вывода,
/// можно сохранить нужную область буфера и использовать её.
/// Лучше использовать конец для сохранения областей,
/// так как начало используется для постоянно обновляющихся значений.
pub struct TextureGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex>,
    vertex_buffer_ranges:Vec<Range<usize>>,
    draw:Program,
    draw_rotate:Program,
    draw_shift:Program,
}

impl TextureGraphics{
    pub fn new(display:&Display,buffer_size:usize,glsl:u16)->TextureGraphics{
        let (rotation,shift,vertex_shader,fragment_shader)=if glsl==120{(
            include_str!("shaders/120/texture_rotation_vertex_shader.glsl"),
            include_str!("shaders/120/texture_shift_vertex_shader.glsl"),
            include_str!("shaders/120/texture_vertex_shader.glsl"),
            include_str!("shaders/120/texture_fragment_shader.glsl")
        )}
        else{(
            include_str!("shaders/texture_rotation_vertex_shader.glsl"),
            include_str!("shaders/texture_shift_vertex_shader.glsl"),
            include_str!("shaders/texture_vertex_shader.glsl"),
            include_str!("shaders/texture_fragment_shader.glsl")
        )};

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,buffer_size).unwrap(),
            vertex_buffer_ranges:Vec::<Range<usize>>::with_capacity(buffer_size),
            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment_shader,None).unwrap(),
            draw_shift:Program::from_source(display,shift,fragment_shader,None).unwrap(),
        }
    }

    /// Строит объект с нуля и выводит, игнорируя все области.
    /// Переписывает координаты с начала буфера (0..4).
    pub fn draw_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);

        let slice=self.vertex_buffer.slice(0..4).unwrap();
        slice.write(&image_base.vertex_buffer());

        frame.draw(
            slice,
            indices,
            &self.draw,
            &uniform!{
                tex:&texture.0,
                colour_filter:image_base.colour_filter
            },
            draw_parameters
        )
    }

    /// Строит объект с нуля и выводит, игнорируя все области.
    /// Переписывает координаты с начала буфера (0..4).
    pub fn draw_shift_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        [dx,dy]:[f32;2],
        draw_parameters:&DrawParameters,
        frame:&mut Frame,
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);

        let shift=unsafe{[
            dx/window_center[0],
            -dy/window_center[1]
        ]};

        let slice=self.vertex_buffer.slice(0..4).unwrap();
        slice.write(&image_base.vertex_buffer());

        frame.draw(
            slice,
            indices,
            &self.draw_shift,
            &uniform!{
                tex:&texture.0,
                colour_filter:image_base.colour_filter,
                shift:shift
            },
            draw_parameters
        )
    }

    /// Строит объект с нуля и выводит под данным углом, игнорируя все области.
    /// Переписывает координаты с начала буфера (0..4).
    pub fn draw_rotate_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        angle:f32,
        frame:&mut Frame,
        draw_parameters:&DrawParameters
    )->Result<(),DrawError>{
        let indices=NoIndices(PrimitiveType::TriangleStrip);

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            tex:&texture.0,
            cos:cos,
            sin:sin,
            window_center:unsafe{window_center},
            colour_filter:image_base.colour_filter,
        };


        let slice=self.vertex_buffer.slice(0..4).unwrap();
        slice.write(&image_base.rotation_vertex_buffer());

        frame.draw(
            slice,
            indices,
            &self.draw_rotate,
            &uni,
            draw_parameters
        )
    }
}

/// Функции для работы с областями.
impl TextureGraphics{
    /// Добавляет область и записывает в неё данные.
    /// Возвращает номер (индекс) области.
    /// 
    /// Области могут пересекаться.
    pub fn bind_range(&mut self,range:Range<usize>,data:&[TexturedVertex])->Option<usize>{
        let i=self.vertex_buffer_ranges.len();

        let slice=self.vertex_buffer.slice(range.clone())?;
        slice.write(&data);

        self.vertex_buffer_ranges.push(range);

        Some(i)
    }

    pub fn pop_range(&mut self)->Option<Range<usize>>{
        self.vertex_buffer_ranges.pop()
    }

    /// 
    pub fn rewrite_range(&mut self,range:usize,data:&[TexturedVertex])->Option<()>{
        let range=self.vertex_buffer_ranges.get(range)?;
        let slice=self.vertex_buffer.slice(range.clone())?;
        slice.write(&data);
        Some(())
    }

    /// Удаляет выбранную область без проверки.
    pub fn unbind(&mut self,index:usize){
        self.vertex_buffer_ranges.remove(index);
    }

    /// Рисует выбранную область.
    pub fn draw_range<'a,I:Into<IndicesSource<'a>>>(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        indices:I,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let range=self.vertex_buffer_ranges[index].clone();
        let slice=self.vertex_buffer.slice(range).unwrap();
        let uni=uniform!{
            tex:&texture.0,
            colour_filter:colour_filter,
        };

        frame.draw(
            slice,
            indices,
            &self.draw,
            &uni,
            draw_parameters
        )
    }

    /// Рисует выбранную область, сдвигая координаты.
    pub fn draw_shift_range<'a,I:Into<IndicesSource<'a>>>(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        [dx,dy]:[f32;2],
        indices:I,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let range=self.vertex_buffer_ranges[index].clone();
        let slice=self.vertex_buffer.slice(range).unwrap();

        let shift=unsafe{[
            dx/window_center[0],
            -dy/window_center[1]
        ]};

        let uni=uniform!{
            tex:&texture.0,
            colour_filter:colour_filter,
            shift:shift
        };

        frame.draw(
            slice,
            indices,
            &self.draw_shift,
            &uni,
            draw_parameters
        )
    }

    /// Рисует выбранную область, поворачивая координаты.
    /// 
    /// Угол в радианах.
    pub fn draw_rotate_range<'a,I:Into<IndicesSource<'a>>>(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        angle:f32,
        indices:I,
        draw_parameters:&DrawParameters,
        frame:&mut Frame
    )->Result<(),DrawError>{
        let range=self.vertex_buffer_ranges[index].clone();
        let slice=self.vertex_buffer.slice(range).unwrap();

        let (sin,cos)=angle.sin_cos();

        let uni=uniform!{
            tex:&texture.0,
            cos:cos,
            sin:sin,
            window_center:unsafe{window_center},
            colour_filter:colour_filter,
        };

        frame.draw(
            slice,
            indices,
            &self.draw_rotate,
            &uni,
            draw_parameters
        )
    }
}