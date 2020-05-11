use super::{
    Texture,
    GameGraphics
};

use graphics::{
    DrawState,
    Graphics,
    triangulation,
    ImageSize,
    math::Matrix2d,
    types::Color
};


pub struct ImageBase{
    pub rect:[f32;4],
    pub color:Color,
}

impl ImageBase{
    pub fn new(color:Color,rect:[f32;4])->ImageBase{
        Self{
            rect,
            color,
        }
    }

    pub fn draw(&self,texture:&Texture,draw_state:&DrawState,transform:Matrix2d,g:&mut GameGraphics){
        let source_rectangle={
            let (w,h) = texture.get_size();
            [0f64,0f64,w as f64,h as f64]
        };

        let rect=[
            self.rect[0] as f64,
            self.rect[1] as f64,
            self.rect[2] as f64,
            self.rect[3] as f64,
        ];

        g.tri_list_uv(draw_state,&self.color,texture,|f|{
            f(&triangulation::rect_tri_list_xy(transform,rect),&triangulation::rect_tri_list_uv(texture,source_rectangle))
        });
    }
}

// pub struct ImageBaseSrc{
//     pub rect:[f64;4],
//     pub color:Color,
//     pub src_rect:[f64;4],
// }

// impl ImageBaseSrc{
//     pub fn new(color:Color,rect:[f64;4],src_rect:[f64;4])->ImageBaseSrc{
//         Self{
//             rect,
//             color,
//             src_rect,
//         }
//     }

//     pub fn draw(&self,texture:&Texture,draw_state:&DrawState,transform:Matrix2d,g:&mut GameGraphics){
//         g.tri_list_uv(draw_state,&self.color,texture,|f|{
//             f(&triangulation::rect_tri_list_xy(transform,self.rect),&triangulation::rect_tri_list_uv(texture,self.src_rect))
//         });
//     }
// }