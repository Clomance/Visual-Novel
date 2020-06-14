use std::path::Path;
use std::fs::File;
use std::thread::JoinHandle;

use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use rodio::{
    Source,
    Sink,
    Decoder,
    source::Buffered,
};

enum AudioCommand{
    Add(Buffered<Decoder<File>>),
    SetVolume(f32),
    Clear,
    Close,
}

unsafe impl std::marker::Sync for AudioCommand{}
unsafe impl std::marker::Send for AudioCommand{}

pub struct Audio{
    command:Sender<AudioCommand>,
    thread:Option<JoinHandle<()>>,
    tracks:Vec<Buffered<Decoder<File>>>
}

impl Audio{
    pub fn new()->Audio{
        let (sender,receiver)=channel();
        let thread=std::thread::spawn(move||{
            let device=rodio::default_output_device().unwrap();
            let sink=Sink::new(&device);
            loop{
                match receiver.recv(){
                    Ok(command)=>{
                        match command{
                            AudioCommand::Add(track)=>{
                                sink.append(track.repeat_infinite())
                            }
                            AudioCommand::SetVolume(volume)=>{
                                sink.set_volume(volume)
                            }
                            AudioCommand::Clear=>{
                                sink.stop()
                            }
                            AudioCommand::Close=>{
                                break
                            }
                        }
                    }
                    Err(_)=>break
                }
            }
        });

        Self{
            command:sender,
            thread:Some(thread),
            tracks:Vec::new(),
        }
    }

    pub fn set_volume(&self,volume:f32){
        self.command.send(AudioCommand::SetVolume(volume));
    }

    pub fn add_track<P:AsRef<Path>>(&mut self,path:P){
        let file=File::open(path).unwrap();
        let track=rodio::Decoder::new(file).unwrap().buffered();
        self.tracks.push(track)
    }

    pub fn play_track(&self,index:usize){
        self.command.send(AudioCommand::Add(self.tracks[index].clone()));
    }
}

impl Drop for Audio{
    fn drop(&mut self){
        self.command.send(AudioCommand::Close);
        if let Some(thread)=self.thread.take(){
            thread.join();
        }
    }
}