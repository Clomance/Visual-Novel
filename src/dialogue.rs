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

        let mut names_cache=Vec::<String>::with_capacity(5);
        let mut dialogues=Vec::<String>::with_capacity(50);
        let mut names=Vec::<usize>::with_capacity(50);

        names_cache.push("Я".to_string()); // Для отображения слов игрока

        names_cache.push("".to_string()); // Для отображения мыслей игрока

        let mut reader=BufReader::new(dialogue_file);

        // Таблица с краткими именами (short_names,ptr at names_cache)
        let names_table=read_head(&mut names_cache,&mut reader);

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
            if short_name=="{}"{
                names.push(0); // Установка имени героя
            }
            else if short_name=="_"{
                names.push(1); // Установка пустого имени
            }
            else{
                // Поиск имени
                let index=search_short_name(short_name,&names_table.0);
                let names_cache_index=names_table.1[index];
                names.push(names_cache_index);
            }
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

// (краткие имена, индексы имён)
fn read_head(names_cache:&mut Vec<String>,reader:&mut BufReader<File>)->(Vec<String>,Vec<usize>){
    let mut len=names_cache.len();
    let mut names=Vec::new();
    let mut short_names=Vec::new();

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
            return (short_names,names)
        }
        // Проверка формата
        let split_line:Vec<&str>=line_str.split("=").collect();
        if split_line.len()!=2{
            panic!();
        }
        // Перевод в строку
        let name=split_line[0].trim().to_string();
        let short_name=split_line[1].trim().to_string();
        // Сохранение
        names_cache.push(name);
        short_names.push(short_name);
        names.push(len);
        len+=1;
        line.clear()
    }

    (short_names,names)
}

fn search_short_name(short_name:&str,short_names:&Vec<String>)->usize{
    for (c,name) in short_names.iter().enumerate(){
        if short_name==name{
            return c
        }
    }
    panic!("search short name");
}