use crate::*;

pub struct DialogueFormatted<'a>{
    names:Vec<&'a str>,
    dialogues:Vec<TextLines>,
}

impl<'a> DialogueFormatted<'a>{
    pub fn empty()->DialogueFormatted<'a>{
        Self{
            names:Vec::new(),
            dialogues:Vec::new(),
        }
    }
    pub fn new(names:Vec<&'a str>,dialogues:Vec<TextLines>)->DialogueFormatted<'a>{
        Self{
            names,
            dialogues
        }
    }

    pub fn len(&self)->usize{
        self.names.len()
    }

    pub fn get_line(&self,line:usize)->(&str,&TextLines){
        (&self.names[line],&self.dialogues[line])
    }
}

pub struct Dialogue{
    names_cache:Vec<String>,
    dialogues:Vec<String>,
    names:Vec<usize>
}

impl Dialogue{
    pub fn new<P:AsRef<Path>+Debug+Clone>(path:P)->Dialogue{
        let mut dialogue_file=OpenOptions::new().read(true).open(path.clone()).unwrap();

        let mut names_cache=Vec::<String>::with_capacity(5);
        let mut dialogues=Vec::<String>::with_capacity(50);
        let mut names=Vec::<usize>::with_capacity(50);

        // Для отображения слов игрока
        names_cache.push("Я".to_string());

        // Таблица с краткими именами (short_names,ptr to names_cache)
        let mut names_table=(Vec::new(),Vec::new());

        let mut input=String::with_capacity(512);
        dialogue_file.read_to_string(&mut input).unwrap();

        let mut line_number=0usize;
        let mut lines=input.lines();

        while let Some(line)=lines.next(){
            line_number+=1;
            let line=line.trim();
            
            // Проверка на пустоту
            if line.is_empty(){
                continue
            }
            // Проверка на начало заголовка
            if line=="{"{
                names_table=read_head(&mut names_cache,&mut lines,&mut line_number,path.clone());
                continue
            }
            // Проверка форматирования
            let split_line:Vec<&str>=line.splitn(2,"-").collect();
            let len=split_line.len();
            if len==0{
                //
            }
            if len!=2{
                panic!("LoadingDialogueError: path {:?} line {}",path,line_number);
            }
            // Перевод в строку
            let short_name=split_line[0].trim();
            let dialogue=split_line[1].trim().to_string();

            // Поиск имени
            if short_name=="{}"{
                // Установка имени героя
                names.push(0);
            }
            else{
                // Поиск имени
                let index=search_short_name(short_name,&names_table.0,line_number,path.clone());
                let names_cache_index=names_table.1[index];
                names.push(names_cache_index);
            }
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
    // Получение текущей реплики - (имя,реплика)
    pub fn get_line(&self,step:usize)->(&str,&str){
        (&self.names_cache[self.names[step]],&self.dialogues[step])
    }

    pub fn format<'a>(&'a self,user_name:&str,line_length:f64,font_size:u32,glyphs:&mut GlyphCache)->DialogueFormatted{
        let len=self.names.len();
        let mut names=Vec::with_capacity(len);
        let mut dialogues=Vec::with_capacity(len);

        for c in 0..len{
            let name=self.names_cache[self.names[c]].as_str();
            names.push(name);
            let dialogue=self.dialogues[c].replace("{}",user_name);
            let dialogue=TextLines::new(dialogue,line_length,font_size,glyphs);
            dialogues.push(dialogue);
        }

        DialogueFormatted::new(names,dialogues)
    }
}

fn read_head<P:AsRef<Path>+Debug>(names_cache:&mut Vec<String>,lines:&mut Lines,line_number:&mut usize,path:P)->(Vec<String>,Vec<usize>){
    let mut len=names_cache.len();
    let mut names=Vec::new();
    let mut short_names=Vec::new();
    
    for line in lines{
        //println!("{}",line);
        
        *line_number+=1;
        // Проверка на завершение заголовка
        let line=line.trim();
        if line=="}"{
            return (short_names,names)
        }
        // Проверка формата
        let split_line:Vec<&str>=line.split("=").collect();
        if split_line.len()!=2{
            panic!("DialogueLoadingError: path {:?} line {}",path,line_number);
        }
        // Переавод в строку
        let name=split_line[0].trim().to_string();
        let short_name=split_line[1].trim().to_string();
        // Сохранение
        names_cache.push(name);
        short_names.push(short_name);
        names.push(len);
        len+=1;
    }
    panic!("LoadingDialogueError: no head closure, path {:?}",path);
}

fn search_short_name<P:AsRef<Path>+Debug>(short_name:&str,short_names:&Vec<String>,line:usize,path:P)->usize{
    for (c,name) in short_names.iter().enumerate(){
        if short_name==name{
            return c
        }
    }
    panic!("LoadingDialogueError: No such short name, path {:?} line {}",path,line);
}