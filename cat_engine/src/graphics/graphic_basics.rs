use crate::Colour;

use super::{Graphics,SimpleObject,Vertex2D};

use glium::{
    DrawError,
    DrawParameters,
    index::{
        NoIndices,
        PrimitiveType,
    },
};

/// Четырёхугольник. Quadrilateral.
#[derive(Clone)]
pub struct Quadrilateral{
    pub points:[Vertex2D;4],
    pub colour:Colour,
}

impl Quadrilateral{
    pub fn new(points:[Vertex2D;4],colour:Colour)->Quadrilateral{
        Self{
            points,
            colour
        }
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_simple(self,draw_parameters)
    }
}

impl<'a> SimpleObject<'a> for Quadrilateral{
    type Indices=NoIndices;
    fn colour(&self)->Colour{
        self.colour
    }

    fn point_buffer(&self)->Vec<Vertex2D>{
        let mut vec=Vec::with_capacity(4);

        for point in &self.points{
            vec.push(point.clone());
        }

        vec
    }

    fn indices(&self)->NoIndices{
        NoIndices(PrimitiveType::TriangleStrip)
    }
}



/// Прямоугольник.
#[derive(Clone)]
pub struct Rectangle{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour:Colour,
}

impl Rectangle{
    /// rect - [x1, y1, width, height]
    pub fn new(rect:[f32;4],colour:Colour)->Rectangle{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            colour
        }
    }

    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],colour:Colour)->Rectangle{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            colour
        }
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_simple(self,draw_parameters)
    }

    #[inline(always)]
    pub fn draw_shift(&self,shift:[f32;2],draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_shift_simple(self,shift,draw_parameters)
    }
}

impl<'a> SimpleObject<'a> for Rectangle{
    type Indices=NoIndices;
    fn colour(&self)->Colour{
        self.colour
    }

    fn point_buffer(&self)->Vec<Vertex2D>{
        let mut vec=Vec::with_capacity(4);

        vec.push(Vertex2D::new(self.x1,self.y1));
        vec.push(Vertex2D::new(self.x1,self.y2));
        vec.push(Vertex2D::new(self.x2,self.y1));
        vec.push(Vertex2D::new(self.x2,self.y2));

        vec
    }

    fn indices(&self)->NoIndices{
        NoIndices(PrimitiveType::TriangleStrip)
    }
}

#[derive(Clone)]
pub struct RectangleBorder{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub width:f32,
    pub colour:Colour,
}

impl RectangleBorder{
    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],width:f32,colour:Colour)->RectangleBorder{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            width,
            colour
        }
    }

    /// Converts a rectanlge to border.
    pub fn from_rectangle(rect:Rectangle,width:f32)->RectangleBorder{
        Self{
            x1:rect.x1,
            y1:rect.y1,
            x2:rect.x2,
            y2:rect.y2,
            width,
            colour:rect.colour
        }
    }

    pub fn rectangle_base(rect:Rectangle,width:f32,colour:Colour)->RectangleBorder{
        Self{
            x1:rect.x1,
            y1:rect.y1,
            x2:rect.x2,
            y2:rect.y2,
            width,
            colour
        }
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        draw_parameters.line_width=Some(self.width);
        graphics.draw_simple(self,draw_parameters)
    }

    #[inline(always)]
    pub fn draw_shift(&self,shift:[f32;2],draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        draw_parameters.line_width=Some(self.width);
        graphics.draw_shift_simple(self,shift,draw_parameters)
    }
}

impl<'a> SimpleObject<'a> for RectangleBorder{
    type Indices=NoIndices;
    fn colour(&self)->Colour{
        self.colour
    }

    fn point_buffer(&self)->Vec<Vertex2D>{
        let mut vec=Vec::with_capacity(4);

        vec.push(Vertex2D::new(self.x1,self.y1));
        vec.push(Vertex2D::new(self.x1,self.y2));
        vec.push(Vertex2D::new(self.x2,self.y2));
        vec.push(Vertex2D::new(self.x2,self.y1));

        vec
    }

    fn indices(&self)->NoIndices{
        NoIndices(PrimitiveType::LineLoop)
    }
}

/// Прямоугольник с рамкой.
#[derive(Clone)]
pub struct RectangleWithBorder{
    pub rect:Rectangle,
    pub border_width:f32,
    pub border_colour:Colour,
}

