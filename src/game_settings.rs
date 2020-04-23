use crate::*;

pub struct GameSettings{
    pub game_name:String,
    pub _continue:bool, // Флаг продолжения игры
    pub user_name:String,
    pub saved_page:usize, // Страница на которой остановился пользователь (page_table)
    pub saved_dialogue:usize, // Место в диалоге на котором остановился пользователь (dialogue_box)
    pub pages:usize,
    pub signs_per_frame:f64, // Знаков на кадр
    pub volume:f64, // Громкость игры
}

impl GameSettings{
    //
    pub const fn new()->GameSettings{
        Self{
            game_name:String::new(),
            _continue:true,
            user_name:String::new(),
            pages:0,
            saved_page:0,
            saved_dialogue:0,
            signs_per_frame:0.25f64,
            volume:0.5f64,
        }
    }
    // Загрузка настроек
    pub fn load(&mut self){
        // Общие настройки пользоавателя
        let mut settings_file=OpenOptions::new().read(true).open("settings/game_settings").unwrap();
        let mut buffer=[0u8;8];

        settings_file.read_exact(&mut buffer).unwrap();
        self.saved_page=usize::from_be_bytes(buffer);
        //
        settings_file.read_exact(&mut buffer).unwrap();
        self.saved_dialogue=usize::from_be_bytes(buffer);
        //
        settings_file.read_exact(&mut buffer).unwrap();
        self.signs_per_frame=f64::from_be_bytes(buffer);
        //
        settings_file.read_exact(&mut buffer).unwrap();
        self.volume=f64::from_be_bytes(buffer);


        // Редактируемый файл настроек
        settings_file=OpenOptions::new().read(true).open("settings/settings.txt").unwrap();
        let mut reader=BufReader::new(settings_file);

        self.game_name=read_line(&mut reader);

        self._continue=read_line(&mut reader);

        self.user_name=read_line(&mut reader);
    }
    // Установка позиций для сохранения
    pub fn set_saved_position(&mut self,page:usize,dialogue:usize){
        self.saved_page=page;
        self.saved_dialogue=dialogue;
    }
    // Сохрание настроек
    pub fn save(&mut self){
        let mut settings_file=OpenOptions::new().write(true).truncate(true).open("settings/game_settings").unwrap();

        let mut buffer=self.saved_page.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
        //
        buffer=self.saved_dialogue.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
        //
        buffer=self.signs_per_frame.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();
        //
        buffer=self.volume.to_be_bytes();
        settings_file.write_all(&buffer).unwrap();

        settings_file=OpenOptions::new().write(true).truncate(true).open("settings/settings.txt").unwrap();

        let values=[
            self.game_name.to_string(),
            self._continue.to_string(),
            self.user_name.clone(),
        ];

        for value in &values{
            settings_file.write_all(value.as_bytes()).unwrap();
            settings_file.write_all(b"\n").unwrap();
        }
    }
}
// Перевод строки в нужный тип
pub fn read_line<T:FromStr>(reader:&mut BufReader<File>)->T{
    let mut line=String::new();
    let _r=reader.read_line(&mut line);
    match line.trim().parse::<T>(){
        Ok(t)=>t,
        Err(_)=>panic!()
    }
}