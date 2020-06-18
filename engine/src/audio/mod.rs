use std::{
    path::Path,
    fs::File,
    thread::JoinHandle,
    sync::mpsc::{channel,Sender},
};

use rodio::{
    Source,
    Sink,
    Decoder,
    source::Buffered,
};

/// Результат выполнения команды. The result of an audio command.
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum AudioCommandResult{
    Ok,
    NoSuchTrack,
    AudioThreadClosed,
    FileError,
    DecodeError,
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

/// Повторение трека. Repeating a track.
pub enum Repeat{
    Once,
    Forever,
}

enum AudioCommand{
    PlayOnce(Buffered<Decoder<File>>),
    PlayForever(Buffered<Decoder<File>>),
    SetChannel(usize),
    SetVolume(f32),
    Pause,
    Unpause,
    Close,
}

unsafe impl std::marker::Sync for AudioCommand{}
unsafe impl std::marker::Send for AudioCommand{}

/// Система управления звуком. Audio operation system.
///
/// Пока что только для вывода. Output only for now.
/// 
/// Запускается в отдельном потоке, чтобы WINAPI не ругался.
/// Передача команд осуществляется с помощью `std::sync::mpsc::Sender` и `std::sync::mpsc::Receiver`.
/// 
/// Так же хранит звуковые треки, которые можно запустить.
pub struct Audio{
    command:Sender<AudioCommand>,
    thread:Option<JoinHandle<()>>,
    tracks:Vec<Buffered<Decoder<File>>>
}

impl Audio{
    /// Подключение к текущему устройству.
    /// Запуск потока и подключение канала связи.
    /// 
    /// Connects to the current device.
    /// Spawns a thread and creates communication channels.
    pub fn new(channels:usize)->Audio{
        let (sender,receiver)=channel();
        let thread=std::thread::spawn(move||{
            let device=rodio::default_output_device().unwrap();
            let mut sinks=Vec::with_capacity(channels);
            for _ in 0..channels{
                sinks.push(Sink::new(&device))
            }

            let mut current_channel=&mut sinks[0];

            loop{
                match receiver.recv(){
                    Ok(command)=>{
                        match command{
                            AudioCommand::PlayOnce(track)=>{
                                *current_channel=Sink::new(&device);
                                current_channel.append(track.stoppable());
                            }
                            AudioCommand::PlayForever(track)=>{
                                current_channel.append(track.stoppable().repeat_infinite());
                            }
                            AudioCommand::SetChannel(channel)=>{
                                current_channel=&mut sinks[channel]
                            }
                            AudioCommand::SetVolume(volume)=>{
                                current_channel.set_volume(volume)
                            }
                            AudioCommand::Pause=>{
                                current_channel.pause()
                            }
                            AudioCommand::Unpause=>{
                                current_channel.play()
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

    /// Добавляет звуковой трек.
    /// 
    /// Adds sound track.
    pub fn add_track<P:AsRef<Path>>(&mut self,path:P)->AudioCommandResult{
        let file=match File::open(path){
            Ok(file)=>file,
            Err(_)=>return AudioCommandResult::FileError
        };
        let track=match rodio::Decoder::new(file){
            Ok(track)=>track.buffered(),
            Err(_)=>return AudioCommandResult::DecodeError
        };
        self.tracks.push(track);
        AudioCommandResult::Ok
    }

    /// Удаляет звуковой трек.
    /// 
    /// Removes sound track.
    pub fn remove_track(&mut self,index:usize){
        self.tracks.remove(index);
    }

    /// Возвращает количество доступных треков.
    /// 
    /// Returns amount of tracks.
    pub fn tracks_len(&self)->usize{
        self.tracks.len()
    }

    /// Удаляет все аудио треки.
    /// 
    /// Deletes all audio tracks.
    pub fn delete_tracks(&mut self){
        self.tracks.clear()
    }

    /// Запускает трек.
    /// 
    /// Передаёт трек аудио потоку, отчищает звуковой буфер,
    /// добавляет трек и запускает его.
    /// 
    /// Starts playing a track.
    /// 
    /// Sends the track to the audio thread, clears the audio buffer,
    /// adds the track and starts playing it.
    pub fn play_track(&self,index:usize,repeat:Repeat)->AudioCommandResult{
        let track=match self.tracks.get(index){
            Some(track)=>track.clone(),
            None=>return AudioCommandResult::NoSuchTrack
        };
        let command=match repeat{
            Repeat::Once=>AudioCommand::PlayOnce(track),
            Repeat::Forever=>AudioCommand::PlayForever(track),
        };
        match self.command.send(command){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::AudioThreadClosed
        }
    }

    /// Устанавливает громкость.
    /// 
    /// Sets the volume.
    /// 
    /// 0 <= volume <= 1
    pub fn set_volume(&self,volume:f32)->AudioCommandResult{
        match self.command.send(AudioCommand::SetVolume(volume)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::AudioThreadClosed
        }
    }


    /// Устанавливает текущий канал для управления им.
    /// 
    /// Аудио поток паникует, если нет такого канала.
    /// 
    /// Sets current channel to operate it.
    /// 
    /// Audio thread panics, if there is no such channel.
    pub fn set_channel(&self,channel:usize)->AudioCommandResult{
        match self.command.send(AudioCommand::SetChannel(channel)){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::AudioThreadClosed
        }
    }

    /// Остановливает проигрывание.
    /// 
    /// Pauses playing.
    pub fn pause(&self)->AudioCommandResult{
        match self.command.send(AudioCommand::Pause){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::AudioThreadClosed
        }
    }

    /// Запускает проигрывание после паузы.
    /// 
    /// Starts playing after a pause.
    pub fn unpause(&self)->AudioCommandResult{
        match self.command.send(AudioCommand::Unpause){
            Ok(())=>AudioCommandResult::Ok,
            Err(_)=>AudioCommandResult::AudioThreadClosed
        }
    }
}

/// Посылает команду об остановке и ждёт пока поток закончил работу.
/// 
/// Sends stopping command and waits until thread is closed.
impl Drop for Audio{
    fn drop(&mut self){
        self.command.send(AudioCommand::Close);
        if let Some(thread)=self.thread.take(){
            thread.join();
        }
    }
}