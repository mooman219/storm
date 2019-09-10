use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use rodio::*;

pub type SinkID = u32;

pub struct SinkController {
    sink: Sink,
    tracks_to_play: Vec<String>,
    song_count: usize
}

impl SinkController {
    pub fn new(sink: Sink) -> SinkController {
        SinkController {
            sink,
            tracks_to_play: vec![],
            song_count: 0
        }
    }

    pub fn play_track(&mut self, track: String) {
        let file_path_copy = String::from(track.clone());
        let file = File::open(track).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.add_track(file_path_copy);
    }

    pub fn add_track(&mut self, track: String)  {
        self.tracks_to_play.push(track)
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn play(&mut self) {
        self.play();
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.set_volume(volume);
    }

    pub fn tick(&mut self) {
        if self.sink.empty() {
            let song_path = String::from(self.tracks_to_play[self.song_count % self.tracks_to_play.len()].clone());
            let file = File::open(song_path).unwrap();
            self.song_count += 1;
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            self.sink.append(source);
        }
    }
}

pub enum AudioMessage {
    NewSink(SinkID),
    PlayTrack(String, SinkID),
    SetSinkVolume(f32, SinkID),
    PauseSink(SinkID),
    ResumeSink(SinkID),
}

pub struct BrubackState {
    sinks: HashMap<SinkID, SinkController>,
    rx: Receiver<AudioMessage>,
    device: Device
}

impl BrubackState {
    pub fn new(rx: Receiver<AudioMessage>, device: Device) -> BrubackState {
        BrubackState {
            sinks: HashMap::new(),
            rx,
            device
        }
    }

    pub fn update(&mut self) { 
        loop{    
            let message = self.rx.try_recv();

            if let Ok(message) = message {
                match message {
                    AudioMessage::NewSink(sink_id) => {
                        self.sinks.insert(sink_id, SinkController::new(Sink::new(&self.device)));
                    },
                    AudioMessage::PlayTrack(track, sink_id) => {
                        let sink = self.sinks.get_mut(&sink_id).unwrap();
                        sink.play_track(track);
                    },
                    AudioMessage::PauseSink(sink_id) => {
                        let sink = self.sinks.get_mut(&sink_id).unwrap();
                        sink.pause();
                    },
                    AudioMessage::ResumeSink(sink_id) => {
                        let sink = self.sinks.get_mut(&sink_id).unwrap();
                        sink.play();
                    },
                    AudioMessage::SetSinkVolume(volume, sink_id) => {
                        let sink = self.sinks.get_mut(&sink_id).unwrap();
                        sink.set_volume(volume);
                    }
                }
            }

            for (_, sink) in self.sinks.iter_mut() {
                sink.tick();
            }
        }
    }
}

pub struct Bruback {
    sender: Sender<AudioMessage>,
    sink_id_count: SinkID
}

impl Bruback {
    pub fn new() -> Bruback {
        let device = rodio::default_output_device().unwrap();
        
        let mut music_track_sink = Sink::new(&device);
        let mut sound_effect_sink = Sink::new(&device);

        let (tx, rx): (Sender<AudioMessage>, Receiver<AudioMessage>) = mpsc::channel();

        let child = thread::spawn(move || {
            let mut bruback_state = BrubackState::new(rx, device);
            bruback_state.update();         
        });

        Bruback {
            sender: tx,
            sink_id_count: 0
        }
    }


    pub fn create_new_sink(&mut self) -> SinkID {
        let new_sink_id = self.sink_id_count + 1;
        self.sink_id_count += 1;
        let _ = self.sender.send(AudioMessage::NewSink(new_sink_id));
        return new_sink_id;
    }

    pub fn play_music(&mut self, file_path: String, sink_id: SinkID) {
        self.sender.send(AudioMessage::PlayTrack(file_path, sink_id));
    }

    pub fn pause_track(&mut self, sink_id: SinkID) {
        self.sender.send(AudioMessage::PauseSink(sink_id));
    }

    pub fn resume_track(&mut self, sink_id: SinkID) {
        self.sender.send(AudioMessage::ResumeSink(sink_id));
    }

    pub fn set_music_volume(&mut self, level: f32, sink_id: SinkID) {
        self.sender.send(AudioMessage::SetSinkVolume(level, sink_id));
    }
}