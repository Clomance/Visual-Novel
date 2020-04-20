use crate::*;

// Таблица распределения ресурсов (картинок, диалогов, персонажей) по страницам
pub struct PageTable<'a,'b,'c>{
    wallpapers:Vec<&'b RgbaImage>,
    dialogues:Vec<&'c Dialogue>,
    characters:Vec<&'a RgbaImage>,
    page:usize
}

impl<'a,'b,'c> PageTable<'a,'b,'c>{
    pub fn new(characters:&'a Vec<RgbaImage>,wallpapers:&'b Vec<RgbaImage>,dialogues:&'c Vec<Dialogue>,saved_page:usize)->PageTable<'a,'b,'c>{
        let mut len=0;
        let cap=10;
        let mut table=Self{
            wallpapers:Vec::with_capacity(cap),
            dialogues:Vec::with_capacity(cap),
            characters:Vec::with_capacity(cap),
            page:saved_page,
        };

        let table_file=OpenOptions::new().read(true).open("settings/page_table.txt").unwrap();

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
                len+=1;
                let (wallpaper,dialogue,character)=load_page_settings(&mut reader);
                table.wallpapers.push(&wallpapers[wallpaper]);
                table.dialogues.push(&dialogues[dialogue]);
                table.characters.push(&characters[character]);
            }
            line.clear();
        }

        unsafe{
            Settings.pages=len;
        }

        table
    }

    pub fn current_page(&self)->usize{
        self.page
    }

    pub fn next_page(&mut self)->bool{
        if self.page+1<self.wallpapers.len(){
            self.page+=1;
            true
        }
        else{
            false
        }
    }

    pub fn current_character(&self)->&'a RgbaImage{
        &self.characters[self.page]
    }

    pub fn current_wallpaper(&self)->&'b RgbaImage{
        &self.wallpapers[self.page]
    }

    pub fn current_dialogue(&self)->&'c Dialogue{
        &self.dialogues[self.page]
    }
}

// (wallpaper, dialogue, character)
fn load_page_settings(reader:&mut BufReader<File>)->(usize,usize,usize){
    // Проверка трёх полей
    let mut wallpaper=None;
    let mut dialogue=None;
    let mut character=None;

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
        // Проверка номера
        let index=match split_line[1].parse::<usize>(){
            Ok(num)=>num,
            Err(_)=>panic!("LoadingPageTableError: not a number"),
        };
        match split_line[0]{
            "wallpaper"=>wallpaper=Some(index),
            "dialogue"=>dialogue=Some(index),
            "character"=>character=Some(index),
            _=>panic!("LoadingPageTableError: no such field"),
        }

        line.clear();
    }

    (wallpaper.unwrap(),dialogue.unwrap(),character.unwrap())
}