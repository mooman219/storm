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

    pub fn mix(&mut self, interval: f32, gain: &mut Smoothed, out: &mut [[f32; 2]]) {
        let interval = interval as f32;
        for (i, target) in out.iter_mut().enumerate() {
            let time = interval * i as f32;
            let result = (time * self.frequency + self.phase).sin() * gain.next(interval);
            target[0] += result;
            target[1] += result;
        }
        let time = interval * (out.len() as f32);
        self.phase = (self.phase + time * self.frequency) % core::f32::consts::TAU;
    }
}
