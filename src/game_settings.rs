use crate::*;

pub struct GameSettings{
    pub game_name:String,
    pub _continue:bool,
    pub user_name:String,
    pub saved_page:usize, // Страница на которой остановился пользователь (page_table)
    pub saved_dialogue:usize, // Место в диалоге на котором остановился пользователь (dialogue_box)
    pub pages:usize,
    pub page_wallpapers:usize,
    pub characters_len:usize, // Количество персонажей
    pub signs_per_frame:f64, // Знаков на кадр
}

impl GameSettings{
    //
    pub const fn new()->GameSettings{
        Self{
            game_name:String::new(),
            _continue:true,
            user_name:String::new(),
            saved_page:0,
            saved_dialogue:0,
            pages:0,
            page_wallpapers:0,
            characters_len:0,
            signs_per_frame:0.25f64,
        }
    }
    // Загрузка настроек
    pub fn load(&mut self){
        // Открытие файлов и загрузка данных
        let mut settings_file=OpenOptions::new().read(true).open("settings/game_settings").unwrap();
        let mut buffer=[0u8;8];
        settings_file.read_exact(&mut buffer).unwrap();
        self.signs_per_frame=f64::from_be_bytes(buffer);

        settings_file=OpenOptions::new().read(true).open("settings/settings.txt").unwrap();
        let mut settings_str=String::new();

        settings_file.read_to_string(&mut settings_str).unwrap();

        let mut lines=settings_str.lines();

        self.game_name=read_line(&mut lines);

        self._continue=read_line(&mut lines);

        self.user_name=read_line(&mut lines);

        self.saved_page=read_line(&mut lines);

        self.saved_dialogue=read_line(&mut lines);

        self.pages=read_line(&mut lines);

        self.page_wallpapers=read_line(&mut lines);

        self.characters_len=read_line(&mut lines);
    }
    // Установка позиций для сохранения
    pub fn set_saved_position(&mut self,page:usize,dialogue:usize){
        self.saved_page=page;
        self.saved_dialogue=dialogue;
    }
    // Сохрание настроек
    pub fn save(&mut self){
        let mut settings_file=OpenOptions::new().write(true).truncate(true).open("settings/game_settings").unwrap();
        let buffer=self.signs_per_frame.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();

        settings_file=OpenOptions::new().write(true).truncate(true).open("settings/settings.txt").unwrap();

        let values=[
            self.game_name.to_string(),
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
// Перевод строки в нужный тип
pub fn read_line<T:FromStr>(lines:&mut Lines)->T{
    match lines.next().unwrap().parse::<T>(){
        Ok(result)=>result,
        Err(_)=>panic!("LoadingGameSettingsError")
    }
}