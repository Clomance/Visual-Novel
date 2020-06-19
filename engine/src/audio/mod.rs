mod audio_track;
mod sample;
mod sample_rate;
mod channels;

pub use audio_track::*;
use sample_rate::*;

use cpal::{
    Device,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    UnknownTypeOutputBuffer,
    StreamData,
    StreamId,
    EventLoop,
};

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::LockResult;
use std::thread::JoinHandle;

#[derive(Debug,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    ThreadClosed,
    TrackError,
}

impl AudioCommandResult{
    pub fn unwrap(self){
        if self!=AudioCommandResult::Ok{
            panic!("{:?}",self)
        }
    }

    pub fn expect(self,msg:&str){
        if self!=AudioCommandResult::Ok{
            panic!("{} {:?}",msg,self)
        }
    }
}

enum AudioSystemCommand{
    AddTrack(Track<i16>),
    PlayOnce(usize),
    PlayForever(usize),
    SetVolume(f32),
    Close,
}

enum Play{
    None,
    Once(SampleRateConverter<std::vec::IntoIter<i16>>),
    Forever(SampleRateConverter<std::iter::Cycle<std::vec::IntoIter<i16>>>),
}

unsafe impl std::marker::Sync for AudioSystemCommand{}
unsafe impl std::marker::Send for AudioSystemCommand{}

/// Простой аудио движок.
/// Simple audio engine.
/// 
pub struct Audio{
    event_loop:Arc<EventLoop>,
    streams:Arc<Mutex<Vec<StreamId>>>,
    command:std::sync::mpsc::Sender<AudioSystemCommand>,
    thread:Option<JoinHandle<()>>,
}

impl Audio{
    pub fn new(settings:AudioSettings)->Audio{
        let mut volume=0.5f32;
        let mut tracks:Vec<Track<i16>>=Vec::with_capacity(settings.track_buffer_capacity);
        let channels=Arc::new(Mutex::new(Vec::with_capacity(settings.channels)));

        let c=channels.clone();

        let host=cpal::default_host();
        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Передача команд от управляющего потока выполняющему
        let (sender,receiver)=std::sync::mpsc::channel::<AudioSystemCommand>();

        let thread=std::thread::spawn(move||{

            let mut play=Play::None;

            let device=host.default_output_device().unwrap();
            let format=device.default_output_format().unwrap();

            let device_sample_rate=format.sample_rate;
            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            c.lock().unwrap().push(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();
            event_loop.clone().run(move|stream,result|{
                match receiver.try_recv(){
                    Ok(command)=>match command{
                        AudioSystemCommand::AddTrack(new_track)=>{
                            if tracks.len()<tracks.capacity(){
                                tracks.push(new_track)
                            }
                        }
                        AudioSystemCommand::PlayOnce(i)=>{
                            play=Play::Once(tracks[i].clone().into_iter(device_sample_rate));
                        }
                        AudioSystemCommand::PlayForever(i)=>{
                            play=Play::Forever(tracks[i].clone().endless_iter(device_sample_rate));
                        }
                        AudioSystemCommand::SetVolume(v)=>{
                            volume=v;
                        }
                        AudioSystemCommand::Close=>{
                            panic!("Closing audio thread")
                        },
                    }
                    Err(_)=>{}
                }

                match result{
                    Ok(data)=>{
                        match data{
                            StreamData::Output{buffer:UnknownTypeOutputBuffer::I16(mut buffer)}=>{
                                match &mut play{
                                    Play::None=>{}
                                    Play::Once(track)=>{
                                        for b in buffer.iter_mut(){
                                            *b=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                        }
                                    }
                                    Play::Forever(track)=>{
                                        for b in buffer.iter_mut(){
                                            *b=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                        }
                                    }
                                }
                            }

                            StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(mut buffer)}=>{
                                match &mut play{
                                    Play::None=>{}
                                    Play::Once(track)=>{
                                        for b in buffer.iter_mut(){
                                            let sample=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                            *b=(i16::max_value()+sample) as u16;
                                        }
                                    }
                                    Play::Forever(track)=>{
                                        for b in buffer.iter_mut(){
                                            let sample=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                            *b=(i16::max_value()+sample) as u16;
                                        }
                                    }
                                }
                            }
                            StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(mut buffer)}=>{
                                match &mut play{
                                    Play::None=>{}
                                    Play::Once(track)=>{
                                        for b in buffer.iter_mut(){
                                            let sample=track.next().unwrap_or(0i16) as f32 * volume;
                                            *b=sample/(i16::max_value() as f32);
                                        }
                                    }
                                    Play::Forever(track)=>{
                                        for b in buffer.iter_mut(){
                                            let sample=track.next().unwrap_or(0i16) as f32 * volume;
                                            *b=sample/(i16::max_value() as f32);
                                        }
                                    }
                                }
                            }
                            _=>{}
                        }
                    }
                    Err(e)=>{eprintln!("an error occurred on stream {:?}: {}",stream,e);return}
                }

                
            });
        });

        Self{
            event_loop:el,
            streams:channels,
            command:sender,
            thread:Some(thread),
        }
    }

    pub fn default_output_device()->Option<Device>{
        cpal::default_host().default_output_device()
    }

    /// Добавляет трек в массив треков, удаляет, если массив переполнен
    pub fn add_track<P:AsRef<Path>>(&mut self,path:P)->AudioCommandResult{
        let track=match Track::new(path){
            TrackResult::Ok(track)=>track,
            _=>return AudioCommandResult::TrackError
        };
        match self.command.send(AudioSystemCommand::AddTrack(track)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    pub fn play_once(&mut self,index:usize)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PlayOnce(index)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    pub fn play_forever(&mut self,index:usize)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::PlayForever(index)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    pub fn pause(&mut self)->AudioCommandResult{
        let stream=match self.streams.lock(){
            LockResult::Ok(streams)=>streams.get(0).unwrap().clone(),
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };
        self.event_loop.pause_stream(stream);
        AudioCommandResult::Ok
    }

    pub fn play(&mut self)->AudioCommandResult{
        let stream=match self.streams.lock(){
            LockResult::Ok(streams)=>streams.get(0).unwrap().clone(),
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };
        self.event_loop.play_stream(stream);
        AudioCommandResult::Ok
    }

    pub fn set_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetVolume(volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }
}

impl Drop for Audio{
    fn drop(&mut self){
        let _=self.command.send(AudioSystemCommand::Close);
        if let Some(thread)=self.thread.take(){
            let _=thread.join();
        }
        println!("Dropped");
    }
}

pub struct AudioSettings{
    pub channels:usize,
    pub track_buffer_capacity:usize,
}

impl AudioSettings{
    pub fn new()->AudioSettings{
        Self{
            channels:1,
            track_buffer_capacity:1,
        }
    }
}