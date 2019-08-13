use std::fs::File;
use std::io::BufReader;

use rodio::*;

pub struct Bruback {
    device: Device,
    music_track_sink: Sink,
    sound_effect_sink: Sink
}

impl Bruback {
    pub fn new() -> Bruback {
        let device = rodio::default_output_device().unwrap();
        Bruback {
            music_track_sink: Sink::new(&device),
            sound_effect_sink: Sink::new(&device),
            device
        }
    }

    pub fn play_music(&mut self, file_path: String) {
        let file = File::open(file_path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.music_track_sink.append(source);
    }
    
    pub fn set_music_volume(&mut self, level: f32) {
        self.music_track_sink.set_volume(level);
    }

    pub fn set_effect_volume(&mut self, level: f32) {
        self.sound_effect_sink.set_volume(level);
    }

    pub fn play_sound_effect(&mut self, file_path: String) {
        self.sound_effect_sink.set_volume(0.05);

        let file = File::open(file_path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.sound_effect_sink.append(source);

    }
}