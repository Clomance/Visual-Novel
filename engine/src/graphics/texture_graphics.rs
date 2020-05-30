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

// Графическая основа для отрисовки текстур (картинок)
// Размер буферов регулируется вручную при создании

// Чтобы постоянно не загружать координаты для вывода,
// можно сохранить нужную область буфера и использовать её
// Лучше использовать конец для сохранения областей,
// а начало для постоянно обновляющихся значений
pub struct TextureGraphics{
    vertex_buffer:VertexBuffer<TexturedVertex>,
    vertex_buffer_ranges:Vec<Range<usize>>,
    draw:Program,
    draw_rotate:Program,
    draw_move:Program,
}

impl TextureGraphics{
    pub fn new(display:&Display,buffer_size:usize,glsl:u16)->TextureGraphics{
        let (rotation,moving,vertex_shader,fragment_shader)=if glsl==120{(
            include_str!("shaders/120/texture_rotation_vertex_shader_120.glsl"),
            include_str!("shaders/120/texture_moving_vertex_shader_120.glsl"),
            include_str!("shaders/120/texture_vertex_shader_120.glsl"),
            include_str!("shaders/120/texture_fragment_shader_120.glsl")
        )}
        else{(
            include_str!("shaders/texture_rotation_vertex_shader.glsl"),
            include_str!("shaders/texture_moving_vertex_shader.glsl"),
            include_str!("shaders/texture_vertex_shader.glsl"),
            include_str!("shaders/texture_fragment_shader.glsl")
        )};

        Self{
            vertex_buffer:VertexBuffer::empty_dynamic(display,buffer_size).unwrap(),
            vertex_buffer_ranges:Vec::<Range<usize>>::with_capacity(buffer_size),
            draw:Program::from_source(display,vertex_shader,fragment_shader,None).unwrap(),
            draw_rotate:Program::from_source(display,rotation,fragment_shader,None).unwrap(),
            draw_move:Program::from_source(display,moving,fragment_shader,None).unwrap(),
        }
    }

    // Переписывает координаты с начала буфера и выводит, игнорируя все области
    pub fn draw_image(
        &self,
        image_base:&ImageBase,
        texture:&Texture,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame,
    ){
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
        );
    }

    pub fn draw_rotate_image(
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
            TexturedVertex::new([x1,y1],[0.0,1.0]),
            TexturedVertex::new([x1,y2],[0.0,0.0]),
            TexturedVertex::new([x2,y1],[1.0,1.0]),
            TexturedVertex::new([x2,y2],[1.0,0.0])
        ];

        let slice=self.vertex_buffer.slice(0..4).unwrap();
        slice.write(&rect);

        frame.draw(
            slice,
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

// Функции для работы с областями
impl TextureGraphics{
    // Добавляет область
    // Записывает в неё данные
    // Области могут пересекаться
    // Возвращает номер (индекс) области
    pub fn bind_range(&mut self,range:Range<usize>,data:&[TexturedVertex])->Option<usize>{
        let i=self.vertex_buffer_ranges.len();

        let slice=self.vertex_buffer.slice(range.clone())?;
        slice.write(&data);

        self.vertex_buffer_ranges.push(range);

        Some(i)
    }

    // Удаляет выбранную область
    pub fn unbind(&mut self,index:usize){
        self.vertex_buffer_ranges.remove(index);
    }

    // Рисует выбранную облать
    pub fn draw_range<'a,I:Into<IndicesSource<'a>>>(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        indices:I,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
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
        );
    }

    // Рисует выбранную область, сдвигая координаты
    pub fn draw_move_range<'a,I:Into<IndicesSource<'a>>>(
        &self,
        index:usize,
        texture:&Texture,
        colour_filter:Colour,
        movement:[f32;2],
        indices:I,
        draw_parameters:&mut DrawParameters,
        frame:&mut Frame
    ){
        let range=self.vertex_buffer_ranges[index].clone();
        let slice=self.vertex_buffer.slice(range).unwrap();
        let uni=uniform!{
            tex:&texture.0,
            colour_filter:colour_filter,
            movement:movement
        };

        frame.draw(
            slice,
            indices,
            &self.draw_move,
            &uni,
            draw_parameters
        );
    }
}