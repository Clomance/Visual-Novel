#![allow(non_upper_case_globals)]

use std::{
    fs::{
        DirBuilder,
        OpenOptions,
        metadata,
        read_dir,
        File
    },
    io::{Write,BufReader,BufRead,ErrorKind},
};

fn main(){
    match metadata("settings"){
        Ok(_meta)=>{
            #[cfg(any(not(debug_assertions),feature="qrelease"))]
            set_default_settings(false);
            build_resource_file(false);
        }
        Err(e)=>{
            if e.kind()==ErrorKind::NotFound{
                DirBuilder::new().create("./settings").unwrap();
                set_default_settings(true);
                build_resource_file(true);
            }
            else{
                panic!("{:?}",e);
            }
        }
    }
    
}

const paths:&[&str]=&[
    "./resources/images/characters",
    "./resources/images/wallpapers",
    "./resources/dialogues",
    "./resources/page_table.txt"
];



// Получение имён персонажей диалога
pub fn read_characters(reader:&mut BufReader<File>)->Vec<(String,u8)>{
    let mut names=Vec::with_capacity(5);
    let mut line=String::new();

    // Поиск заголовка
    while let Ok(bytes)=reader.read_line(&mut line){
        if bytes==0{
            break
        }
        // Пропуск пустых строк
        let line_str=line.trim();
        if line_str.is_empty(){
            continue
        }
        // Проверка начала заголовка
        if line_str=="{"{
            break
        }
        line.clear()
    }

    line.clear();

    // Чтение заголовка
    while let Ok(bytes)=reader.read_line(&mut line){
        if bytes==0{
            panic!("Ошибка в диалоге: нет конца заголовка");
        }
        let line_str=line.trim();
        // Пропуск пустых строк
        if line_str.is_empty(){
            continue
        }
        // Проверка на завершение заголовка
        if line_str=="}"{
            return names
        }
        // Проверка формата
        let split_line:Vec<&str>=line_str.split("=").collect();
        if split_line.len()!=2{
            panic!("Ошибка в диалоге: неверный формат");
        }
        // Перевод в строку
        let name=split_line[1];
        if let Some(start)=name.find('('){
            let end=name.find(')').unwrap();
            let location=match &name[start+1..end]{
                "Left"=>0u8,
                "LeftCenter"=>1u8,
                "CenterLeft"=>2u8,
                "Center"=>3u8,
                "CenterRight"=>4u8,
                "RightCenter"=>5u8,
                "Right"=>6u8,
                _=>panic!()
            };
            let name=name[..start].trim().to_string();
            
            names.push((name,location));
        }
        else{
            let location=3u8;
            let name=split_line[1].split("(").next().unwrap().trim().to_string();
            names.push((name,location));
        };

        line.clear()
    }

    names
}

// Загрузка именён обоев и диалога одной страницы
// (wallpaper, dialogue)
pub fn load_page_settings(reader:&mut BufReader<File>)->(String,String){
    let mut wallpaper=None;
    let mut dialogue=None;

    let mut line=String::new();
    let mut line_str;

    while let Ok(bytes)=reader.read_line(&mut line){
        line_str=line.trim();
        if line_str=="}" || bytes==0{
            break
        }

        let split_line:Vec<&str>=line.split("=").map(|s|s.trim()).collect();

        // Проверка форматирования
        if split_line.len()!=2{
            panic!("LoadingPageTableError");
        }
        match split_line[0]{
            "wallpaper"=>wallpaper=Some(split_line[1].to_string()),
            "dialogue"=>dialogue=Some(split_line[1].to_string()),
            _=>panic!("LoadingPageTableError: no such field"),
        }

        line.clear();
    }

    (wallpaper.unwrap(),dialogue.unwrap())
}

fn search(vec:&Vec<String>,v:&str)->usize{
    for (c,name) in vec.iter().enumerate(){
        if v==name{
            return c
        }
    }
    panic!("search");
}

// Сброс настроек игры при релизе
fn set_default_settings(create:bool){
    let mut game_settings=OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(create)
            .open("settings/game_settings")
            .unwrap();

    game_settings.write_all(&[0]).unwrap(); // Новая игра

    let buffer=[0u8;8];
    // Текущая страница игры
    game_settings.write_all(&buffer).unwrap();
    // Текущее положение в диалоге на странице
    game_settings.write_all(&buffer).unwrap();
    // Количество символов в секунду
    let mut buffer=0.25f32.to_be_bytes();
    game_settings.write_all(&buffer).unwrap();
    // Значение громкости
    buffer=0.5f32.to_be_bytes();
    game_settings.write_all(&buffer).unwrap();
    // Количество сделанных скриншотов (номер следующего)
    game_settings.write_all(&[0u8;4]).unwrap();
    // Выбранный монитор
    game_settings.write_all(&[0u8;8]).unwrap();
}