impl RectangleWithBorder{
    /// rect - [x1, y1, width, height]
    pub fn new(rect:[f32;4],colour:Colour)->RectangleWithBorder{
        Self{
            rect:Rectangle::new(rect,colour),
            border_width:1f32,
            border_colour:colour,
        }
    }

    /// rect - [x1, y1, x2, y2]
    pub const fn raw(rect:[f32;4],colour:Colour,width:f32,border_colour:Colour)->RectangleWithBorder{
        Self{
            rect:Rectangle::raw(rect,colour),
            border_width:width,
            border_colour,
        }
    }
    pub const fn border(mut self,width:f32,colour:Colour)->RectangleWithBorder{
        self.border_width=width;
        self.border_colour=colour;
        self
    }

    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        self.rect.draw(draw_parameters,graphics)?;
        let border=RectangleBorder::rectangle_base(self.rect.clone(),self.border_width,self.border_colour);
        border.draw(draw_parameters,graphics)
    }
}

pub struct Line{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub radius:f32,
    pub colour:Colour,
}

impl Line{
    /// rect - [x1, y1, x2, y2]
    pub const fn new(rect:[f32;4],radius:f32,colour:Colour)->Line{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            radius,
            colour,
        }
    }

    pub fn position(&self)->[f32;4]{
        [
            self.x1,
            self.y1,
            self.x2,
            self.y2,
        ]
    }

    pub fn set_position(&mut self,[x1,y1,x2,y2]:[f32;4]){
        self.x1=x1;
        self.y1=y1;
        self.x2=x2;
        self.y2=y2;
    }

    pub fn shift_y(&mut self,dy:f32){
        self.y1+=dy;
        self.y2+=dy;
    }

    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        draw_parameters.line_width=Some(self.radius);
        graphics.draw_simple(self,draw_parameters);
    }
}


impl<'a> SimpleObject<'a> for Line{
    type Indices=NoIndices;
    fn colour(&self)->Colour{
        self.colour
    }

    fn point_buffer(&self)->Vec<Vertex2D>{
        let mut vec=Vec::with_capacity(2);

        vec.push(Vertex2D::new(self.x1,self.y1));
        vec.push(Vertex2D::new(self.x2,self.y2));

        vec
    }

    fn indices(&self)->NoIndices{
        NoIndices(PrimitiveType::LinesList)
    }
}

const ellipse_points:usize=15; // Количество точек для эллипса

/// Круг с центром в точке (x, y)
/// и радиусом 'radius',
/// который заполняется цветом 'colour'.
pub struct Circle{
    pub x:f32,
    pub y:f32,
    pub radius:f32,
    pub colour:Colour,
}

impl Circle{
    /// circle - [x, y, radius]
    pub const fn new(circle:[f32;3],colour:Colour)->Circle{
        Self{
            x:circle[0],
            y:circle[1],
            radius:circle[2],
            colour
        }
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_simple(self,draw_parameters)
    }
}

impl<'a> SimpleObject<'a> for Circle{
    type Indices=NoIndices;

    fn colour(&self)->Colour{
        self.colour
    }

    fn point_buffer(&self)->Vec<Vertex2D>{
        let r_x=self.radius;
        let r_y=self.radius;

        let c_x=self.x;
        let c_y=self.y;

        let mut shape=vec![Vertex2D{position:[c_x,c_y]};4*ellipse_points+2];

        let dx=r_x/ellipse_points as f32;
        let mut x=dx;

        for c in 1..ellipse_points{
            let y=((r_x-x)*(r_x+x)).sqrt();
            
            shape[c].position=[c_x+x,c_y+y];

            shape[2*ellipse_points-c].position=[c_x+x,c_y-y];

            shape[2*ellipse_points+c].position=[c_x-x,c_y-y];

            shape[4*ellipse_points-c].position=[c_x-x,c_y+y];

            x+=dx;
        }

        shape[1].position=[c_x,c_y+r_y];
        shape[ellipse_points].position=[c_x+r_x,c_y];
        shape[2*ellipse_points].position=[c_x,c_y-r_y];
        shape[3*ellipse_points].position=[c_x-r_x,c_y];
        shape[4*ellipse_points].position=[c_x,c_y+r_y];

        shape
    }

    fn indices(&self)->NoIndices{
        NoIndices(PrimitiveType::TriangleFan)
    }
}

// Круг с центром в точке (x, y)
// и радиусов 'radius',
// который заполняется цветом 'colour'
// pub struct CircleWithBorder{
//     pub x:f32,
//     pub y:f32,
//     pub radius:f32,
//     pub colour:Colour,
//     pub border_radius:f32,
//     pub border_colour:Colour,
// }

