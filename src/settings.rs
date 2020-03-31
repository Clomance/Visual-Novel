use crate::*;

const default_window_size:[f64;2]=[1280f64,720f64];

pub struct GameSettings{
    pub window_size:[f64;2],
    pub fullscreen:bool,
    pub characters_len:usize,
    pub pages:usize,
    pub page_wallpapers:usize,
    pub new_game:bool,
}

impl GameSettings{
    //
    pub const fn new()->GameSettings{
        Self{
            window_size:default_window_size,
            fullscreen:false,
            characters_len:0,
            pages:0,
            page_wallpapers:0,
            new_game:true,
        }
    }
    //
    pub fn load(&mut self){
        // Открытие файла и загрузка данных
        let mut settings_file=OpenOptions::new().read(true).open("settings/settings.txt").unwrap();
        let mut settings_str=String::new();
        settings_file.read_to_string(&mut settings_str).unwrap();


        for line in settings_str.lines(){
            let line=line.trim();
            let split_line:Vec<&str>=line.split("=").map(|word|word.trim()).collect();
            // Проверка формата
            if split_line.len()!=2{
                panic!("SettingsFileError");
            }
            // Поиск совпадений
            match split_line[0]{
                "fullscreen"=>{
                    if split_line[1]=="true"{
                        self.fullscreen=true;
                    }
                    else if split_line[1]=="false"{
                        self.fullscreen=false;
                    }   
                    else{
                        panic!("SettingsFileError");
                    }
                }
                "window_width"=>{
                    self.window_size[0]=split_line[1].parse::<f64>().unwrap();
                }
                "window_height"=>{
                    self.window_size[1]=split_line[1].parse::<f64>().unwrap();
                }
                "pages"=>{
                    self.pages=split_line[1].parse::<usize>().unwrap();
                }
                "page_wallpapers"=>{
                    self.page_wallpapers=split_line[1].parse::<usize>().unwrap();
                }
                "characters"=>{
                    self.characters_len=split_line[1].parse::<usize>().unwrap();
                }
                "new_game"=>{
                    if split_line[1]=="true"{
                        self.new_game=true;
                    }
                    else if split_line[1]=="false"{
                        self.new_game=false;
                    }   
                    else{
                        panic!("SettingsFileError");
                    }
                }
                _=>{}
            }
        }
    }
}