use sdl2::{
    Sdl,
    AudioSubsystem,
    mixer::{self,Music as SdlMusic},
};

use std::path::Path;

pub struct Music{
    context:Sdl,
    audio:AudioSubsystem,
    music:Vec<SdlMusic<'static>>,
}

impl Music{
    pub fn new()->Music{
        let _=mixer::init(mixer::InitFlag::MP3);

        mixer::open_audio(
                mixer::DEFAULT_FREQUENCY,
                mixer::DEFAULT_FORMAT,
                mixer::DEFAULT_CHANNELS,
                1024
            )
            .unwrap();

        mixer::allocate_channels(2);

        let sdl=sdl2::init().unwrap();

        Self{
            audio:sdl.audio().unwrap(),
            context:sdl,
            music:Vec::new(),
        }
    }

    pub fn add_music<P:AsRef<Path>>(&mut self,path:P){
        let music=SdlMusic::from_file(path).unwrap();
        self.music.push(music);
    }

    pub fn start_music(&self,index:usize){
        self.music[index].play(-1);
    }

    // 0 - 1.0
    pub fn set_volume(&self,volume:f32){
        SdlMusic::set_volume(volume as i32) // 0 - 128
    }
}

