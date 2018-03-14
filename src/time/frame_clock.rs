use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use time::convert::*;

pub struct FrameClock {
    last_tick: Instant,
    target: u64,
}

impl FrameClock {
    pub fn new() -> FrameClock {
        FrameClock {
            last_tick: Instant::now(),
            target: 0,
        }
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.target = if fps == 0 { 0 } else { NANOS_PER_SEC / fps };
    }
    pub fn tick(&mut self) {
        // Sleep logic.
        let duration = as_nanoseconds(&self.last_tick.elapsed());
        if duration < self.target {
            let diff = (self.target - duration) as u32;
            if self.target < 16666667 {
                // Spin instead of sleeping above 60FPS.
                let spin_start = Instant::now();
                while as_nanoseconds(&spin_start.elapsed()) < diff as u64 {}
            } else {
                sleep(Duration::new(0, diff));
            }
        }
        self.last_tick = Instant::now();
    }
}
