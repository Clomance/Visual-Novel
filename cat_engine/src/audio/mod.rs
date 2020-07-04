//! # Простая аудио система. Simple audio system. `feature = "audio"`.
//! 
//! Аудио система имеет свой поток для работы со звуком.
//! Он контролируется через канал `std::sync::mpsc::channel()`.
//! Также в нём есть массив аудио треков, которые можно запустить.
//! 
//! Пока поддерживает только один канал для проигрывания треков
//! и только формат `mp3`.
//! 
//! 
//! Закрывается поток с паникой, так что не паникуте!
//! 
//! Некоторый код был взят из [rodio](https://github.com/RustAudio/rodio).
//! 
//! #
//! 
//! The audio system has it's own thread for handling the sound.
//! It's controled with channel `std::sync::mpsc::channel()`.
//! Also it has audio track array.
//! 
//! The system supports only one channel for playing tracks
//! and only `mp3` format.
//! 
//! 
//! The thread closes with panic, so don't panic!
//! 
//! Some code was taken from [rodio](https://github.com/RustAudio/rodio).
//! 
//! #
//! 
//! ```
//! let settings=AudioSettings::new();
//! let audio=Audio::new(settings).unwrap();
//! 
//! audio.add_track("audio.mp3"); // track index = 0
//! 
//! audio.play_once(0); // plays the track with index 0
//! ```

mod audio_track;
mod sample;
mod sample_rate;
mod channels;

use audio_track::*;
use sample_rate::*;

use channels::ChannelCountConverter;

use cpal::{
    Host,
    HostId,
    HostUnavailable,
    Device,
    DevicesError,
    Devices,
    OutputDevices,
    traits::{
        HostTrait,
        DeviceTrait,
        EventLoopTrait
    },
    UnknownTypeOutputBuffer,
    StreamData,
    StreamId,
    EventLoop,
    Sample,
    Format,
};

use std::{
    io,
    vec::IntoIter,
    iter::Cycle,
    path::Path,
    thread::{Builder,JoinHandle},
    sync::{
        Arc,
        Mutex,
        LockResult,
        mpsc::{Sender,Receiver,channel},
    },
};

/// Результат выполнения команды. The result of command accomplishing.
#[derive(Clone,Debug,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    NoSuchTrack,
    NoStream,
    ThreadClosed,
    TrackError,
}

impl AudioCommandResult{
    /// Паникует, если результат не `Ok`.
    /// 
    /// Panics if the result isn't `Ok`.
    pub fn unwrap(self){
        if self!=AudioCommandResult::Ok{
            panic!("{:?}",self)
        }
    }

    /// Паникует и выводит сообщение, если результат не `Ok`.
    /// 
    /// Panics и prints the message if the result isn't `Ok`.
    pub fn expect(self,msg:&str){
        if self!=AudioCommandResult::Ok{
            panic!("{} {:?}",msg,self)
        }
    }
}

enum AudioSystemCommand{
    PlayOnce(usize),
    PlayForever(usize),
    Stop,

    SetVolume(f32),
    Close,
}

enum Play{
    None,
    Once(ChannelCountConverter<SampleRateConverter<IntoIter<i16>>>),
    Forever(ChannelCountConverter<SampleRateConverter<Cycle<IntoIter<i16>>>>),
}

unsafe impl std::marker::Sync for AudioSystemCommand{}
unsafe impl std::marker::Send for AudioSystemCommand{}

const audio_thread_stack_size:usize=1024;

/// Простой аудио движок.
/// Simple audio engine.
/// 
/// Пока только вывод доступен.
/// 
/// Only output is available now.
/// 
pub struct Audio{
    tracks:Arc<Mutex<Vec<Track<i16>>>>,
    event_loop:Arc<EventLoop>,
    stream:Arc<Mutex<Option<StreamId>>>,
    command:Sender<AudioSystemCommand>,
    thread:Option<JoinHandle<()>>,
}

impl Audio{
    /// For default host and device.
    /// 
    /// Returns the result of starting an audio thread.
    pub fn new(settings:AudioSettings)->io::Result<Audio>{
        // Массив треков
        let tracks=Arc::new(Mutex::new(Vec::with_capacity(settings.track_array_capacity)));
        let stream=Arc::new(Mutex::new(None));

        let t=tracks.clone();
        let s=stream.clone();

        let host=cpal::default_host();
        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Передача команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioSystemCommand>();

        let thread_result=Builder::new()
                .name("Audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{
            let play=Play::None;

            let device=host.default_output_device().unwrap();
            let mut format=device.default_output_format().unwrap();

            format.channels=settings.output_type.into_channels();

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            Audio::event_loop_handler(
                event_loop,
                format,
                receiver,
                play,
                tracks,
                settings.volume,
            );
        });

