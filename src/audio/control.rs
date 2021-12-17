use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::audio::Sound;
use crate::math::Interpolation;

/// Various controls for managing an active sound.
pub struct SoundControl {
    volume: AtomicU64,
    paused: AtomicBool,
    stop: AtomicBool,
}

impl SoundControl {
    /// Sets the sound's volume.
    /// # Arguments
    ///
    /// * `volume` - A value between `[0, 1]`, where 0 is muted, and 1 is the sound's original volume.
    /// * `smooth` - The duration in seconds to fade the change in volume from the current value to
    /// the given value.
    pub fn set_volume(&self, volume: f32, smooth: f32) {
        let (volume, smooth) = clamp_volume(volume, smooth);
        self.volume.store(pack(volume, smooth), Ordering::Relaxed)
    }

    /// Pauses the sound. The sound can later be resumed.
    pub fn pause(&self) {
        self.paused.store(true, Ordering::Relaxed);
    }

    /// Resumes the sound. Only a paused sound can be resumed.
    pub fn resume(&self) {
        self.paused.store(false, Ordering::Relaxed);
    }

    /// Stops the sound. This action is irreversible.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub(crate) fn load_volume(&self) -> (f32, f32) {
        unpack(self.volume.load(Ordering::Relaxed))
    }

    pub(crate) fn load_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    pub(crate) fn load_stop(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }
}

pub struct SoundInstance {
    control: Arc<SoundControl>,
    source: Sound,
    volume: Interpolation,
    paused: bool,
    smooth: f32,
    time: f64,
}

impl SoundInstance {
    pub fn mix(&mut self, interval: f32, out: &mut [[f32; 2]]) -> bool {
        // Stopping the sound.
        if self.control.load_stop() {
            return true;
        }

        // Sync volume.
        let (volume, smooth) = self.control.load_volume();
        if volume != self.volume.end() || smooth != self.smooth {
            self.volume.update(volume);
            self.smooth = 1.0 / smooth;
        }

        // Current and next state are paused.
        let paused = self.control.load_paused();
        if self.paused && paused {
            return false;
        }

        let mut sample = self.time * self.source.sample_rate();
        let rate = (interval as f64) * self.source.sample_rate();

        if self.paused != paused {
            let (start, step) = if paused {
                let start = self.volume.get();
                let step = -start / (out.len() as f32);
                (start, step)
            } else {
                let start = 0.0;
                let step = self.volume.get() / (out.len() as f32);
                (start, step)
            };
            for (index, target) in out.iter_mut().enumerate() {
                let index = index as f32;
                let amplitude = (start + step * index).perceptual();
                self.source.mix(sample, amplitude, target);
                sample += rate;
            }
            self.paused = paused;
        } else if self.volume.progress() == 1.0 {
            let amplitude = self.volume.get().perceptual();
            for target in out.iter_mut() {
                self.source.mix(sample, amplitude, target);
                sample += rate;
            }
        } else {
            let progress = interval / self.smooth;
            for target in out.iter_mut() {
                let amplitude = self.volume.get().perceptual();
                self.source.mix(sample, amplitude, target);
                self.volume.advance(progress);
                sample += rate;
            }
        }

        self.time += (interval as f64) * (out.len() as f64);
        self.time >= self.source.duration()
    }
}

pub fn make(sound: &Sound, volume: f32, smooth: f32, paused: bool) -> (Arc<SoundControl>, SoundInstance) {
    let (volume, smooth) = clamp_volume(volume, smooth);
    let control = Arc::new(SoundControl {
        volume: AtomicU64::new(pack(volume, smooth)),
        paused: AtomicBool::new(paused),
        stop: AtomicBool::new(false),
    });
    let instance = SoundInstance {
        control: control.clone(),
        source: sound.clone(),
        volume: Interpolation::new(0.0, volume),
        paused,
        smooth: smooth,
        time: 0.0,
    };
    (control, instance)
}

fn clamp_volume(volume: f32, smooth: f32) -> (f32, f32) {
    let volume = if volume < 0.0 {
        0.0
    } else if volume > 1.0 {
        1.0
    } else {
        volume
    };
    let smooth = if smooth < 0.02 {
        0.02
    } else {
        smooth
    };
    (volume, smooth)
}

fn pack(volume: f32, smooth: f32) -> u64 {
    ((volume.to_bits() as u64) << 32) | smooth.to_bits() as u64
}

fn unpack(packed: u64) -> (f32, f32) {
    let volume = f32::from_bits((packed >> 32) as u32);
    let smooth = f32::from_bits(packed as u32);
    (volume, smooth)
}

trait Perceptual {
    fn perceptual(&self) -> Self;
}

impl Perceptual for f32 {
    fn perceptual(&self) -> Self {
        self * self
    }
}
