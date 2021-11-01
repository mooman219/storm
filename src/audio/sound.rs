use std::sync::atomic::{AtomicBool, AtomicU32};

use alloc::sync::Arc;

use crate::math::lerp;
use crate::AudioState;

#[derive(Copy, Clone, Debug)]
/// An error that prevents successful decoding of an audio stream.
pub enum SoundError {
    /// The channel count is unsupported. Only mono and stero sounds are supported.
    UnsupportedChannelCount,
    /// A feature in the audio file isn't supported by the parser.
    UnsupportedFeature,
    /// The audio file not formatted correctly for the encoding.
    InvalidFormat,
}

#[derive(Copy, Clone, Debug)]
enum Channels {
    Mono,
    Stero,
}

/// Basic audio container.
#[derive(Clone)]
pub struct Sound {
    sample_rate: f64,
    samples: Arc<[[f32; 2]]>,
}

impl Sound {
    /// Creates a new sound from a slice of stereo samples.
    pub fn new(sample_rate: u32, samples: Vec<[f32; 2]>) -> Result<Sound, SoundError> {
        let sample_rate = sample_rate as f64;
        Ok(Sound {
            sample_rate,
            samples: samples.into(),
        })
    }

    /// The duration of the sound in seconds.
    pub fn duration(&self) -> f64 {
        self.samples.len() as f64 / self.sample_rate
    }

    /// Plays a sound with a given volume.
    /// # Arguments
    ///
    /// * `volume` - A value between [0, 1], where 0 is muted, and 1 is the sound's original volume.
    /// # Returns
    ///
    /// * `SoundControl` - A handle to control sound properties during play.
    pub fn play(&self, volume: f32) -> SoundControl {
        let shared = Arc::new(Shared {
            active: AtomicBool::new(false),
            gain: AtomicU32::new(f32::to_bits(0.2)),
            smooth: AtomicU32::new(f32::to_bits(0.1)),
        });
        let control = SoundControl {
            shared: shared.clone(),
            duration: self.duration(),
        };
        let instance = SoundInstance {
            test: SineTest::new(261.63),
            shared: shared,
            source: self.clone(),
            time: 0.0,
            volume: Smoothed::new(volume),
        };
        AudioState::ctx().send(instance);
        control
    }

    fn mix(&self, sample: f64, amplitude: f32, out: &mut [f32; 2]) {
        if sample < 0.0 || sample + 1.0 >= self.samples.len() as f64 {
            return;
        }
        let trunc = sample.trunc();
        let whole = trunc as usize;
        let t = (sample - trunc) as f32;
        let a = self.samples[whole];
        let b = self.samples[whole + 1];
        out[0] += lerp(a[0], b[0], t) * amplitude;
        out[1] += lerp(a[1], b[1], t) * amplitude;
    }
}

struct Shared {
    active: AtomicBool,
    /// f32 encoded as a u32 that represents decibels.
    gain: AtomicU32,
    smooth: AtomicU32,
}

pub struct SoundInstance {
    test: SineTest,
    shared: Arc<Shared>,
    source: Sound,
    time: f64,
    volume: Smoothed,
}

pub struct SineTest {
    phase: f32,
    frequency: f32,
}
impl SineTest {
    pub fn new(frequency: f32) -> SineTest {
        SineTest {
            phase: 0.0,
            frequency: frequency * core::f32::consts::TAU,
        }
    }

    pub fn mix(&mut self, interval: f64, volume: &mut Smoothed, out: &mut [[f32; 2]]) {
        volume.set_increment(interval);
        let interval = interval as f32;
        for (i, target) in out.iter_mut().enumerate() {
            let time = interval * i as f32;
            let result = (time * self.frequency + self.phase).sin() * volume.next();
            target[0] += result;
            target[1] += result;
        }
        let time = interval * (out.len() as f32);
        self.phase = (self.phase + time * self.frequency) % core::f32::consts::TAU;
    }
}

impl SoundInstance {
    pub fn mix(&mut self, interval: f64, out: &mut [[f32; 2]]) {
        self.test.mix(interval, &mut self.volume, out);
    }

    // pub fn mix(&mut self, interval: f64, out: &mut [[f32; 2]]) {
    //     self.volume.set_increment(interval);
    //     let mut current_sample = self.time * self.source.sample_rate;
    //     let sample_rate = interval * self.source.sample_rate;

    //     for target in out.iter_mut() {
    //         self.source.mix(current_sample, self.volume.next(), target);
    //         current_sample += sample_rate;
    //     }

    //     let elapsed = interval * (out.len() as f64);
    //     self.time += elapsed;
    // }

    pub fn is_complete(&self) -> bool {
        self.time >= self.source.duration()
    }
}

/// Represents various controls for an instance of a Sound.
pub struct SoundControl {
    shared: Arc<Shared>,
    duration: f64,
}

impl SoundControl {
    /// The duration of the sound in seconds.
    pub fn duration(&self) -> f64 {
        self.duration
    }
}

pub struct Smoothed {
    previous: f32,
    target: f32,
    t: f64,
    inc: f64,
    smooth: f64,
}

impl Smoothed {
    pub fn new(volume: f32) -> Smoothed {
        Smoothed {
            previous: 0.0,
            target: volume,
            t: 0.0,
            inc: 0.0,
            smooth: 5.0,
        }
    }

    pub fn set_increment(&mut self, duration: f64) {
        self.inc = duration / self.smooth;
    }

    pub fn next(&mut self) -> f32 {
        if self.t >= 1.0 {
            return Smoothed::convert(self.target);
        }
        let result = lerp(self.previous, self.target, self.t as f32);
        self.t += self.inc;
        Smoothed::convert(result)
    }

    fn convert(volume: f32) -> f32 {
        volume * volume
    }
}
