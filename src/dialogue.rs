use crate::*;

pub struct DialogueFormatted<'a>{
    names:Vec<&'a str>,
    dialogues:Vec<String>,
}

impl<'a> DialogueFormatted<'a>{
    pub fn empty()->DialogueFormatted<'a>{
        Self{
            names:Vec::new(),
            dialogues:Vec::new(),
        }
    }
    pub fn new(names:Vec<&'a str>,dialogues:Vec<String>)->DialogueFormatted<'a>{
        Self{
            names,
            dialogues
        }
    }

    pub fn len(&self)->usize{
        self.names.len()
    }

    pub fn get_name(&self,step:usize)->&str{
        &self.names[step]
    }

    pub fn get_line(&self,step:usize)->&str{
        &self.dialogues[step]
    }
}

pub struct Dialogue{
    names_cache:Vec<String>,
    dialogues:Vec<String>,
    names:Vec<usize>
}

impl Dialogue{
    pub fn new<P:AsRef<Path>+Debug+Clone>(path:P)->Dialogue{
        let dialogue_file=OpenOptions::new().read(true).open(path.clone()).unwrap();

        let mut dialogues=Vec::<String>::with_capacity(10);
        let mut names=Vec::<usize>::with_capacity(10);

        let mut reader=BufReader::new(dialogue_file);

        // Таблица с краткими именами (short_names,na)
        let (short_names,names_cache)=read_head(&mut reader);

        let mut line=String::new();
        while let Ok(bytes)=reader.read_line(&mut line){
            if bytes==0{
                break
            }
            let line_str=line.trim();
            
            // Проверка на пустоту
            if line_str.is_empty(){
                continue
            }
            // Проверка форматирования
            let split_line:Vec<&str>=line_str.splitn(2,"-").collect();
            let len=split_line.len();

            if len!=2{
                panic!();
            }
            // Перевод в строку
            let short_name=split_line[0].trim();
            let dialogue=split_line[1].trim().to_string();

            // Поиск имени
            let names_cache_index=search_short_name(short_name,&short_names);
            names.push(names_cache_index);

            line.clear(); // Очистка строки
            dialogues.push(dialogue);
        }


        Self{
            names_cache:names_cache,
            dialogues:dialogues,
            names:names
        }
    }

    pub fn len(&self)->usize{
        self.dialogues.len()
    }

    pub fn get_name(&self,step:usize)->&str{
        &self.names_cache[self.names[step]]
    }

    pub fn get_line(&self,step:usize)->&str{
        &self.dialogues[step]
    }

    pub fn format<'a>(&'a self,user_name:&str)->DialogueFormatted{
        let len=self.names.len();
        let mut names=Vec::with_capacity(len);
        let mut dialogues=Vec::with_capacity(len);

        for c in 0..len{
            let name=self.names_cache[self.names[c]].as_str();
            names.push(name);
            let dialogue=self.dialogues[c].replace("{}",user_name);
            dialogues.push(dialogue);
        }

        DialogueFormatted::new(names,dialogues)
    }
}

// Получение имён персонажей диалога
pub fn read_characters(reader:&mut BufReader<File>)->Vec<(String,CharacterLocation)>{
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
        // Проверка на завершение заголовка
        let line_str=line.trim();
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
                "Left"=>CharacterLocation::Left,
                "LeftCenter"=>CharacterLocation::LeftCenter,
                "CenterLeft"=>CharacterLocation::CenterLeft,
                "Center"=>CharacterLocation::Center,
                "CenterRight"=>CharacterLocation::CenterRight,
                "RightCenter"=>CharacterLocation::RightCenter,
                "Right"=>CharacterLocation::Right,
                _=>panic!()
            };
            let name=name[..start].trim().to_string();
            
            names.push((name,location));
        }
        else{
            let location=CharacterLocation::Center;
            let name=split_line[1].split("(").next().unwrap().trim().to_string();
            names.push((name,location));
        };
        // Сохранение
        
        line.clear()
    }

    names
}

// Чтение заголовка (краткие имена, полные имена имён)
// Формат заголовка:
/*
{
    [краткое имя] = [полное имя персонажа].[дополнительные черты]
}
*/
// (имя файла текстуры персонажа - [полное имя персонажа].[дополнительные черты].png)

fn read_head(reader:&mut BufReader<File>)->(Vec<String>,Vec<String>){
    let mut names_cache=Vec::with_capacity(5);
    names_cache.push("Я".to_string()); // Для отображения слов игрока
    names_cache.push("".to_string()); // Для отображения мыслей игрока

    let mut short_names=Vec::with_capacity(5);
    short_names.push("{}".to_string()); // Для отображения слов игрока
    short_names.push("_".to_string()); // Для отображения мыслей игрока


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
            break
        }
        // Проверка на завершение заголовка
        let line_str=line.trim();
        if line_str=="}"{
            return (short_names,names_cache)
        }
        // Проверка формата
        let split_line:Vec<&str>=line_str.split("=").collect();
        if split_line.len()!=2{
            panic!();
        }
        // Перевод в строку
        let short_name=split_line[0].trim().to_string();
        let name=split_line[1].split(".").next().unwrap().split("(").next().unwrap().trim().to_string();
        
        // Сохранение
        short_names.push(short_name);
        names_cache.push(name);
        line.clear()
    }

    (short_names,names_cache)
}

fn search_short_name(short_name:&str,short_names:&Vec<String>)->usize{
    for (c,name) in short_names.iter().enumerate(){
        if short_name==name{
            return c
        }
    }
    panic!("search short name");
}