use std::time::Instant;
use std::time::Duration;

use render::time::*;

pub struct FrameClock {
    last_tick: Instant,
    last_duration: Duration,
    last_fps: u32,
    frames: u32,
    timer: u64,
}

impl FrameClock {
    pub fn new() -> FrameClock {
        FrameClock {
            last_tick: Instant::now(),
            last_duration: Duration::from_millis(0),
            last_fps: 0,
            frames: 0,
            timer: 0,
        }
    }

    pub fn get_fps(&self) -> u32 {
        self.last_fps
    }

    pub fn get_duration(&self) -> Duration {
        self.last_duration
    }

    pub fn tick(&mut self) {
        self.last_duration = self.last_tick.elapsed();
        self.last_tick = Instant::now();
        self.frames += 1;
        self.timer += as_nanoseconds(&self.last_duration);
        if self.timer > NANOS_PER_SEC {
            println!("FPS: {}", self.frames);
            self.last_fps = self.frames;
            self.frames = 0;
            self.timer = 0;
        }
    }
}