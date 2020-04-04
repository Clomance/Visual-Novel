use crate::*;

pub struct GameSettings{
    pub _continue:bool,
    pub user_name:String,
    pub saved_page:usize, // Страница на которой остановился пользователь (page_table)
    pub saved_dialogue:usize, // Место в диалоге на котором остановился пользователь (dialogue_box)
    pub pages:usize,
    pub page_wallpapers:usize,
    pub characters_len:usize, 
}

impl GameSettings{
    //
    pub const fn new()->GameSettings{
        Self{
            _continue:true,
            user_name:String::new(),
            saved_page:0,
            saved_dialogue:0,
            pages:0,
            page_wallpapers:0,
            characters_len:0,
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

        self.saved_page=lines.next().unwrap().parse::<usize>().unwrap();

        self.saved_dialogue=lines.next().unwrap().parse::<usize>().unwrap();

        self.pages=lines.next().unwrap().parse::<usize>().unwrap();

        self.page_wallpapers=lines.next().unwrap().parse::<usize>().unwrap();

        self.characters_len=lines.next().unwrap().parse::<usize>().unwrap();
    }

    // Установка позиций для сохранения
    pub fn set_saved_position(&mut self,page:usize,dialogue:usize){
        self.saved_page=page;
        self.saved_dialogue=dialogue;
    }

    //
    pub fn save(&mut self){
        let mut settings_file=OpenOptions::new().write(true).truncate(true).open("settings/settings.txt").unwrap();

        let values=[
            self._continue.to_string(),
            self.user_name.clone(),
            self.saved_page.to_string(),
            self.saved_dialogue.to_string(),
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