// impl CircleWithBorder{
//     // rect - [x,y,radius]
//     pub const fn new(rect:[f32;3],colour:Colour)->CircleWithBorder{
//         Self{
//             x:rect[0],
//             y:rect[1],
//             radius:rect[2],
//             colour,
//             border_colour:colour,
//             border_radius:1f32,
//         }
//     }

//     pub fn border(mut self,radius:f32,colour:Colour)->CircleWithBorder{
//         self.border_colour=colour;
//         self.border_radius=radius;
//         self
//     }

//     #[inline(always)]
//     pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
//         graphics.draw_simple(self,draw_parameters)
//     }
// }

// // // // // // // // // // // // // // // // // // // // // // // // // //
// impl<'a> SimpleObject<'a> for CircleWithBorder{
//     type Indices=NoIndices;
//     fn colour(&self)->Colour{
//         self.colour
//     }

//     fn point_buffer(&self)->Vec<Vertex2D>{
//         let r_x=self.radius;
//         let r_y=self.radius;

//         let c_x=self.x;
//         let c_y=self.y;

//         let mut shape=vec![Vertex2D{position:[c_x,c_y]};4*ellipse_points+2];

//         let dx=r_x/ellipse_points as f32;
//         let mut x=dx;

//         for c in 1..ellipse_points{
//             let y=((r_x-x)*(r_x+x)).sqrt();
            
//             shape[c].position=[c_x+x,c_y+y];

//             shape[2*ellipse_points-c].position=[c_x+x,c_y-y];

//             shape[2*ellipse_points+c].position=[c_x-x,c_y-y];

//             shape[4*ellipse_points-c].position=[c_x-x,c_y+y];

//             x+=dx;
//         }

//         shape[1].position=[c_x,c_y+r_y];
//         shape[ellipse_points].position=[c_x+r_x,c_y];
//         shape[2*ellipse_points].position=[c_x,c_y-r_y];
//         shape[3*ellipse_points].position=[c_x-r_x,c_y];
//         shape[4*ellipse_points].position=[c_x,c_y+r_y];

//         shape
//     }

//     fn indices(&self)->NoIndices{
//         NoIndices(PrimitiveType::TriangleFan)
//     }
// }


//     fn draw_simple(&self,draw_parameters:&mut DrawParameters,frame:&mut Frame,graphics:&SimpleGraphics){
//         unsafe{
//             let k=window_center[0]/window_center[1];
//             let r_x=self.radius/window_center[0];
//             let r_y=self.radius/window_center[1];

//             let c_x=self.x/window_center[0]-1f32;
//             let c_y=1f32-self.y/window_center[1];

//             let mut shape=[Vertex2D{position:[c_x,c_y]};4*ellipse_points+2];

//             let dx=r_x/ellipse_points as f32;
//             let mut x=dx;

//             for c in 1..ellipse_points{
//                 let y=((r_x-x)*(r_x+x)).sqrt()*k;
                
//                 shape[c].position=[c_x+x,c_y+y];

//                 shape[2*ellipse_points-c].position=[c_x+x,c_y-y];

//                 shape[2*ellipse_points+c].position=[c_x-x,c_y-y];

//                 shape[4*ellipse_points-c].position=[c_x-x,c_y+y];

//                 x+=dx;
//             }

//             shape[1].position=[c_x,c_y+r_y];
//             shape[ellipse_points].position=[c_x+r_x,c_y];
//             shape[2*ellipse_points].position=[c_x,c_y-r_y];
//             shape[3*ellipse_points].position=[c_x-r_x,c_y];
//             shape[4*ellipse_points].position=[c_x,c_y+r_y];

//             let slice=graphics.vertex_buffer.slice(0..4*ellipse_points+2).unwrap();
//             slice.write(&shape);

//             let mut indices=NoIndices(PrimitiveType::TriangleFan);

//             frame.draw(slice,indices,&graphics.draw,&uniform!{colour:self.colour},draw_parameters);

//             indices=NoIndices(PrimitiveType::LineLoop);

//             let slice=graphics.vertex_buffer.slice(1..4*ellipse_points+1).unwrap();
//             draw_parameters.line_width=Some(self.border_radius);
//             frame.draw(slice,indices,&graphics.draw,&uniform!{colour:self.border_colour},draw_parameters);
//         }
//     }
// }