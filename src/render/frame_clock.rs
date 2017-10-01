use std::time::Duration;
use std::time::Instant;

use render::time::*;

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