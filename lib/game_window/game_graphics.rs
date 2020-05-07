use glium::index::{NoIndices, PrimitiveType};
use glium::{Program, Surface,VertexBuffer,implement_vertex,uniform,Display};
use graphics::color::gamma_srgb_to_linear;
use graphics::{self, DrawState, Graphics};
use shader_version::glsl::GLSL;
use shader_version::{OpenGL, Shaders};

use glium::uniforms::{Sampler, SamplerWrapFunction};
use std::cmp::min;

use crate::*;

const CHUNKS: usize = 100;

#[derive(Copy,Clone)]
struct PlainVertex{
    color:[f32; 4],
    pos: [f32; 2],
}

implement_vertex!(PlainVertex, color, pos);
#[derive(Copy, Clone)]
struct TexturedVertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

implement_vertex!(TexturedVertex, pos, uv);
pub struct Glium2d {
    pub colored_offset: usize,
    pub colored_draw_state: DrawState,
    plain_buffer: VertexBuffer<PlainVertex>,
    textured_buffer: VertexBuffer<TexturedVertex>,
    shader_texture: Program,
    shader_color: Program,
}

impl Glium2d {
    /// Creates a new `Glium2d`.
    pub fn new(opengl:OpenGL,window:&Display)->Glium2d{
        use shaders_graphics2d::{colored,textured};

        let src=|bytes|unsafe{std::str::from_utf8_unchecked(bytes)};
        let glsl=opengl.to_glsl();

        let plain_buffer =
            VertexBuffer::empty_dynamic(window, CHUNKS * graphics::BACK_END_MAX_VERTEX_COUNT)
                .unwrap();
        Glium2d {
            colored_offset: 0,
            colored_draw_state: Default::default(),
            plain_buffer: plain_buffer,
            textured_buffer: VertexBuffer::empty_dynamic(
                window,
                graphics::BACK_END_MAX_VERTEX_COUNT,
            )
            .unwrap(),
            shader_texture: Program::from_source(
                window,
                Shaders::new()
                    .set(GLSL::V1_20, src(textured::VERTEX_GLSL_120))
                    .set(GLSL::V1_50, src(textured::VERTEX_GLSL_150_CORE))
                    .get(glsl)
                    .unwrap(),
                Shaders::new()
                    .set(GLSL::V1_20, src(textured::FRAGMENT_GLSL_120))
                    .set(GLSL::V1_50, src(textured::FRAGMENT_GLSL_150_CORE))
                    .get(glsl)
                    .unwrap(),
                None,
            )
            .ok()
            .expect("failed to initialize textured shader"),
            shader_color: Program::from_source(
                window,
                Shaders::new()
                    .set(GLSL::V1_20, src(colored::VERTEX_GLSL_120))
                    .set(GLSL::V1_50, src(colored::VERTEX_GLSL_150_CORE))
                    .get(glsl)
                    .unwrap(),
                Shaders::new()
                    .set(GLSL::V1_20, src(colored::FRAGMENT_GLSL_120))
                    .set(GLSL::V1_50, src(colored::FRAGMENT_GLSL_150_CORE))
                    .get(glsl)
                    .unwrap(),
                None,
            )
            .ok()
            .expect("failed to initialize colored shader"),
        }
    }
}

use glium::Frame;

/// Graphics back-end.
pub struct GameGraphics<'d,'s>{
    pub system: &'d mut Glium2d,
    pub surface: &'s mut Frame,
}

impl<'d,'s> GameGraphics<'d,'s>{
    pub fn new(system:&'d mut Glium2d,surface:&'s mut Frame)->GameGraphics<'d,'s>{
        GameGraphics{
            system,
            surface
        }
    }

    pub fn frame(&mut self)->&mut Frame{
        &mut self.surface
    }

    pub fn flush_colored(&mut self){
        let slice=self.system.plain_buffer.slice(0..self.system.colored_offset).unwrap();

        self.surface.draw(
                slice,
                &NoIndices(PrimitiveType::TrianglesList),
                &self.system.shader_color,
                &uniform! {},
                &draw_state::convert_draw_state(&self.system.colored_draw_state),
            )
            .ok()
            .expect("failed to draw triangle list");

        self.system.colored_offset=0;
        self.system.plain_buffer.invalidate();
    }
}

/// Implemented by all graphics back-ends.
impl<'d,'s> Graphics for GameGraphics<'d,'s>{
    type Texture=Texture;

    /// Clears background with a color.
    fn clear_color(&mut self,color:[f32;4]){
        let color = gamma_srgb_to_linear(color);
        let (r, g, b, a) = (color[0], color[1], color[2], color[3]);
        self.surface.clear_color(r, g, b, a);
    }

    fn clear_stencil(&mut self, value: u8) {
        self.surface.clear_stencil(value as i32);
    }

    /// Renders list of 2d triangles.
    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], mut f: F)
    where F: FnMut(&mut dyn FnMut(&[[f32; 2]])),
    {
        let color = gamma_srgb_to_linear(*color);
        // Flush when draw state changes.
        if &self.system.colored_draw_state != draw_state {
            self.flush_colored();
            self.system.colored_draw_state = *draw_state;
        }
        f(&mut |vertices: &[[f32; 2]]| {
            let n = vertices.len();
            if self.system.colored_offset + n > CHUNKS * graphics::BACK_END_MAX_VERTEX_COUNT {
                self.flush_colored();
            }
            let slice = self
                .system
                .plain_buffer
                .slice(self.system.colored_offset..self.system.colored_offset + n)
                .unwrap();
            slice.write({
                &(0..n)
                    .map(|i| PlainVertex {
                        color: color,
                        pos: vertices[i],
                    })
                    .collect::<Vec<_>>()
            });
            self.system.colored_offset += n;
        })
    }

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &Texture,
        mut f: F,
    ) where
        F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]])),{

        let mut sampler = Sampler::new(&texture.0);
        sampler.1.wrap_function = (texture.1[0], texture.1[1], SamplerWrapFunction::Clamp);

        let color = gamma_srgb_to_linear(*color);
        if self.system.colored_offset > 0 {
            self.flush_colored();
        }
        f(&mut |vertices: &[[f32; 2]], texture_coords: &[[f32; 2]]| {
            let len = min(vertices.len(), texture_coords.len());

            self.system.textured_buffer.invalidate();
            let slice = self.system.textured_buffer.slice(0..len).unwrap();

            slice.write({
                &(0..len)
                    .map(|i| TexturedVertex {
                        pos: vertices[i],
                        // FIXME: The `1.0 - ...` is because of a wrong convention
                        uv: [texture_coords[i][0], 1.0 - texture_coords[i][1]],
                    })
                    .collect::<Vec<_>>()
            });

            self.surface
                .draw(
                    slice,
                    &NoIndices(PrimitiveType::TrianglesList),
                    &self.system.shader_texture,
                    &uniform! {
                        color: color,
                        s_texture: sampler
                    },
                    &draw_state::convert_draw_state(draw_state),
                )
                .ok()
                .expect("failed to draw triangle list");
        })
    }
}