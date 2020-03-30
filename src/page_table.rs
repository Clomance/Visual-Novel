use crate::*;

pub struct PageTable<'b,'c>{
    //characters_cache:&'a Vec<Character>, // Кэш персонажей
    wallpapers:&'b Vec<Texture>,
    dialogues:&'c Vec<Dialogue>,
    //characters_table:Vec<Vec<usize>>, // Распеделение персонажей из кэша по страницам
    page:usize
}

impl<'b,'c> PageTable<'b,'c>{
    pub fn new(//characters:Vec<Character>,characters_table:Vec<Vec<usize>>,
                wallpapers:&'b Vec<Texture>,dialogues:&'c Vec<Dialogue>)->PageTable<'b,'c>{
        Self{
            //characters_cache:characters,
            wallpapers:wallpapers,
            dialogues:dialogues,
            //characters_table:characters_table,
            page:0
        }
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

    pub fn current_wallpaper(&self)->&'b Texture{
        &self.wallpapers[self.page]
    }

    pub fn current_dialogue(&self)->&'c Dialogue{
        &self.dialogues[self.page]
    }
}