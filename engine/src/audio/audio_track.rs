use super::sample_rate::SampleRateConverter;

use std::path::Path;
use std::fs::File;

use minimp3::Decoder;

use cpal::SampleRate;

#[derive(Clone)]
pub struct Track{
    data:Vec<i16>,
    sample_rate:SampleRate,
}

impl Track{
    pub fn new<P:AsRef<Path>>(path:P)->Track{
        let mut data=Vec::new();
        let mut decoder=Decoder::new(File::open(path).unwrap());
        let sample_rate=if let Ok(mut frame)=decoder.next_frame(){
            data.append(&mut frame.data);
            SampleRate(frame.sample_rate as u32)
        }
        else{
            SampleRate(0)
        };

        while let Ok(mut frame)=decoder.next_frame(){
            data.append(&mut frame.data);
        }

        Self{
            data,
            sample_rate
        }
    }

    pub fn len(&self)->usize{
        self.data.len()
    }

    pub fn endless_iter(self,sample_rate:SampleRate)->SampleRateConverter<std::iter::Cycle<std::vec::IntoIter<i16>>>{
        SampleRateConverter::new(self.data.into_iter().cycle(),self.sample_rate,sample_rate,2)
    }

    pub fn toSampleRateConverter(self,sample_rate:SampleRate)->SampleRateConverter<std::vec::IntoIter<i16>>{
        SampleRateConverter::new(self.data.into_iter(),self.sample_rate,sample_rate,2)
    }
}