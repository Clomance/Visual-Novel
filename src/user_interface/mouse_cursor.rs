use crate::*;

pub const wallpaper_movement_scale:f64=16f64;

const common_color:Color=[0.2,0.3,0.9,0.8];
const pressed_color:Color=[0.1,0.2,0.8,0.8];


const common_radius:f64=15f64;
const pressed_radius:f64=10f64;

const d_radius:f64=common_radius-pressed_radius;

const common_diametr:f64=common_radius*2f64;
const pressed_diametr:f64=pressed_radius*2f64;

const wallpaper_smooth:f64=2f64; // Сглаживание - количество кадров

pub struct MouseCursor{
    radius:f64,
    pub current_position:[f64;2],
    rect:[f64;4],
    cursor:Ellipse
}

impl MouseCursor{
    pub const fn new()->MouseCursor{
        Self{
            radius:common_radius,
            current_position:[0f64;2],
            rect:[0f64,0f64,common_diametr,common_diametr],
            cursor:Ellipse{
                color:common_color,
                border:None,
                resolution:360, // Количесво углов
            }
        }
    }

    pub fn get_position(&self)->[f64;2]{
        self.current_position
    }

    pub fn set_position(&mut self,current_position:[f64;2]){
        self.current_position=current_position;
        self.rect[0]=current_position[0]-self.radius;
        self.rect[1]=current_position[1]-self.radius;
    }

    // При нажатии левой кнопки мыши
    pub fn pressed(&mut self){
        self.cursor.color=pressed_color;
        self.radius=pressed_radius;

        self.rect[0]+=d_radius/2f64;
        self.rect[1]+=d_radius/2f64;
        self.rect[2]-=d_radius;
        self.rect[3]-=d_radius;
    }
    // При освобождении левой кнопки мыши
    pub fn released(&mut self){
        self.cursor.color=common_color;
        self.radius=common_radius;

        self.rect[0]-=d_radius/2f64;
        self.rect[1]-=d_radius/2f64;
        self.rect[2]+=d_radius;
        self.rect[3]+=d_radius;
    }

    pub fn shift(&mut self,d:[f64;2]){
        unsafe{
            self.current_position[0]+=d[0];
            
            if self.current_position[0]>window_width{
                self.current_position[0]=window_width;
                self.rect[0]=window_width-self.radius;
            }
            else if self.current_position[0]<0f64{
                self.current_position[0]=0f64;
                self.rect[0]=-self.radius;
            }
            else{
                self.rect[0]+=d[0];
            }

            self.current_position[1]+=d[1];

            if self.current_position[1]>window_height{
                self.current_position[1]=window_height;
                self.rect[1]=window_height-self.radius;
            }
            else if self.current_position[1]<0f64{
                self.current_position[1]=0f64;
                self.rect[1]=-self.radius;
            }
            else{
                self.rect[1]+=d[1];
            }
        }
    }

    pub fn movement(&mut self,event:&Event){
        if let Some(d)=event.mouse_relative_args(){
            self.shift(d)
        }
    }

    pub fn movement_wallpaper(&mut self,event:&Event,wallpaper:&mut Wallpaper){
        if let Some(mouse)=event.mouse_relative_args(){
            self.shift(mouse);

            // Радиус 
            let r_x=unsafe{
                window_center[0]-self.current_position[0]
            };

            let r_y=unsafe{
                window_center[1]-self.current_position[1]
            };

            // let x=if r_x.is_sign_negative(){
            //     -((-r_x).log2()-r_x/8f64)
            // }
            // else{
            //     r_x.log2()+r_x/8f64
            // };

            // let y=if r_y.is_sign_negative(){
            //     -((-r_y).log2()-r_y/8f64)
            // }
            // else{
            //     r_y.log2()+r_y/8f64
            // };

            let position=[
                r_x/wallpaper_movement_scale,
                r_y/wallpaper_movement_scale
            ];

            wallpaper.set_position(position);
        }
    }

    pub fn movement_wallpaper_saved(&mut self,wallpaper:&mut Wallpaper){
        let r_x=unsafe{
            window_center[0]-self.current_position[0]
        };

        let r_y=unsafe{
            window_center[1]-self.current_position[1]
        };

        let position=[
            r_x/wallpaper_movement_scale,
            r_y/wallpaper_movement_scale
        ];

        wallpaper.set_position(position);
    }

    pub fn draw(&self,context:&Context,graphics:&mut GlGraphics){
        self.cursor.draw(self.rect,&context.draw_state,context.transform,graphics)
    }
}
