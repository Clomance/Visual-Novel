use engine::{
    graphics::Graphics,
    Window,
    image::{
        ImageBase,
        Texture,
    },
    glium::DrawParameters,
};

const radius:f32=30f32;
const d_radius:f32=5f32;

// Иконка курсора мышки
// Загружает картинку их папки ресурсов
pub struct MouseCursorIcon{
    range:usize,
    image_base:ImageBase,
    shift:[f32;2],
    texture:Texture,
    radius:f32,
    visible:bool,
}

impl MouseCursorIcon{
    pub fn new(window:&mut Window)->MouseCursorIcon{
        let image_base=ImageBase::new([1f32;4],[0f32,0f32,radius,radius]);

        let range=window.graphics().bind_image(8..12usize,image_base.clone()).unwrap();

        Self{
            range,
            image_base,
            texture:Texture::from_path(window.display(),"resources/images/mouse_icon.png").unwrap(),
            shift:[0f32;2],
            radius:radius/2f32,
            visible:true,
        }
    }

    pub fn set_visible(&mut self,visible:bool){
        self.visible=visible
    }

    pub fn switch_visibility(&mut self){
        self.visible=!self.visible
    }

    // При нажатии кнопки мыши
    pub fn pressed(&mut self){
        self.image_base.x1+=d_radius;
        self.image_base.y1+=d_radius;
        self.image_base.x2-=d_radius;
        self.image_base.y2-=d_radius;
        self.radius-=d_radius;
    }

    // При освобождении кнопки мыши
    pub fn released(&mut self){
        self.image_base.x1-=d_radius;
        self.image_base.y1-=d_radius;
        self.image_base.x2+=d_radius;
        self.image_base.y2+=d_radius;
        self.radius+=d_radius;
    }

    #[inline(always)]
    pub fn draw(&self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        if self.visible{
            graphics.draw_move_range_image(
                self.range,
                &self.texture,
                self.image_base.colour_filter,
                self.shift,
                draw_parameters
            );
        }
    }
}