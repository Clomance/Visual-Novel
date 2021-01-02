use crate::{
    // consts
    game_name,
    mouse_cursor_icon_index,
    wallpaper_index,
    wallpaper_movement_scale,
    swipe_screen_index,
    swipe_updates,
    // statics
    game_settings,
    // enums
    Game,
    // functions
    get_swipe_texture,
    draw_on_texture
};

use super::{
    // structs
    Settings,
    // consts
    button_pressed,
    // enums
    SwipeDirection,
};

use lib::{
    colours::{White,Gray,Light_blue},
    user_interface::{
        Menu,
        MenuSettings,
        EditTextView,
        EditTextViewSettings,
    },
};

use cat_engine::{
    // types
    Colour,
    // statics
    mouse_cursor,
    window_center,
    window_height,
    window_width,
    // enums
    KeyboardButton,
    // structs
    Window,
    WindowEvent,
    MouseButton,
    graphics::{Graphics2D,DependentObject},
    texture::{ImageObject,ImageBase,Texture},
    image::RgbaImage,

    audio::AudioWrapper,
};



const menu_movement_scale:f32=10f32;

const leaf_movement_scale:f32=12f32;

const x_max_speed:f32=3f32;
const x_min_speed:f32=0f32;

const y_max_speed:f32=2f32;
const y_min_speed:f32=-1f32;

const x_accelerate:f32=1f32/32f32;
const y_accelerate:f32=1f32/32f32;

const leaf_spawn_times:&[u16]=&[
    44,153,10,93,
    588,97,126,642,
    323,65,23,224,
    123,345,49,342,
    164,32,471
];
const leaf_spawn_positions:&[f32]=&[
    0.123f32,0.42f32,0.73f32,0.154f32,
    0.223f32,0.96f32,0.5f32,0.3451f32,
    0.67f32,0.12f32,0.7521f32,0.542f32,
    0.241f32,0.82f32,0.87f32,0.4111f32
];

struct Leaf{
    x:f32,
    y:f32,
    x_speed:f32,
    y_speed:f32,
    left_side:bool,
    x_up:bool,
    y_up:bool
}

impl Leaf{
    pub fn new(x:f32)->Leaf{
        Self{
            x,
            y:0f32,
            x_speed:0f32,
            y_speed:0f32,
            // Движение из левой части в правую
            // или из правой в левую
            left_side:true,
            // Ускорение или замедление
            x_up:true,
            y_up:true
        }
    }
}

pub struct MainMenu{
    leaf:usize,
    leaves:Vec<Leaf>,
    menu:Menu,
    enter_name:bool,
    user_name:EditTextView,
}

impl MainMenu{
    pub fn new(window:&Window,graphics:&mut Graphics2D,images:&[RgbaImage])->MainMenu{
        // Изменение картинки обоев
        graphics.get_textured_object_texture(wallpaper_index).update(&images[0]);

        let leaf=Texture::from_image(&images[1],window.display()).unwrap();
        let leaf=graphics.add_texture(leaf);
        let leaf_image_base=ImageBase::new([0f32,-100f32,100f32,100f32],White);
        let leaf=graphics.add_textured_object(&leaf_image_base,leaf).unwrap();


        let mut buttons=Vec::with_capacity(4);
        if unsafe{game_settings.continue_game}{
            buttons.push("Продолжить");
        }
        buttons.push("Новая игра");
        buttons.push("Настройки");
        buttons.push("Выход");

        let menu_settings=MenuSettings::new(game_name,buttons.into_iter())
                .header_font_size(60f32)
                .button_size([160f32,60f32])
                .button_font_size(26f32);

        let enter_name_rect=unsafe{[
            window_center[0]-120f32,
            window_center[1]-100f32,
            240f32,
            140f32,
        ]};
        let enter_name_settings=EditTextViewSettings::new("",enter_name_rect);

        Self{
            leaf,
            leaves:Vec::with_capacity(10),
            menu:Menu::new(menu_settings,graphics),
            enter_name:false,
            user_name:EditTextView::new(enter_name_settings,graphics),
        }
    }

    pub fn open(&mut self,window:&mut Window,swipe_direction:SwipeDirection,graphics:&mut Graphics2D)->Game{
        let mut result=Game::Next;

        let mut frames=0u8;

        let mut current_page_shift=[0f32;2];

        let (
            mut next_page_shift,
            dshift
        )=unsafe{
            match swipe_direction{
                SwipeDirection::Up=>(
                    [
                        0f32,
                        window_height
                    ],
                    [
                        0f32,
                        -window_height/swipe_updates as f32
                    ]
                ),
                SwipeDirection::Down=>(
                    [
                        0f32,
                        -window_height
                    ],
                    [
                        0f32,
                        window_height/swipe_updates as f32
                    ]
                ),
                SwipeDirection::Left=>(
                    [
                        window_width,
                        0f32
                    ],
                    [
                        -window_width/swipe_updates as f32,
                        0f32
                    ]
                ),
                SwipeDirection::Right=>(
                    [
                        -window_width,
                        0f32
                    ],
                    [
                        window_width/swipe_updates as f32,
                        0f32
                    ]
                )
            }
        };

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>result=Game::Exit,
                WindowEvent::Update=>{
                    frames+=1;
                    if frames==swipe_updates{
                        window.stop_events();
                    }
                    else{
                        current_page_shift[0]+=dshift[0];
                        current_page_shift[1]+=dshift[1];

                        next_page_shift[0]+=dshift[0];
                        next_page_shift[1]+=dshift[1];
                    }
                }

                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};