        let thread=match thread_result{
            Ok(thread)=>thread,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            tracks:t,
            event_loop:el,
            stream:s,
            command:sender,
            thread:Some(thread),
        })
    }

    /// For given host and device.
    /// 
    /// Returns the result of starting the audio thread.
    pub fn with_host_and_device(settings:AudioSettings,host:Host,device:Device)->io::Result<Audio>{
        // Массив треков
        let tracks=Arc::new(Mutex::new(Vec::with_capacity(settings.track_array_capacity)));
        let stream=Arc::new(Mutex::new(None));

        let t=tracks.clone();
        let s=stream.clone();

        let event_loop=Arc::new(host.event_loop());
        let el=event_loop.clone();
        // Передача команд от управляющего потока выполняющему
        let (sender,receiver)=channel::<AudioSystemCommand>();

        let thread_result=Builder::new()
                .name("Audio thread".to_string())
                .stack_size(audio_thread_stack_size)
                .spawn(move||{
            let play=Play::None;

            let mut format=device.default_output_format().unwrap();

            format.channels=settings.output_type.into_channels();

            let main_stream=event_loop.build_output_stream(&device,&format).expect("stream");

            *stream.lock().unwrap()=Some(main_stream.clone());

            event_loop.play_stream(main_stream.clone()).unwrap();

            Audio::event_loop_handler(
                event_loop,
                format,
                receiver,
                play,
                tracks,
                settings.volume,
            );
        });

        let thread=match thread_result{
            Ok(thread)=>thread,
            Err(e)=>return Err(e),
        };

        Ok(Self{
            tracks:t,
            event_loop:el,
            stream:s,
            command:sender,
            thread:Some(thread),
        })
    }

    pub fn available_hosts()->Vec<HostId>{
        cpal::available_hosts()
    }

    pub fn host_from_id(id:HostId)->Result<Host,HostUnavailable>{
        cpal::host_from_id(id)
    }

    /// Может вызвать панику, если окно запущено в том же потоке.
    /// 
    /// This function may panic if the window is running in the same thread.
    pub fn default_output_device()->Option<Device>{
        cpal::default_host().default_output_device()
    }

    /// Возвращает все доступные устройства текущего хоста.
    /// 
    /// Может вызвать панику, если окно запущено в том же потоке.
    /// 
    /// Returns all available devices of the default host.
    /// 
    /// This function may panic if the window is running in the same thread.
    pub fn output_devices()->Result<OutputDevices<Devices>,DevicesError>{
        cpal::default_host().output_devices()
    }

    /// Добавляет трек в массив треков.
    /// 
    /// Adds the track to the track array.
    pub fn add_track<P:AsRef<Path>>(&self,path:P)->AudioCommandResult{
        let track=match Track::new(path){
            TrackResult::Ok(track)=>track,
            _=>return AudioCommandResult::TrackError
        };

        match self.tracks.lock(){
            Ok(mut lock)=>{
                lock.push(track);
                AudioCommandResult::Ok
            },
            Err(_)=>AudioCommandResult::ThreadClosed,
        }
    }

    /// Удаляет трек из массива треков.
    /// 
    /// Removes the track from the track array.
    pub fn remove_track(&self,index:usize)->AudioCommandResult{
        let mut lock=match self.tracks.lock(){
            Ok(lock)=>lock,
            Err(_)=>return AudioCommandResult::ThreadClosed,
        };
        if index<lock.len(){
            lock.remove(index);
            AudioCommandResult::Ok
        }
        else{
            AudioCommandResult::NoSuchTrack
        }
    }

    /// Удаляет все треки из массива треков.
    /// 
    /// Removes all tracks from the track array.
    pub fn remove_all_tracks(&self)->AudioCommandResult{
        self.tracks.lock().unwrap().clear();
        AudioCommandResult::Ok
    }

    /// Запускает трек без повторов.
    ///
    /// Sets the track to play once.
    pub fn play_once(&self,index:usize)->AudioCommandResult{
        let stream_lock=self.stream.lock();
        if let Err(_)=stream_lock{
            return AudioCommandResult::NoStream
        }

        let tracks_lock=match self.tracks.lock(){
            Ok(lock)=>lock,
            Err(_)=>return AudioCommandResult::ThreadClosed,
        };

        if index>=tracks_lock.len(){
            return AudioCommandResult::NoSuchTrack
        }

        match self.command.send(AudioSystemCommand::PlayOnce(index)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Запускает трек, который постоянно повторяется.
    /// 
    /// Sets the track to play forever.
    pub fn play_forever(&self,index:usize)->AudioCommandResult{
        let stream_lock=self.stream.lock();
        if let Err(_)=stream_lock{
            return AudioCommandResult::NoStream
        }

        let tracks_lock=match self.tracks.lock(){
            Ok(lock)=>lock,
            Err(_)=>return AudioCommandResult::ThreadClosed,
        };

        if index>=tracks_lock.len(){
            return AudioCommandResult::NoSuchTrack
        }

        match self.command.send(AudioSystemCommand::PlayForever(index)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Запускает проигрывание канала.
    /// 
    /// Starts playing the stream.
    pub fn play(&self)->AudioCommandResult{
        let stream=match self.stream.lock(){
            LockResult::Ok(stream)=>stream.clone(),
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        match stream{
            Some(stream)=>{
                self.event_loop.play_stream(stream);
            }
            None=>{}
        }

        AudioCommandResult::Ok
    }
    /// Ставит на паузу проигрывание канала.
    /// 
    /// Pauses the stream.
    pub fn pause(&self)->AudioCommandResult{
        let stream=match self.stream.lock(){
            LockResult::Ok(stream)=>stream.clone(),
            LockResult::Err(_)=>return AudioCommandResult::ThreadClosed
        };

        match stream{
            Some(stream)=>{
                self.event_loop.pause_stream(stream);
            }
            None=>{}
        }

        AudioCommandResult::Ok
    }

    /// Останавливает проигрывание путём удаления трека из буфера для вывода.
    /// 
    /// Stops playing by removing current track from playing buffer.
    pub fn stop(&self)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::Stop){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }

    /// Устанавливает громкость.
    /// 
    /// Sets the volume.
    pub fn set_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioSystemCommand::SetVolume(volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::ThreadClosed
        }
    }
}

impl Audio{
    fn event_loop_handler(
        event_loop:Arc<EventLoop>,
        format:Format,
        receiver:Receiver<AudioSystemCommand>,
        mut play:Play,
        tracks:Arc<Mutex<Vec<Track<i16>>>>,
        mut volume:f32,
    )->!{
        event_loop.clone().run(move|stream,result|{
            match receiver.try_recv(){
                Ok(command)=>match command{
                    AudioSystemCommand::PlayOnce(i)=>{
                        let lock=tracks.lock().unwrap();
                        let track:&Track<i16>=lock.get(i).unwrap();
                        let track_channels=track.channels();
                        let track=track.clone().into_iter(format.sample_rate);
                        let track=ChannelCountConverter::new(track,track_channels,format.channels);
                        play=Play::Once(track);
                    }
                    AudioSystemCommand::PlayForever(i)=>{
                        let lock=tracks.lock().unwrap();
                        let track:&Track<i16>=lock.get(i).unwrap();
                        let track_channels=track.channels();
                        let track=track.clone().endless_iter(format.sample_rate);
                        let track=ChannelCountConverter::new(track,track_channels,format.channels);
                        play=Play::Forever(track);
                    }
                    AudioSystemCommand::Stop=>{
                        play=Play::None;
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


            match &mut play{
                Play::None=>{}

                Play::Once(track)=>{
                    match result{
                        Ok(data)=>{
                            match data{
                                StreamData::Output{buffer:UnknownTypeOutputBuffer::I16(mut buffer)}
                                =>for b in buffer.iter_mut(){
                                    *b=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                }

                                StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(mut buffer)}
                                =>for b in buffer.iter_mut(){
                                    let sample=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                    *b=sample.to_u16();
                                }

                                StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(mut buffer)}
                                =>for b in buffer.iter_mut(){
                                    let sample=track.next().unwrap_or(0i16);
                                    *b=sample.to_f32()*volume;
                                }

                                _=>{}
                            }
                        }
                        Err(e)=>{
                            eprintln!("an error occurred on stream {:?}: {}",stream,e);
                            return
                        }
                    }
                }

                Play::Forever(track)=>{
                    match result{
                        Ok(data)=>{
                            match data{
                                StreamData::Output{buffer:UnknownTypeOutputBuffer::I16(mut buffer)}
                                =>for b in buffer.iter_mut(){
                                    *b=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                }

                                StreamData::Output{buffer:UnknownTypeOutputBuffer::U16(mut buffer)}
                                =>for b in buffer.iter_mut(){
                                    let sample=(track.next().unwrap_or(0i16) as f32 * volume) as i16;
                                    *b=sample.to_u16();
                                }

                                StreamData::Output{buffer:UnknownTypeOutputBuffer::F32(mut buffer)}
                                =>for b in buffer.iter_mut(){
                                    let sample=track.next().unwrap_or(0i16);
                                    *b=sample.to_f32()*volume;
                                }

                                _=>{}
                            }
                        }
                        Err(e)=>{
                            eprintln!("an error occurred on stream {:?}: {}",stream,e);
                            return
                        }
                    }
                }
            }
        })
    }
}

/// Отправляет команду для остановки и ожидает окончание работы потока.
/// 
/// Sends closing command and then waits for the thread to finish.
impl Drop for Audio{
    fn drop(&mut self){
        let _=self.command.send(AudioSystemCommand::Close);
        if let Some(thread)=self.thread.take(){
            let _=thread.join();
        }
    }
}


/// Тип аудио вывода. Audio output type.
#[derive(Clone)]
pub enum AudioOutputType{
    Mono,
    Stereo,
}

impl AudioOutputType{
    pub fn into_channels(self)->u16{
        match self{
            AudioOutputType::Mono=>1u16,
            AudioOutputType::Stereo=>2u16,
        }
    }
}

#[derive(Clone)]
pub struct AudioSettings{
    /// The default is 0.5.
    pub volume:f32,

    /// The default is Stereo.
    pub output_type:AudioOutputType,

    /// Вместимость массива для треков.
    /// 
    /// The capacity of the track array.
    /// 
    /// The default is 1.
    pub track_array_capacity:usize,
}

impl AudioSettings{
    pub fn new()->AudioSettings{
        Self{
            volume:0.5f32,
            output_type:AudioOutputType::Stereo,
            track_array_capacity:1,
        }
    }
}