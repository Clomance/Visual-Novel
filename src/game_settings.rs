use crate::*;

const default_window_size:Size=Size{
    width:0f64,
    height:0f64
};

pub struct GameSettings{
    pub window_size:Size,
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
            if line.is_empty(){
                continue
            }
            let split_line:Vec<&str>=line.split("=").map(|word|word.trim()).collect();
            // Проверка формата
            if split_line.len()!=2{
                panic!("SettingsFileError");
            }
            // Поиск совпадений
            match split_line[0]{
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
                field=>panic!("SettingsFileError: Unknown field - {}",field),
            }
        }
    }

    pub fn save(&mut self){
        let mut settings_file=OpenOptions::new().write(true)
                .truncate(true).open("settings/settings.txt").unwrap();

        let fields=[
            "\npages = ",
            "\npage_wallpapers = ",
            "\ncharacters = ",
            "\nnew_game = ",
        ];
        let values=[
            self.pages.to_string(),
            self.page_wallpapers.to_string(),
            self.characters_len.to_string(),
            self.new_game.to_string(),
        ];

        for (c,field) in fields.iter().enumerate(){
            settings_file.write_all(field.as_bytes()).unwrap();
            settings_file.write_all(values[c].as_bytes()).unwrap();
        }
    }
}