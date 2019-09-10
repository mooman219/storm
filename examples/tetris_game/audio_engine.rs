use std::fs::File;
use std::io::BufReader;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use rodio::*;


pub enum AudioMessage {
    PlaySong(String),
    PlaySoundEffect(String),
    SetMusicVolume(f32),
    SetSoundEffectVolume(f32),
    Pause,
    Resume,
}

pub struct Bruback {
    sender: Sender<AudioMessage>,
}

impl Bruback {
    pub fn new() -> Bruback {
        let device = rodio::default_output_device().unwrap();
        
        let mut music_track_sink = Sink::new(&device);
        let mut sound_effect_sink = Sink::new(&device);

        let (tx, rx): (Sender<AudioMessage>, Receiver<AudioMessage>) = mpsc::channel();

        let child = thread::spawn(move || {
        let mut current_main_track : String = String::new();
        let mut songs = vec![];
        let mut song_count = 0;

            loop{
                
                let message = rx.try_recv();

                if let Ok(message) = message {
                    match message {
                        AudioMessage::PlaySong(song_file_path) => {
                            let file_path_copy = String::from(song_file_path.clone());
                            let file = File::open(song_file_path).unwrap();
                            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                            songs.push(file_path_copy);
                            song_count = 1;
                            music_track_sink.append(source);
                        },
                        AudioMessage::PlaySoundEffect(sound_effect_path) => {
                            let file = File::open(sound_effect_path).unwrap();
                            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                            sound_effect_sink.append(source);
                        },
                        AudioMessage::SetMusicVolume(volume) => {
                            music_track_sink.set_volume(volume);
                        },
                        AudioMessage::SetSoundEffectVolume(volume) => {
                            sound_effect_sink.set_volume(volume);
                        },
                        AudioMessage::Pause => {
                            music_track_sink.pause();
                        },
                        AudioMessage::Resume => {
                            music_track_sink.play();
                        }
                    }
                }

                if music_track_sink.empty() && song_count > 0 {
                    let song_path = String::from(songs[song_count % songs.len()].clone());
                    let file = File::open(song_path).unwrap();
                    song_count += 1;
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    music_track_sink.append(source);
                }
            }
        });

        Bruback {
            sender: tx
        }
    }

    pub fn play_music(&mut self, file_path: String) {
        self.sender.send(AudioMessage::PlaySong(file_path));
    }

    pub fn pause_music(&mut self) {
        self.sender.send(AudioMessage::Pause);
    }

    pub fn resume(&mut self) {
        self.sender.send(AudioMessage::Resume);
    }

    pub fn set_music_volume(&mut self, level: f32) {
        self.sender.send(AudioMessage::SetMusicVolume(level));
    }

    pub fn set_effect_volume(&mut self, level: f32) {
        self.sender.send(AudioMessage::SetSoundEffectVolume(level));
    }

    pub fn play_sound_effect(&mut self, file_path: String) {
        self.sender.send(AudioMessage::PlaySoundEffect(file_path));        
    }
}