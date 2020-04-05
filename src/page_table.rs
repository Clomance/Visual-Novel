use crate::*;

// Таблица распределения ресурсов (картинок, диалогов, персонажей) по страницам
pub struct PageTable<'a,'b,'c>{
    wallpapers:Vec<&'b RgbaImage>,
    dialogues:Vec<&'c Dialogue>,
    characters:Vec<&'a Character>,
    page:usize
}

impl<'a,'b,'c> PageTable<'a,'b,'c>{
    pub fn new(characters:&'a Vec<Character>,wallpapers:&'b Vec<RgbaImage>,dialogues:&'c Vec<Dialogue>)->PageTable<'a,'b,'c>{
        let mut len=unsafe{Settings.pages};
        let mut table=Self{
            wallpapers:Vec::with_capacity(len),
            dialogues:Vec::with_capacity(len),
            characters:Vec::with_capacity(len),
            page:unsafe{Settings.saved_page},
        };

        let mut table_file=OpenOptions::new().read(true).open("settings/page_table.txt").unwrap();

        let mut table_str=String::with_capacity(len*45);
        table_file.read_to_string(&mut table_str).unwrap();


        let mut lines=table_str.lines(); // Разделение на строки


        while let Some(line)=lines.next(){
            let line=line.trim();
            // Проверка на пустоту строки
            if line.is_empty(){
                continue
            }
            // Проверка на начало блока страницы
            if let Some(_)=line.find("{"){
                len-=1;
                let (wallpaper,dialogue,character)=load_page_settings(&mut lines);
                table.wallpapers.push(&wallpapers[wallpaper]);
                table.dialogues.push(&dialogues[dialogue]);
                table.characters.push(&characters[character]);
            }
        }

        if len!=0{
            panic!("LoadingPageTableError: Not enough pages")
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

    pub fn currents_character(&self)->&'a Character{
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
fn load_page_settings(lines:&mut Lines)->(usize,usize,usize){
    // Проверка трёх полей
    let mut wallpaper=None;
    let mut dialogue=None;
    let mut character=None;

    for line in lines.take(3){

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
    }
    let line=lines.next().unwrap().trim();
    // Проверка на завершение
    if line!="}"{
        panic!("LoadingPageTableError: no end of page block")
    }
    (wallpaper.unwrap(),dialogue.unwrap(),character.unwrap())
}