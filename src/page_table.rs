use crate::*;

// Таблица распределения ресурсов (картинок, диалогов, персонажей) по страницам
pub struct PageTable<'a,'c>{
    wallpapers:Vec<&'a RgbaImage>,
    dialogues:Vec<&'c Dialogue>,
    characters:Vec<Vec<(&'a RgbaImage,CharacterLocation)>>,
    page:usize
}

impl<'a,'c> PageTable<'a,'c>{
    pub fn new(textures_:&'a Textures,dialogues:&'c Vec<Dialogue>)->PageTable<'a,'c>{
        let mut len=0;
        let cap=10;
        let mut table=Self{
            wallpapers:Vec::with_capacity(cap),
            dialogues:Vec::with_capacity(cap),
            characters:Vec::with_capacity(cap),
            page:unsafe{Settings.saved_page},
        };

        let mut table_file=OpenOptions::new().read(true).open("settings/page_table").unwrap();

        let mut buffer=[0u8;8];

        while let Ok(_)=table_file.read_exact(&mut buffer){
            let wallpaper=usize::from_be_bytes(buffer);

            table_file.read_exact(&mut buffer).unwrap();
            let dialogue=usize::from_be_bytes(buffer);

            table_file.read_exact(&mut buffer[0..1]).unwrap();
            let char_len=buffer[0] as usize;

            let mut characters=Vec::with_capacity(len);
            for _ in 0..char_len{
                table_file.read_exact(&mut buffer).unwrap();
                let character=usize::from_be_bytes(buffer);

                table_file.read_exact(&mut buffer[0..1]).unwrap();
                let location:CharacterLocation=unsafe{std::mem::transmute(buffer[0])};
                characters.push((textures_.character(character),location));
            }
            // Проверка на начало блока страницы
            len+=1;
            table.wallpapers.push(&textures_.wallpaper(wallpaper));
            table.dialogues.push(&dialogues[dialogue]);
            table.characters.push(characters);
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

    pub fn current_character(&self)->&Vec<(&'a RgbaImage,CharacterLocation)>{
        &self.characters[self.page]
    }

    pub fn current_wallpaper(&self)->&'a RgbaImage{
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