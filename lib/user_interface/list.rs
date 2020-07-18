use cat_engine::{
    mouse_cursor,
    Colour,
    graphics::{
        Graphics,
        RectangleWithBorder,
        Line,
    },
    text::{TextBase,Glyphs},
    glium::DrawParameters,
};

use crate::colours::{Light_blue,Black};

const dcolour:f32=0.125; // На столько измененяется цвет при нажитии/освобождении

const item_margin:f32=5f32;

#[derive(Clone)]
pub enum ListFocus{
    None,
    Base,
    Item(usize),
}

pub struct List<'a>{
    base:RectangleWithBorder,
    list:RectangleWithBorder,
    list_lines:Line,
    text:TextBase,
    glyphs:&'a Glyphs,
    pressed:ListFocus,
    item_line_height:f32,
    items:Vec<String>,
    chosen_item:Option<usize>,
    list_opened:bool
}

impl<'b> List<'b>{
    pub fn new<'a,T:ToString>(settings:ListSettings<'a,T>,glyphs:&'b Glyphs)->List<'b>{
        let item_len=settings.items.len();
        let line_len=settings.item_line_height*item_len as f32;

        let mut items:Vec<String>=Vec::with_capacity(item_len);
        for i in settings.items.iter(){
            items.push(i.to_string())
        }

        let base=RectangleWithBorder::new(settings.base_rect,settings.base_colour).border(1f32,Black);

        let x1=base.rect.x1;
        let x2=base.rect.x2;
        let y2=base.rect.y2;

        Self{
            base,
            pressed:ListFocus::None,
            list:RectangleWithBorder::raw(
                [
                    x1,
                    y2,
                    x2,
                    y2+line_len
                ],
                settings.list_colour,
                1f32,
                Black
            ),
            list_lines:Line::new([x1,y2,x2,y2],1f32,Black),
            text:TextBase::new(Black,20f32).position([x1,y2-item_margin]),
            glyphs:glyphs,
            item_line_height:settings.item_line_height,
            items,
            chosen_item:settings.chosen_item,
            list_opened:false
        }
    }
    /// Изменение цвета при нажатии
    pub fn press_colour(&mut self){
        self.base.rect.colour[0]-=dcolour;
        self.base.rect.colour[1]-=dcolour;
        self.base.rect.colour[2]-=dcolour;
    }

    /// Изменение цвета при освобождении
    pub fn release_colour(&mut self){
        self.base.rect.colour[0]+=dcolour;
        self.base.rect.colour[1]+=dcolour;
        self.base.rect.colour[2]+=dcolour;
    }

    /// Проверка нажатия на кнопку и локальные действия
    pub fn pressed(&mut self)->ListFocus{
        let [x,y]=unsafe{mouse_cursor.position()};

        self.pressed=if self.base.rect.x1<x && self.base.rect.x2>x && self.base.rect.y1<y && self.base.rect.y2>y{
            self.press_colour();
            ListFocus::Base
        }
        else{
            if self.list_opened{
                if self.list.rect.y1<y && self.list.rect.y2>y{
                    let c=((y-self.base.rect.y2)/self.item_line_height) as usize;
                    ListFocus::Item(c)
                }
                else{
                    ListFocus::None
                }
            }
            else{
                ListFocus::None
            }
        };

        self.pressed.clone()
    }

    pub fn released(&mut self)->ListFocus{
        let [x,y]=unsafe{mouse_cursor.position()};

        match self.pressed{
            ListFocus::Base=>{
                if self.base.rect.x1<x && self.base.rect.x2>x && self.base.rect.y1<y && self.base.rect.y2>y{
                    self.release_colour();
                    self.list_opened=!self.list_opened;
                    ListFocus::Base
                }
                else{
                    ListFocus::None
                }
            }
            ListFocus::Item(item)=>{
                if self.base.rect.y2<y{
                    let c=((y-self.base.rect.y2)/self.item_line_height) as usize;
                    if item==c{
                        self.chosen_item=Some(c);
                        ListFocus::Item(c)
                    }
                    else{
                        ListFocus::None
                    }
                }
                else{
                    ListFocus::None
                }
            }
            ListFocus::None=>ListFocus::None
        }
    }

    pub fn draw_base(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        self.base.draw(draw_parameters,graphics);
        if let Some(item)=self.chosen_item{
            let text=&self.items[item];
            self.text.draw(text,draw_parameters,graphics,self.glyphs).unwrap();
        }
    }

    pub fn draw_list(&mut self,draw_parameters:&mut DrawParameters,graphics:&mut Graphics){
        let position=self.text.position;
        let lines_position=self.list_lines.position();

        self.list.draw(draw_parameters,graphics);

        let dy=self.item_line_height;

        let x=self.list.rect.x1;
        let y=self.list.rect.y1+dy-item_margin;

        self.text.set_position([x,y]);

        for item in &self.items{
            self.text.draw(item,draw_parameters,graphics,self.glyphs);
            self.text.shift_y(dy);
            self.list_lines.shift_y(dy);
            self.list_lines.draw(draw_parameters,graphics);
        }

        self.list_lines.set_position(lines_position);
        self.text.set_position(position)
    }

    pub fn draw(&mut self,draw_parameters:&mut DrawParameters,g:&mut Graphics){
        self.draw_base(draw_parameters,g);

        if self.list_opened{
            self.draw_list(draw_parameters,g);
        }
    }

    pub fn draw_smooth(&mut self,alpha:f32,draw_parameters:&mut DrawParameters,g:&mut Graphics){
        self.base.rect.colour[3]=alpha;
        self.text.colour[3]=alpha;

        self.draw_base(draw_parameters,g);

        if self.list_opened{
            self.list.rect.colour[3]=alpha;
            self.list_lines.colour[3]=alpha;
            self.draw_list(draw_parameters,g);
        }
        
    }
}



pub struct ListSettings<'a,T:ToString>{
    pub base_rect:[f32;4],
    pub base_colour:Colour,
    pub list_colour:Colour,
    pub item_line_height:f32,
    pub items:&'a [T],
    pub chosen_item:Option<usize>,
}

impl<'a,T:ToString> ListSettings<'a,T>{
    /// rect - [x, y, width, height]
    pub fn new(base_rect:[f32;4],items:&'a [T])->ListSettings<'a,T>{
        Self{
            base_rect,
            base_colour:Light_blue,
            list_colour:Light_blue,
            item_line_height:50f32,
            items,
            chosen_item:None,
        }
    }

    pub fn base_colour(mut self,colour:Colour)->ListSettings<'a,T>{
        self.list_colour=colour;
        self
    }

    pub fn chosen_item(mut self,item:usize)->ListSettings<'a,T>{
        self.chosen_item=Some(item);
        self
    }

    pub fn item_line_height(mut self,height:f32)->ListSettings<'a,T>{
        self.item_line_height=height;
        self
    }
}
