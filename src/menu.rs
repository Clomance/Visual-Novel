use crate::*;


const button_margin:f64=10f64;

pub struct Menu<'a>{
    head:String,
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    buttons:Vec<MenuButton>,
    glyphs:GlyphCache<'a>
}

impl<'a> Menu<'a>{
    pub fn new(head:String,buttons_size:[f64;2],menu_buttons_text:&[&str],glyphs:GlyphCache<'a>)->Menu<'a>{
        let len=menu_buttons_text.len();
        let mut menu_buttons=Vec::with_capacity(len);

        let lenf64=len as f64;
        let menu_len=buttons_size[1]*lenf64+button_margin*(lenf64-1f64);


        let x1=unsafe{(Settings.window_size[0]-buttons_size[0])/2f64};
        let y1=unsafe{
            (Settings.window_size[1]-menu_len)/2f64
        };

        let mut rect=[
            x1,
            y1,
            buttons_size[0],
            buttons_size[1]
        ];
        for text in menu_buttons_text{
            let button=MenuButton::new(rect,text.to_string());
            menu_buttons.push(button);
            rect[1]+=buttons_size[1]+button_margin;
        }
        Self{
            head:head,
            x1:x1,
            y1:y1,
            x2:x1+buttons_size[0],
            y2:y1+menu_len,
            buttons:menu_buttons,
            glyphs:glyphs,
        }
    }

    pub fn clicked(&mut self)->Option<usize>{
        let x=unsafe{mouse_position[0]};
        let y=unsafe{mouse_position[1]};

        if self.x1<x && self.x2>x && self.y1<y && self.y2>y{
            for (c,button) in self.buttons.iter_mut().enumerate(){
                if button.clicked(){
                    return Some(c)
                }
            }
            None
        }
        else{
            None
        }
    }

    pub fn draw(&mut self,draw_state:&DrawState,transform:Matrix2d,g:&mut GlGraphics){
        for button in &mut self.buttons{
            button.draw(draw_state,transform,g,&mut self.glyphs);
        }
    }
}