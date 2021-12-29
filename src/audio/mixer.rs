use crate::audio::SoundInstance;
use crate::sync::Consumer;
use alloc::vec::Vec;

pub struct Mixer {
    receiver: Consumer<SoundInstance>,
    active: Vec<SoundInstance>,
    sample_interval: f32,
}

impl Mixer {
    pub fn new(sample_rate: u32, receiver: Consumer<SoundInstance>) -> Mixer {
        Mixer {
            receiver,
            active: Vec::with_capacity(32),
            sample_interval: 1.0 / sample_rate as f32,
        }
    }

    pub fn sample(&mut self, out: &mut [[f32; 2]]) {
        while let Some(instance) = self.receiver.try_pop() {
            self.active.push(instance);
        }

        for target in out.iter_mut() {
            *target = [0.0, 0.0];
        }

        let mut index = 0;
        while index < self.active.len() {
            let instance = &mut self.active[index];
            if instance.mix(self.sample_interval, out) {
                self.active.swap_remove(index);
            } else {
                index += 1;
            }
        }
    }
}
