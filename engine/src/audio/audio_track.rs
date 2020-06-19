use super::sample_rate::SampleRateConverter;

use std::path::Path;
use std::fs::File;

use minimp3::Decoder;

use cpal::{
    SampleRate,
    SampleFormat,
};

#[derive(Debug)]
pub enum TrackResult<T>{
    Ok(Track<T>),
    FileError(std::io::Error),
    NoData,
}

impl<T:std::fmt::Debug> TrackResult<T>{
    pub fn unwrap(self)->Track<T>{
        if let TrackResult::Ok(track)=self{
            track
        }
        else{
            panic!("{:?}",self)
        }
    }
}

#[derive(Clone,Debug)]
pub struct Track<T>{
    data:Vec<T>,
    channels:u16,
    sample_rate:SampleRate,
    sample_format:SampleFormat,
}

impl Track<i16>{
    pub fn new<P:AsRef<Path>>(path:P)->TrackResult<i16>{
        let mut data=Vec::new();

        let file=match File::open(path){
            Ok(file)=>file,
            Err(e)=>return TrackResult::FileError(e),
        };

        let mut decoder=Decoder::new(file);
        let (channels,sample_rate,sample_format)=match decoder.next_frame(){
            Ok(mut frame)=>{
                data.append(&mut frame.data);
                (
                    frame.channels,
                    SampleRate(frame.sample_rate as u32),
                    SampleFormat::I16
                )
            }
            Err(_)=>return TrackResult::NoData
        };

        while let Ok(mut frame)=decoder.next_frame(){
            data.append(&mut frame.data);
        }

        TrackResult::Ok(Self{
            data,
            channels:channels as u16,
            sample_rate,
            sample_format,
        })
    }

    pub fn endless_iter(self,sample_rate:SampleRate)->SampleRateConverter<std::iter::Cycle<std::vec::IntoIter<i16>>>{
        SampleRateConverter::new(self.data.into_iter().cycle(),self.sample_rate,sample_rate,self.channels)
    }

    pub fn into_iter(self,sample_rate:SampleRate)->SampleRateConverter<std::vec::IntoIter<i16>>{
        SampleRateConverter::new(self.data.into_iter(),self.sample_rate,sample_rate,self.channels)
    }
}

impl<T> Track<T>{
    pub fn data(&self)->&Vec<T>{
        &self.data
    }

    pub fn sample_rate(&self)->SampleRate{
        self.sample_rate
    }

    pub fn sample_format(&self)->SampleFormat{
        self.sample_format
    }

    pub fn len(&self)->usize{
        self.data.len()
    }
}