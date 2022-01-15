use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct Inner {
    volume: AtomicU64,
    paused: AtomicBool,
    stop: AtomicBool,
}

/// Various controls for managing an active sound.
#[repr(transparent)]
#[derive(Clone)]
pub struct SoundControl(Arc<Inner>);

impl SoundControl {
    pub(crate) fn new(volume: f32, smooth: f32, paused: bool) -> SoundControl {
        let volume = pack_volume(volume, smooth);
        SoundControl(Arc::new(Inner {
            volume: AtomicU64::new(volume),
            paused: AtomicBool::new(paused),
            stop: AtomicBool::new(false),
        }))
    }

    /// Sets the sound's volume.
    /// # Arguments
    ///
    /// * `volume` - A value between `[0, 1]`, where 0 is muted, and 1 is the sound's original volume.
    /// * `smooth` - The duration in seconds to fade the change in volume from the current value to
    /// the given value.
    pub fn set_volume(&self, volume: f32, smooth: f32) {
        let volume = pack_volume(volume, smooth);
        self.0.volume.store(volume, Ordering::Relaxed)
    }

    /// Pauses the sound. The sound can later be resumed.
    pub fn pause(&self) {
        self.0.paused.store(true, Ordering::Relaxed);
    }

    /// Resumes the sound. Only a paused sound can be resumed.
    pub fn resume(&self) {
        self.0.paused.store(false, Ordering::Relaxed);
    }

    /// Stops the sound. This action is irreversible.
    pub fn stop(&self) {
        self.0.stop.store(true, Ordering::Relaxed);
    }

    pub(crate) fn load_volume(&self) -> (f32, f32) {
        unpack_volume(self.0.volume.load(Ordering::Relaxed))
    }

    pub(crate) fn load_paused(&self) -> bool {
        self.0.paused.load(Ordering::Relaxed)
    }

    /// Returns if the sound is stopped. If the sound was manually stopped or finished playing, it
    /// will be marked as stopped.
    pub fn is_stopped(&self) -> bool {
        self.0.stop.load(Ordering::Relaxed)
    }
}

fn pack_volume(volume: f32, smooth: f32) -> u64 {
    let volume = if volume < 0.0 {
        0.0
    } else if volume > 1.0 {
        1.0
    } else {
        volume
    };
    let smooth = if smooth < 0.01 {
        0.01
    } else {
        smooth
    };
    ((volume.to_bits() as u64) << 32) | smooth.to_bits() as u64
}

fn unpack_volume(packed: u64) -> (f32, f32) {
    let volume = f32::from_bits((packed >> 32) as u32);
    let smooth = f32::from_bits(packed as u32);
    (volume, smooth)
}
