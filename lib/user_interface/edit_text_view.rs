pub struct EditTextViewSettings<S:Into<String>>{
    text:S,
    capacity:usize,
    font_size:f32,
    text_colour:Colour,
    align:Align,
    rect:[f32;4], // [x1,y1,width,height] - сюда вписывается текст
    background_colour:Colour,
    border_colour:Colour,
}

impl<S:Into<String>> EditTextViewSettings<S>{
    pub fn new(text:S,rect:[f32;4])->EditTextViewSettings<S>{
        Self{
            text,
            capacity:20usize,
            font_size:20f32,
            text_colour:Black,
            align:Align::center(),
            rect,
            background_colour:White,
            border_colour:Black
        }
    }

    pub fn background_colour(mut self,colour:Colour)->EditTextViewSettings<S>{
        self.background_colour=colour;
        self
    }

    pub fn border_colour(mut self,colour:Colour)->EditTextViewSettings<S>{
        self.border_colour=colour;
        self
    }

    pub fn align(mut self,align:Align)->EditTextViewSettings<S>{
        self.align=align;
        self
    }

    pub fn 
}