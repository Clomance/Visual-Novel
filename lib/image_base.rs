use crate::*;

use opengl_graphics::Texture;

use graphics::{
    DrawState,
    Graphics,
    triangulation,
    ImageSize,
    math::{
        Scalar,
        Matrix2d
    }
};


pub struct ImageBase{
    pub rect:[f64;4],
    pub color:Color,
}

impl ImageBase{
    pub fn new(color:Color,rect:[f64;4])->ImageBase{
        Self{
            rect,
            color,
        }
    }

    pub fn draw(&self,texture:&Texture,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        let source_rectangle={
            let (w,h) = texture.get_size();
            [0.0, 0.0, w as Scalar,h as Scalar]
        };

        g.tri_list_uv(draw_state,&self.color,texture,|f|{
            f(&triangulation::rect_tri_list_xy(transform,self.rect),&triangulation::rect_tri_list_uv(texture,source_rectangle))
        });
    }
}

pub struct ImageBaseSrc{
    pub rect:[f64;4],
    pub color:Color,
    pub src_rect:[f64;4],
}

impl ImageBaseSrc{
    pub fn new(color:Color,rect:[f64;4],src_rect:[f64;4])->ImageBaseSrc{
        Self{
            rect,
            color,
            src_rect,
        }
    }

    pub fn draw(&self,texture:&Texture,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        g.tri_list_uv(draw_state,&self.color,texture,|f|{
            f(&triangulation::rect_tri_list_xy(transform,self.rect),&triangulation::rect_tri_list_uv(texture,self.src_rect))
        });
    }
}