// Создание таблицы ресурсов, если требуется
// Создаёт файл с таблицей распределения ресурсов
// Выполняет проверку изменений в файлах и при надобности изменяет таблицу
// Формат файла
// Номер фона, номер диалога, количество персонажей u8, номера персонажей...
fn build_resource_file(create:bool){
    let mut changed=create; // Флаг изменения в папке ресурсов

    let mut page_table_file=OpenOptions::new()
            .write(true)
            .create(create)
            .open("./settings/page_table")
            .unwrap();

    if !changed{ // Поиск изменений по ресурсам
        let flag_last_changed=page_table_file.metadata().unwrap().modified().unwrap();

        for path in paths{
            let meta=metadata(path).unwrap();
            let last_changed=meta.modified().unwrap();
            if last_changed>flag_last_changed{
                changed=true;
                break
            }
        }

        let dia_dir=read_dir("./resources/dialogues").unwrap();
        for dialogue in dia_dir{
            let last_changed=dialogue.unwrap().metadata().unwrap().modified().unwrap();
            if last_changed>flag_last_changed{
                changed=true;
                break
            }
        }
    }

    // Построение таблицы ресурсов
    if changed || cfg!(not(debug_assertions)) || cfg!(feature="qrelease"){
        // Поиск текстур персонажей и сохранение названий файлов
        let char_meta=metadata("./resources/images/characters").unwrap();
        let mut char_names=Vec::with_capacity(char_meta.len() as usize); // Имена персонажей

        // Сохранение имён персонажей
        let char_dir=read_dir("./resources/images/characters").unwrap();
        for character in char_dir{
            let file=character.unwrap();
            let file_name=file.file_name().into_string().unwrap();

            // Удаление .png из имени файла
            let len=file_name.len();
            let name=file_name[..len-4].to_string();

            char_names.push(name);
        }

        // Поиск текстур обоев и сохранение названий файлов
        let wall_meta=metadata("./resources/images/wallpapers/game").unwrap();
        let mut wall_names=Vec::with_capacity(wall_meta.len() as usize); // Названия фонов

        let wall_dir=read_dir("./resources/images/wallpapers/game").unwrap();
        for wall in wall_dir{
            let file=wall.unwrap();
            let file_name=file.file_name().into_string().unwrap();

            // Удаление .png из имени файла
            let len=file_name.len();
            let name=file_name[..len-4].to_string();

            wall_names.push(name);
        }

        // Поиск диалогов и загрузка их заголовков
        let dia_meta=metadata("./resources/dialogues").unwrap();
        let mut dia_names=Vec::with_capacity(dia_meta.len() as usize); // Названия диалогов
        let mut dialogues_characters=Vec::with_capacity(dia_meta.len() as usize); // Имена персонажей в диалогах

        let dia_dir=read_dir("./resources/dialogues").unwrap();
        for dialogue in dia_dir{
            let file=dialogue.unwrap();
            let path=file.path();

            // Получение названия диалога
            let file_name=file.file_name().into_string().unwrap();

            // Удаление .txt из имени файла
            let len=file_name.len();
            let name=file_name[..len-4].to_string();

            dia_names.push(name);

            let dialogue_file=OpenOptions::new().read(true).open(path).unwrap();
            let characters=read_characters(&mut BufReader::new(dialogue_file));

            // Сопоставление имён персонажей в диалогах и текстур персонажей
            let mut dialogue_characters=Vec::new();
            for (ch,location) in characters{
                let index=search(&char_names,&ch);
                dialogue_characters.push((index,location));
            }
            dialogues_characters.push(dialogue_characters);
        }

        let table_file=OpenOptions::new().read(true).open("resources/page_table.txt").unwrap();

        let mut reader=BufReader::new(table_file);
        let mut line=String::new();
        let mut line_str;

        while let Ok(bytes)=reader.read_line(&mut line){
            if bytes==0{
                break // Конец файла
            }

            line_str=line.trim();
            if line_str.is_empty(){
                continue // Пропуск пустой строки
            }

            // Проверка на начало блока страницы
            if let Some(_)=line_str.find("{"){
                let (wallpaper,dialogue)=load_page_settings(&mut reader);

                let wallpaper=search(&wall_names,&wallpaper); // Поиск фона по названию

                page_table_file.write(&wallpaper.to_be_bytes()).unwrap();

                let dialogue=search(&dia_names,&dialogue); // Поиск диалога по названию
                page_table_file.write(&dialogue.to_be_bytes()).unwrap();

                let len=dialogues_characters[dialogue].len() as u8;
                page_table_file.write(&[len]).unwrap();
                for (ch,location) in &dialogues_characters[dialogue]{
                    page_table_file.write(&ch.to_be_bytes()).unwrap();
                    page_table_file.write(&[location.clone() as u8]).unwrap();
                }
            }
            line.clear();
        }
    }
}