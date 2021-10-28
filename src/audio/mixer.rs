use super::{sound::SoundInstance, spsc::Consumer};

pub struct Mixer {
    receiver: Consumer<SoundInstance>,
    active: Vec<SoundInstance>,
    sample_interval: f64,
}

impl Mixer {
    pub fn new(sample_rate: u32, receiver: Consumer<SoundInstance>) -> Mixer {
        Mixer {
            receiver,
            active: Vec::with_capacity(32),
            sample_interval: 1.0 / sample_rate as f64,
        }
    }

    pub fn sample(&mut self, out: &mut [[f32; 2]]) {
        while let Some(instance) = self.receiver.try_pop() {
            self.active.push(instance);
        }
        for target in out.iter_mut() {
            *target = [0.0, 0.0];
        }
        for instance in self.active.iter_mut() {
            instance.mix(self.sample_interval, out);
        }
    }
}
