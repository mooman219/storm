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
        let duration = self.last_tick.elapsed();
        self.last_tick = Instant::now();
        self.frames += 1;
        self.timer += as_nanoseconds(&duration);
        if self.timer > NANOS_PER_SEC {
            println!("FPS: {}", self.frames);
            self.frames = 0;
            self.timer = 0;
        }
    }
}