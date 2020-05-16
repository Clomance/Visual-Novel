use crate::{
    // statics
    window_center,
    // types
    Colour,
};

use super::{
    // structs
    GameGraphics,
    SimpleObject,
    SimpleGraphics,
    Point2D
};

use glium::{
    uniform,
    Frame,
    DrawParameters,
    index::{
        NoIndices,
        PrimitiveType,
    },
    Surface,
};

// Здесь собраны простые фигуры
// и основные функции к ним

// Прямоугольник
// Заполняется одним цветом
#[derive(Clone)]
pub struct Rectangle{
    pub x1:f32,
    pub y1:f32,
    pub x2:f32,
    pub y2:f32,
    pub colour:Colour,
}

impl Rectangle{
    // rect - [x1,y1,width,height]
    pub fn new(rect:[f32;4],colour:Colour)->Rectangle{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            colour
        }
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_simple(self,draw_parameters)
    }
}

impl SimpleObject for Rectangle{
    fn draw_simple(&self,graphics:&SimpleGraphics,frame:&mut Frame,draw_parameters:&DrawParameters){
        let slice=graphics.vertex_buffer.slice(0..6).unwrap();
        let indices=NoIndices(PrimitiveType::TrianglesList);

        let mut vec=Vec::with_capacity(6);
        unsafe{
            let x1=self.x1/window_center[0]-1f32;
            let y1=1f32-self.y1/window_center[1];

            let x2=self.x2/window_center[0]-1f32;
            let y2=1f32-self.y2/window_center[1];

            vec.push(Point2D{
                position:[x1,y1]
            });
            vec.push(Point2D{
                position:[x1,y2]
            });
            vec.push(Point2D{
                position:[x2,y1]
            });
            vec.push(Point2D{
                position:[x1,y2]
            });
            vec.push(Point2D{
                position:[x2,y1]
            });
            vec.push(Point2D{
                position:[x2,y2]
            });
        }

        slice.write(&vec);

        frame.draw(slice,indices,&graphics.program,&uniform!{colour:self.colour},draw_parameters);
    }
}

#[derive(Clone)]
pub struct RectangleWithBorder{
    pub rect:Rectangle,
    pub border_radius:f32,
    pub border_colour:Colour,
}

impl RectangleWithBorder{
    // rect - [x1,y1,width,height]
    pub fn new(rect:[f32;4],colour:Colour)->RectangleWithBorder{
        Self{
            rect:Rectangle::new(rect,colour),
            border_radius:1f32,
            border_colour:colour,
        }
    }

    pub fn border(mut self,radius:f32,colour:Colour)->RectangleWithBorder{
        self.border_radius=radius;
        self.border_colour=colour;
        self
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_simple(self,draw_parameters)
    }
}

impl SimpleObject for RectangleWithBorder{
    fn draw_simple(&self,graphics:&SimpleGraphics,frame:&mut Frame,draw_parameters:&DrawParameters){
        let mut vec=Vec::with_capacity(6);
        let (x1,y1,x2,y2)=unsafe{(
            self.rect.x1/window_center[0]-1f32,
            1f32-self.rect.y1/window_center[1],

            self.rect.x2/window_center[0]-1f32,
            1f32-self.rect.y2/window_center[1]
        )};
       
        // Закрашивание прямоугольника
        let mut slice=graphics.vertex_buffer.slice(0..6).unwrap();
        let mut indices=NoIndices(PrimitiveType::TrianglesList);

        vec.push(Point2D{
            position:[x1,y1]
        });
        vec.push(Point2D{
            position:[x1,y2]
        });
        vec.push(Point2D{
            position:[x2,y1]
        });
        vec.push(Point2D{
            position:[x1,y2]
        });
        vec.push(Point2D{
            position:[x2,y1]
        });
        vec.push(Point2D{
            position:[x2,y2]
        });

        slice.write(&vec);

        frame.draw(slice,indices,&graphics.program,&uniform!{colour:self.rect.colour},draw_parameters);

        // Обводка прямоугольника
        slice=graphics.vertex_buffer.slice(0..4).unwrap();
        indices=NoIndices(PrimitiveType::LineLoop);
        vec.clear();

        vec.push(Point2D{
            position:[x1,y1]
        });
        vec.push(Point2D{
            position:[x1,y2]
        });
        vec.push(Point2D{
            position:[x2,y2]
        });
        vec.push(Point2D{
            position:[x2,y1]
        });

        slice.write(&vec);

        frame.draw(slice,indices,&graphics.program,&uniform!{colour:self.border_colour},draw_parameters);
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
    // rect - [x1,y1,x2,y2]
    pub fn new(rect:[f32;4],radius:f32,colour:Colour)->Line{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[2],
            y2:rect[3],
            radius,
            colour,
        }
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_simple(self,draw_parameters)
    }
}


