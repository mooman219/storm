use std::time::Duration;
use std::time::Instant;

/// The number of nanoseconds in a microsecond.
const NANOS_PER_MICRO: u64 = 1000;
/// The number of nanoseconds in a millisecond.
const NANOS_PER_MILLI: u64 = 1000_000;
/// The number of nanoseconds in seconds.
const NANOS_PER_SEC: u64 = 1_000_000_000;
/// The number of microseconds per second.
const MICROS_PER_SEC: u64 = 1000_000;
/// The number of milliseconds per second.
const MILLIS_PER_SEC: u64 = 1000;
/// The number of seconds in a minute.
const SECS_PER_MINUTE: u64 = 60;
/// The number of seconds in an hour.
const SECS_PER_HOUR: u64 = 3600;
/// The number of (non-leap) seconds in days.
const SECS_PER_DAY: u64 = 86400;
/// The number of (non-leap) seconds in a week.
const SECS_PER_WEEK: u64 = 604800;

pub struct FrameClock {
    last_tick: Instant,
    frames: u32,
    timer: u64,
}

impl FrameClock {
    pub fn new() -> FrameClock {
        FrameClock {
            last_tick: Instant::now(),
            frames: 0,
            timer: 0,
        }
    }

    pub fn tick(&mut self) {
        let tick = Instant::now();
        let duration = tick.duration_since(self.last_tick);
        self.last_tick = tick;
        self.frames += 1;
        self.timer += as_nanoseconds(&duration);
        if self.timer > NANOS_PER_SEC {
            println!("FPS: {}", self.frames);
            self.frames = 0;
            self.timer = 0;
        }
    }
}

pub fn as_milliseconds(duration: &Duration) -> u64 {
    let mut secs = duration.as_secs();
    let mut nanos = duration.subsec_nanos() as u64;
    secs *= MILLIS_PER_SEC;
    nanos /= NANOS_PER_MILLI;
    secs + nanos
}

pub fn as_nanoseconds(duration: &Duration) -> u64 {
    let mut secs = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;
    secs *= NANOS_PER_SEC;
    secs + nanos
}
