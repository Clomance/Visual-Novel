use crate::{
    Game,
};

use lib::{
    colours::White,
};

use cat_engine::{
    // statics
    mouse_cursor,
    window_center,
    // structs
    Window,
    WindowEvent,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,Texture},
};


const loading_screen_assets:usize=2;

pub struct LoadingScreen{
    cat:usize,
    cat_image_base:ImageObject,
    gear:usize,
    //thread:Option<JoinHandle()>,
}

impl LoadingScreen{
    pub fn new(window:&Window,graphics:&mut Graphics2D)->LoadingScreen{
        // Создание основы для иконки загрузки
        let cat=Texture::from_path("./resources/images/loading_screen_assets.png",window.display()).unwrap();
        graphics.add_texture(cat);

        // Шестерня
        let mut image_base=ImageObject::raw_uv(unsafe{[
                window_center[0]-200f32,
                window_center[1]-200f32,
                400f32,
                400f32
            ]},
            [
                0f32,
                0f32,
                1f32,
                1f32/3f32,
            ],
            White
        );
        let gear=graphics.add_textured_object(&image_base,loading_screen_assets).unwrap();

        // Кот
        image_base.set_new_raw_uv(unsafe{[
                window_center[0]-100f32,
                window_center[1]-100f32,
                200f32,
                200f32
            ]},
            [
                0f32,
                2f32/3f32,
                1f32,
                1f32,
            ],
            White
        );
        let cat=graphics.add_textured_object(&image_base,loading_screen_assets).unwrap();

        Self{
            cat,
            cat_image_base:image_base,
            gear,
        }
    }

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D)->Game{
        let mut frames=0u8;
        let mut angle=0f32;

        let mut result=Game::Next;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>{
                    result=Game::Exit;
                }

                WindowEvent::Update=>{
                    //if unsafe{!loading}{
                        // if let Some(thread)=self.thread.take(){
                        //     self.images=thread.join().expect("Ошибка начальной загрузки");
                        // }

                        // self.saved_drawables.clear();
                        // // Отчистка слоёв
                        // self.object_map.clear_layers();

                        // for _ in 0..3{
                        //     window.graphics2d().delete_last_textured_object();
                        // }
                        // self.

                        //window.stop_events();
                    //}

                    // 0.2 секунды глаза открыты, 0.05 секунды закрыты
                    if frames==20{
                        // cat_eyes_closed
                        self.cat_image_base.set_rect_uv([0f32,1f32/3f32,1f32,1f32/3f32]);
                        graphics.rewrite_textured_object_vertices(self.cat,&self.cat_image_base.vertices());
                    }
                    else if frames==25{
                        // cat
                        self.cat_image_base.set_raw_uv([0f32,2f32/3f32,1f32,1f32]);
                        graphics.rewrite_textured_object_vertices(self.cat,&self.cat_image_base.vertices());

                        frames=0;
                    }
                    frames+=1;

                    // 140 градусов в секунду
                    angle+=0.05;
                }
                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                    window.draw(graphics,|graphics|{
                        graphics.clear_colour(White);
                        // Рендеринг кота
                        graphics.draw_textured_object(self.cat).unwrap();
                        // Рендеринг шестерни
                        graphics.draw_rotate_textured_object(self.gear,unsafe{window_center},angle).unwrap();
                        // Рендеринг курсора
                        graphics.draw_shift_textured_object(0,[dx,dy]).unwrap();
                    }).unwrap();
                }
                _=>{

                }
            }
        });

        // Кот
        graphics.remove_last_textured_object();
        // Шестерня
        graphics.remove_last_textured_object();
        // loading_screen_asset
        graphics.remove_last_texture();

        result
    }
}