impl SimpleObject for Line{
    fn draw_simple(&self,graphics:&SimpleGraphics,frame:&mut Frame,draw_parameters:&DrawParameters){
        let mut vec=Vec::with_capacity(2);

        let (x1,y1,x2,y2)=unsafe{(
            self.x1/window_center[0]-1f32,
            1f32-self.y1/window_center[1],

            self.x2/window_center[0]-1f32,
            1f32-self.y2/window_center[1]
        )};
        
        let slice=graphics.vertex_buffer.slice(0..2).unwrap();
        let indices=NoIndices(PrimitiveType::LineLoop);

        vec.push(Point2D{
            position:[x1,y1]
        });
        vec.push(Point2D{
            position:[x2,y2]
        });

        slice.write(&vec);

        frame.draw(slice,indices,&graphics.program,&uniform!{colour:self.colour},draw_parameters);
    }
}

const ellipse_points:usize=15;

pub struct Ellipse{
    pub x1:f32,
    pub y1:f32,
    pub width:f32,
    pub height:f32,
    pub colour:Colour,
}

impl Ellipse{
    // rect - [x1,x2,width,height]
    pub fn rect(rect:[f32;4],colour:Colour)->Ellipse{
        Self{
            x1:rect[0],
            y1:rect[1],
            width:rect[2],
            height:rect[3],
            colour
        }
    }
}

// Круг с центром в точке (x, y)
// и радиусов 'radius',
// который заполняется цветом 'colour'
pub struct Circle{
    pub x:f32,
    pub y:f32,
    pub radius:f32,
    pub colour:Colour,
}

impl Circle{
    // rect - [x,y,radius]
    pub fn new(rect:[f32;3],colour:Colour)->Circle{
        Self{
            x:rect[0],
            y:rect[1],
            radius:rect[2],
            colour
        }
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut GameGraphics){
        graphics.draw_simple(self,draw_parameters)
    }
}

impl SimpleObject for Circle{
    fn draw_simple(&self,graphics:&SimpleGraphics,frame:&mut Frame,draw_parameters:&DrawParameters){
        unsafe{
            let k=window_center[0]/window_center[1];
            let r_x=self.radius/window_center[0];
            let r_y=self.radius/window_center[1];

            let c_x=self.x/window_center[0]-1f32;
            let c_y=1f32-self.y/window_center[1];

            let mut shape=[Point2D{position:[c_x,c_y]};4*ellipse_points+2];

            let dx=r_x/ellipse_points as f32;
            let mut x=dx;



            for c in 1..ellipse_points{
                let y=((r_x-x)*(r_x+x)).sqrt()*k;
                
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

            let slice=graphics.vertex_buffer.slice(0..4*ellipse_points+2).unwrap();
            slice.write(&shape);

            let indices=NoIndices(PrimitiveType::TriangleFan);

            frame.draw(slice,indices,&graphics.program,&uniform!{colour:self.colour},draw_parameters);
        }
    }
}