use crate::math::lerp;
// use core::f32;

pub struct Smoothed {
    packed: u64,
    previous: f32,
    target: f32,
    max: f32,
    t: f32,
    smooth: f32,
}

impl Smoothed {
    pub fn pack(volume: f32, smooth: f32) -> u64 {
        ((volume.to_bits() as u64) << 32) | smooth.to_bits() as u64
    }

    pub fn unpack(packed: u64) -> (f32, f32) {
        let volume = f32::from_bits((packed >> 32) as u32);
        let smooth = f32::from_bits(packed as u32);
        (volume, smooth)
    }

    pub fn new(packed: u64) -> Smoothed {
        let mut result = Smoothed {
            packed: 0,
            previous: 0.0,
            target: 0.0,
            max: 0.0,
            t: 1.0,
            smooth: 0.0,
        };
        result.set(packed);
        result
    }

    pub fn set(&mut self, packed: u64) {
        if self.packed == packed {
            return;
        }
        self.packed = packed;

        let (volume, smooth) = Smoothed::unpack(packed);
        self.previous = self.target;
        self.t = 0.0;

        if volume < 0.0 {
            self.target = 0.0;
        } else if volume > 1.0 {
            self.target = 1.0;
        } else {
            self.target = volume;
        }

        self.max = convert(self.target);

        if smooth < 0.05 {
            self.smooth = 0.05;
        } else {
            self.smooth = smooth;
        }
    }

    pub fn next(&mut self, duration: f32) -> f32 {
        if self.t >= 1.0 {
            return self.max;
        }
        let result = lerp(self.previous, self.target, self.t as f32);
        self.t += duration / self.smooth;
        convert(result)
    }
}

fn convert(volume: f32) -> f32 {
    volume * volume
}
