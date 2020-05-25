use std::path::Path;

pub struct Music;

impl Music{
    pub fn new()->Music{
        Self
    }

    pub fn add_music<P:AsRef<Path>>(&mut self,_path:P){}

    pub fn start_music(&self,_index:usize){}

    pub fn set_volume(&self,_volume:u8){}
}