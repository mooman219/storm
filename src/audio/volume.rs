use core::sync::atomic::{AtomicU64, Ordering};

use crate::math::lerp;

fn pack(volume: f32, smooth: f32) -> u64 {
    let volume = if volume < 0.0 {
        0.0
    } else if volume > 1.0 {
        1.0
    } else {
        volume
    };
    let smooth = if smooth < 0.05 {
        0.05
    } else {
        smooth
    };
    ((volume.to_bits() as u64) << 32) | smooth.to_bits() as u64
}

fn unpack(packed: u64) -> (f32, f32) {
    let volume = f32::from_bits((packed >> 32) as u32);
    let smooth = f32::from_bits(packed as u32);
    (volume, smooth)
}

#[repr(transparent)]
pub struct VolumeShared {
    state: AtomicU64,
}

impl VolumeShared {
    pub fn new(volume: f32, smooth: f32) -> VolumeShared {
        VolumeShared {
            state: AtomicU64::new(pack(volume, smooth)),
        }
    }

    pub fn store(&self, volume: f32, smooth: f32) {
        self.state.store(pack(volume, smooth), Ordering::Relaxed)
    }

    fn load(&self) -> (f32, f32) {
        unpack(self.state.load(Ordering::Relaxed))
    }
}

pub struct VolumeInstance {
    previous: f32,
    target: f32,
    max: f32,
    t: f32,
    smooth: f32,
}

impl VolumeInstance {
    pub fn new(shared: &VolumeShared) -> VolumeInstance {
        let mut result = VolumeInstance {
            previous: 0.0,
            target: 0.0,
            max: 0.0,
            t: 1.0,
            smooth: 0.0,
        };
        result.sync(shared);
        result
    }

    pub fn sync(&mut self, shared: &VolumeShared) {
        let (volume, smooth) = shared.load();
        if volume == self.target && smooth == self.smooth {
            return;
        }
        self.previous = lerp(self.previous, self.target, self.t as f32);
        self.t = 0.0;
        self.target = volume;
        self.max = convert(volume);
        self.smooth = smooth;
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
