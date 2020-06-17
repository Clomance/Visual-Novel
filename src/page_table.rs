use crate::{CharacterLocation,Dialogue,Settings,Textures};

use cat_engine::image::image::RgbaImage;

use std::{
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
};

/// Таблица распределения ресурсов (картинок, диалогов, персонажей) по страницам.
pub struct PageTable<'a,'c>{
    wallpapers:Vec<&'a PathBuf>,
    dialogues:Vec<&'c Dialogue>,
    characters:Vec<Vec<(&'a RgbaImage,CharacterLocation)>>,
    page:usize
}

impl<'a,'c> PageTable<'a,'c>{
    pub fn new(textures:&'a Textures,dialogues:&'c Vec<Dialogue>)->PageTable<'a,'c>{
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
                characters.push((textures.character(character),location));
            }

            len+=1;
            table.wallpapers.push(&textures.wallpaper(wallpaper));
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

    pub fn current_wallpaper(&self)->&'a PathBuf{
        &self.wallpapers[self.page]
    }

    pub fn current_dialogue(&self)->&'c Dialogue{
        &self.dialogues[self.page]
    }
}