                    let wallpaper_shift=[
                        dx/wallpaper_movement_scale+next_page_shift[0],
                        dy/wallpaper_movement_scale+next_page_shift[1]
                    ];

                    let menu_shift=[
                        dx/menu_movement_scale+next_page_shift[0],
                        dy/menu_movement_scale+next_page_shift[1]
                    ];

                    let leaf_shift=[
                        dx/leaf_movement_scale+next_page_shift[0],
                        dy/leaf_movement_scale+next_page_shift[1]
                    ];

                    window.draw(&graphics,|graphics|{
                        graphics.draw_shift_textured_object(swipe_screen_index,current_page_shift);
                        graphics.draw_shift_textured_object(wallpaper_index,wallpaper_shift).unwrap();


                        for leaf in &self.leaves{
                            let leaf=[
                                leaf.x+leaf_shift[0],
                                leaf.y+leaf_shift[1]
                            ];
                            graphics.draw_shift_textured_object(self.leaf,leaf).unwrap();
                        }

                        self.menu.draw_shift(menu_shift,graphics);
                    });
                }

                _=>{}
            }
        });

        result
    }

    pub fn run(&mut self,window:&mut Window,graphics:&mut Graphics2D,audio:&AudioWrapper)->Game{
        let mut result=Game::Next;

        let mut frames=0u16;
        let mut spawn_time=0usize;
        let mut spawn_position=0usize;

        window.run(|window,event|{
            match event{
                WindowEvent::CloseRequested=>result=Game::Exit,

                WindowEvent::Update=>{
                    frames+=1;

                    if frames==leaf_spawn_times[spawn_time]{
                        frames=0;

                        spawn_time+=1;

                        if spawn_time==leaf_spawn_times.len(){
                            spawn_time=0;
                        }

                        let x=leaf_spawn_positions[spawn_position]*unsafe{window_width};

                        spawn_position+=1;
                        if spawn_position==leaf_spawn_positions.len(){
                            spawn_position=0;
                        }

                        self.leaves.push(Leaf::new(x));
                    }

                    let mut c=0;
                    while c<self.leaves.len(){
                        let leaf=&mut self.leaves[c];

                        if leaf.y>unsafe{window_height}+100f32{
                            // Удаление упавших лепестков
                            self.leaves.remove(c);
                            continue
                        }
                        else{
                            // Движение лепестков
                            if leaf.left_side{
                                leaf.x+=leaf.x_speed;
                            }
                            else{
                                leaf.x-=leaf.x_speed;
                            }

                            leaf.y+=leaf.y_speed;

                            if leaf.x_up{
                                leaf.x_speed+=x_accelerate;
                                if leaf.x_speed>=x_max_speed{
                                    leaf.x_up=false;
                                }
                            }
                            else{
                                leaf.x_speed-=x_accelerate;

                                if leaf.x_speed<=x_min_speed{
                                    leaf.x_up=true;
                                    leaf.left_side=!leaf.left_side;
                                }
                            }

                            if leaf.y_up{
                                leaf.y_speed+=y_accelerate;
                                if leaf.y_speed>=y_max_speed{
                                    leaf.y_up=false;
                                }
                            }
                            else{
                                leaf.y_speed-=y_accelerate;

                                if leaf.y_speed<=y_min_speed{
                                    leaf.y_up=true;
                                }
                            }
                        }
                        // Следующий лепесток
                        c+=1;
                    }
                }

                WindowEvent::RedrawRequested=>{
                    let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                    let wallpaper_shift=[
                        dx/wallpaper_movement_scale,
                        dy/wallpaper_movement_scale
                    ];
                    let menu_shift=[
                        dx/menu_movement_scale,
                        dy/menu_movement_scale
                    ];

                    let leaf_shift=[
                        dx/leaf_movement_scale,
                        dy/leaf_movement_scale
                    ];

                    window.draw(graphics,|graphics|{
                        // Отрисовка обоев
                        graphics.draw_shift_textured_object(wallpaper_index,wallpaper_shift).unwrap();

                        for leaf in &self.leaves{
                            let leaf=[
                                leaf.x+leaf_shift[0],
                                leaf.y+leaf_shift[1]
                            ];
                            graphics.draw_shift_textured_object(self.leaf,leaf).unwrap();
                        }

                        // Отрисовка меню
                        self.menu.draw_shift(menu_shift,graphics);

                        if self.enter_name{
                            self.user_name.draw(graphics);
                        }

                        // Отрисовка курсора
                        graphics.draw_shift_textured_object(mouse_cursor_icon_index,[dx,dy]).unwrap();
                    }).unwrap();
                }

                WindowEvent::MousePressed(button)=>{
                    if let MouseButton::Left=button{
                        let [mut x,mut y]=unsafe{mouse_cursor.position()};

                        if self.enter_name{
                            if !self.user_name.in_area(x,y){
                                self.enter_name=false;
                            }
                        }
                        else{
                            let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                            let menu_shift=[
                                dx/menu_movement_scale,
                                dy/menu_movement_scale
                            ];

                            x-=menu_shift[0];
                            y-=menu_shift[1];
                            if let Some(button)=self.menu.pressed(x,y){
                                audio.play_track("button_pressed",1u32);
                                // Получение индекса кнопки
                                let button_index=self.menu.button_index(button);
                                // Изменение цвета кнопки
                                *graphics.get_simple_object_colour(button_index)=button_pressed;
                            }
                        }
                    }
                }

                WindowEvent::MouseReleased(button)=>{
                    if let MouseButton::Left=button{
                        if !self.enter_name{
                            if let Some(pressed_button)=self.menu.pressed_button(){
                                // Текущее положение курсора
                                let [mut x,mut y]=unsafe{mouse_cursor.position()};
                                // Расстояние от курсора до центра экрана
                                let [dx,dy]=unsafe{mouse_cursor.center_radius()};
                                // Сдвиг меню
                                let menu_shift=[
                                    dx/menu_movement_scale,
                                    dy/menu_movement_scale
                                ];
                                // Корректировка положения курсора
                                // относительно сдвинутого меню
                                x-=menu_shift[0];
                                y-=menu_shift[1];

                                // Получение индекса кнопки
                                let button_index=self.menu.button_index(pressed_button);
                                // Изменение цвета кнопки
                                *graphics.get_simple_object_colour(button_index)=Light_blue;

                                if let Some(mut button)=self.menu.released(x,y){
                                    if unsafe{!game_settings.continue_game}{
                                        button+=1;
                                    }

                                    match button{
                                        // Продолжить игру
                                        0=>{
                                            window.stop_events();
                                        }

                                        // Начать новую игру
                                        1=>{
                                            // Открытие диалога для ввода имени пользователя
                                            self.enter_name=true;
                                        }

                                        // Настройки
                                        2=>{
                                            self.render_to_texture(window,graphics);

                                            match{
                                                let mut settings=Settings::new(window,graphics);
                                                settings.open(window,graphics);
                                                settings.run(window,graphics,audio)
                                            }{
                                                Game::Exit=>{
                                                    result=Game::Exit;
                                                    window.stop_events();
                                                }
                                                _=>{
                                                    self.open(window,SwipeDirection::Right,graphics);
                                                }
                                            }
                                        }

                                        // Выход
                                        3=>{
                                            window.stop_events();
                                            result=Game::Exit;
                                        }

                                        _=>{}
                                    }
                                }
                            }
                        }
                    }
                }

                WindowEvent::CharacterInput(character)=>if self.enter_name{
                    self.user_name.push_char(character,graphics);
                }

                WindowEvent::KeyboardPressed(button)=>match button{
                    KeyboardButton::Escape=>self.enter_name=false,

                    KeyboardButton::Backspace=>if self.enter_name{
                        self.user_name.pop_char(graphics);
                    }

                    KeyboardButton::Enter=>if self.enter_name{
                        unsafe{game_settings.user_name=self.user_name.text(graphics).clone()}
                        window.stop_events();
                    }

                    KeyboardButton::F5=>unsafe{
                        audio.play_track("screenshot",1u32);
                        let path=format!("./screenshots/screenshot{}.png",game_settings.screenshot);
                        game_settings.screenshot+=1;
                        window.save_screenshot(path);
                    }
                    _=>{}
                }

                _=>{

                }
            }
        });

        self.render_to_texture(window,graphics);

        // Удаление всех простых объектов
        graphics.remove_all_simple_objects();
        // Удаление всех текстовых объектов
        graphics.remove_all_text_objects();
        result
    }

    fn render_to_texture(&self,window:&Window,graphics:&mut Graphics2D){
        // Расстояние от курсора до центра экрана
        let [dx,dy]=unsafe{mouse_cursor.center_radius()};

        let swipe_screen_texture=get_swipe_texture(graphics);

        let wallpaper_shift=[
            dx/wallpaper_movement_scale,
            dy/wallpaper_movement_scale
        ];

        let menu_shift=[
            dx/menu_movement_scale,
            dy/menu_movement_scale
        ];

        let leaf_shift=[
            dx/leaf_movement_scale,
            dy/leaf_movement_scale
        ];

        draw_on_texture(&swipe_screen_texture,window,graphics,|graphics|{
            graphics.draw_shift_textured_object(wallpaper_index,wallpaper_shift).unwrap();

            for leaf in &self.leaves{
                let leaf=[
                    leaf.x+leaf_shift[0],
                    leaf.y+leaf_shift[1]
                ];
                graphics.draw_shift_textured_object(self.leaf,leaf).unwrap();
            }

            // Отрисовка меню
            self.menu.draw_shift(menu_shift,graphics);
        });
    }
}