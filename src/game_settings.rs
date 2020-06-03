use std::{
    io::{Read,Write},
    fs::OpenOptions,
};

pub struct GameSettings{
    pub continue_game:bool, // Флаг продолжения игры
    pub user_name:String,
    pub saved_page:usize, // Страница на которой остановился пользователь (page_table)
    pub saved_dialogue:usize, // Место в диалоге на котором остановился пользователь (dialogue_box)
    pub pages:usize, // Количество страниц в игре
    pub signs_per_frame:f32, // Знаков на кадр
    pub volume:u8, // Громкость игры, 0 - 128
    pub screenshot:u32, // номер следующего скришота

    pub monitor:usize, // Номер монитора в списке мониторов
}

impl GameSettings{
    ///
    pub const fn new()->GameSettings{
        Self{
            continue_game:false,
            user_name:String::new(),
            pages:0,
            saved_page:0,
            saved_dialogue:0,
            signs_per_frame:0.25f32,
            volume:64u8,
            screenshot:0u32,

            monitor:0usize,
        }
    }

    /// Загрузка настроек
    pub fn load(&mut self){
        // Общие настройки пользоавателя
        let mut settings_file=OpenOptions::new().read(true).open("settings/game_settings").unwrap();
        let mut buffer=[0u8;8];

        // Продолжение игры
        settings_file.read_exact(&mut buffer[0..1]).unwrap();
        if buffer[0]!=0{
            self.continue_game=true;
            // Имя пользователя при продолжении игры
            settings_file.read_exact(&mut buffer[0..1]).unwrap();
            
            let mut name=vec![0u8;buffer[0] as usize];
            settings_file.read_exact(&mut name).unwrap();
            self.user_name=String::from_utf8(name).unwrap();
        }
        // Текущая страница игры
        settings_file.read_exact(&mut buffer).unwrap();
        self.saved_page=usize::from_be_bytes(buffer);
        // Текущее положение в диалоге на странице
        settings_file.read_exact(&mut buffer).unwrap();
        self.saved_dialogue=usize::from_be_bytes(buffer);
        //
        let mut buffer=[0u8;4];
        // Количество символов в секунду
        settings_file.read_exact(&mut buffer).unwrap();
        self.signs_per_frame=f32::from_be_bytes(buffer);
        // Значение громкости
        settings_file.read_exact(&mut buffer[0..1]).unwrap();
        self.volume=buffer[0];
        // Количество сделанных скриншотов (номер следующего)
        settings_file.read_exact(&mut buffer).unwrap();
        self.screenshot=u32::from_be_bytes(buffer);

        // Выбранный монитор
        let mut buffer=[0u8;8];
        settings_file.read_exact(&mut buffer).unwrap();
        self.monitor=usize::from_be_bytes(buffer);
    }

    /// Установка позиций для сохранения
    pub fn set_saved_position(&mut self,page:usize,dialogue:usize){
        self.saved_page=page;
        self.saved_dialogue=dialogue;
    }

    /// Сохрание настроек
    pub fn save(&mut self){
        let mut settings_file=OpenOptions::new().write(true).truncate(true).open("settings/game_settings").unwrap();
        if self.continue_game{
            settings_file.write_all(&[1]).unwrap(); // Продолжение игры
            // Имя пользователя при продолжении игры
            let buffer=self.user_name.as_bytes();
            let len=buffer.len() as u8;
            settings_file.write_all(&[len]).unwrap();
            settings_file.write_all(buffer).unwrap();
        }
        else{
            settings_file.write_all(&[0]).unwrap(); // Новая игра
        }
        // Текущая страница игры
        let mut buffer=self.saved_page.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
        // Текущее положение в диалоге на странице
        buffer=self.saved_dialogue.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
        // Количество символов в секунду
        let mut buffer=self.signs_per_frame.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
        // Значение громкости
        settings_file.write_all(&[self.volume]).unwrap();
        // Количество сделанных скриншотов (номер следующего)
        buffer=self.screenshot.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();

        // Выбранный монитор
        let buffer=self.monitor.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
    }
}