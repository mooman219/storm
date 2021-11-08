use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;

use crate::audio::volume::{VolumeInstance, VolumeShared};
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
    pub fn play(&self, volume: f32, smooth: f32) -> SoundControl {
        let volume_shared = VolumeShared::new(volume, smooth);
        let volume = VolumeInstance::new(&volume_shared);

        let shared = Arc::new(Shared {
            active: AtomicBool::new(false),
            volume: volume_shared,
        });
        let control = SoundControl {
            shared: shared.clone(),
            duration: self.duration(),
        };
        let instance = SoundInstance {
            shared: shared,
            source: self.clone(),
            time: 0.0,
            volume: volume,
        };
        AudioState::ctx().send(instance);
        control
    }

    fn mix(&self, sample: f64, amplitude: f32, out: &mut [f32; 2]) {
        if sample < 0.0 {
            return;
        }
        let trunc = sample.trunc();
        let whole = trunc as usize;
        if whole + 1 >= self.samples.len() {
            return;
        }
        let t = (sample - trunc) as f32;
        let a = unsafe { self.samples.get_unchecked(whole) };
        let b = unsafe { self.samples.get_unchecked(whole + 1) };
        out[0] += lerp(a[0], b[0], t) * amplitude;
        out[1] += lerp(a[1], b[1], t) * amplitude;
    }
}

struct Shared {
    active: AtomicBool,
    volume: VolumeShared,
}

pub struct SoundInstance {
    shared: Arc<Shared>,
    source: Sound,
    time: f64,
    volume: VolumeInstance,
}

impl SoundInstance {
    pub fn mix(&mut self, interval: f32, out: &mut [[f32; 2]]) {
        self.volume.sync(&self.shared.volume);
        let mut sample = self.time * self.source.sample_rate;
        let rate = (interval as f64) * self.source.sample_rate;

        for target in out.iter_mut() {
            let amplitude = self.volume.next(interval);
            self.source.mix(sample, amplitude, target);
            sample += rate;
        }

        self.time += (interval as f64) * (out.len() as f64);
    }

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

    /// Sets the sound's volume.
    /// # Arguments
    ///
    /// * `volume` - A value between [0, 1], where 0 is muted, and 1 is the sound's original volume.
    /// * `smooth` - The duration in seconds to fade the change in volume from the current value to
    /// the given value.
    pub fn set_volume(&mut self, volume: f32, smooth: f32) {
        self.shared.volume.store(volume, smooth);
    }
}
