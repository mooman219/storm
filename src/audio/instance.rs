use crate::audio::{Sound, SoundControl};
use crate::math::Interpolation;

pub struct SoundInstance {
    control: SoundControl,
    source: Sound,
    volume: Interpolation,
    paused: bool,
    smooth: f32,
    time: f64,
}

impl SoundInstance {
    pub fn new(sound: &Sound, control: &SoundControl) -> SoundInstance {
        let (volume, smooth) = control.load_volume();
        let paused = control.load_paused();
        SoundInstance {
            control: control.clone(),
            source: sound.clone(),
            volume: Interpolation::new(0.0, volume),
            paused,
            smooth: smooth,
            time: 0.0,
        }
    }

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

pub fn make(sound: &Sound, volume: f32, smooth: f32, paused: bool) -> (SoundControl, SoundInstance) {
    let control = SoundControl::new(volume, smooth, paused);
    let instance = SoundInstance::new(sound, &control);
    (control, instance)
}

trait Perceptual {
    fn perceptual(&self) -> Self;
}

impl Perceptual for f32 {
    fn perceptual(&self) -> Self {
        self * self
    }
}
