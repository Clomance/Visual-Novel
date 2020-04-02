use crate::*;

pub struct GameSettings{
    pub _continue:bool,
    pub user_name:String,
    pub pages:usize,
    pub page_wallpapers:usize,
    pub characters_len:usize,
    pub saved_page:usize,
    pub saved_dialog:usize,
}

impl GameSettings{
    //
    pub const fn new()->GameSettings{
        Self{
            characters_len:0,
            pages:0,
            page_wallpapers:0,
            _continue:true,
            user_name:String::new(),
            saved_page:0,
            saved_dialog:0,
        }
    }
    //
    pub fn load(&mut self){
        // Открытие файла и загрузка данных
        let mut settings_file=OpenOptions::new().read(true).open("settings/settings.txt").unwrap();
        let mut settings_str=String::new();

        settings_file.read_to_string(&mut settings_str).unwrap();

        let mut lines=settings_str.lines();

        self._continue=lines.next().unwrap().parse::<bool>().unwrap();

        self.user_name=lines.next().unwrap().to_string();

        self.pages=lines.next().unwrap().parse::<usize>().unwrap();

        self.page_wallpapers=lines.next().unwrap().parse::<usize>().unwrap();

        self.characters_len=lines.next().unwrap().parse::<usize>().unwrap();
    }

    pub fn save(&mut self){
        let mut settings_file=OpenOptions::new().write(true).truncate(true).open("settings/settings.txt").unwrap();

        let values=[
            self._continue.to_string(),
            self.user_name.clone(),
            self.pages.to_string(),
            self.page_wallpapers.to_string(),
            self.characters_len.to_string(),
        ];

        for value in &values{
            settings_file.write_all(value.as_bytes()).unwrap();
            settings_file.write_all(b"\n").unwrap();
        }
    }
}