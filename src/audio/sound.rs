use std::sync::atomic::{AtomicBool, AtomicU32};

use alloc::sync::Arc;

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

    pub fn play(&self) -> SoundControl {
        let (control, instance) = self.split();
        AudioState::ctx().send(instance);
        control
    }

    fn split(&self) -> (SoundControl, SoundInstance) {
        let shared = Arc::new(Shared {
            active: AtomicBool::new(false),
            gain: AtomicU32::new(f32::to_bits(0.2)),
            smooth: AtomicU32::new(f32::to_bits(0.1)),
        });
        let control = SoundControl {
            duration: self.duration(),
            shared: shared.clone(),
        };
        let instance = SoundInstance {
            source: self.clone(),
            time: 0.0,
            shared: shared,
        };
        (control, instance)
    }

    fn mix(&self, sample: f64, out: &mut [f32; 2]) {
        if sample < 0.0 || sample >= (self.samples.len() - 1) as f64 {
            return;
        }
        let whole = sample.trunc() as usize;
        let t = sample.fract() as f32;
        let a = self.samples[whole];
        let b = self.samples[whole + 1];
        out[0] += Sound::lerp(a[0], b[0], t) * 0.1;
        out[1] += Sound::lerp(a[1], b[1], t) * 0.1;
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }
}

struct Shared {
    active: AtomicBool,
    gain: AtomicU32,
    smooth: AtomicU32,
}

pub struct SoundInstance {
    source: Sound,
    time: f64,
    shared: Arc<Shared>,
}

/// Represents various controls for an instance of a Sound.
pub struct SoundControl {
    /// The duration of the sound in seconds.
    duration: f64,
    shared: Arc<Shared>,
}

impl SoundControl {
    /// The duration of the sound in seconds.
    pub fn duration(&self) -> f64 {
        self.duration
    }
}

impl SoundInstance {
    pub(crate) fn mix(&mut self, interval: f64, out: &mut [[f32; 2]]) {
        let initial_sample = self.time * self.source.sample_rate;
        let sample_rate = interval * self.source.sample_rate;

        for (x, target) in out.iter_mut().enumerate() {
            let x = x as f64;
            self.source.mix(sample_rate * x + initial_sample, target);
        }

        let duration = interval * (out.len() as f64);
        self.time += duration;
    }
}
