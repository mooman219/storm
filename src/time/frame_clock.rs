use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use time::convert::*;

pub struct FrameClock {
    last_fps_tick: Instant,
    last_tick: Instant,
    target: u64,
    frames: u32,
    last_fps: u32,
}

impl FrameClock {
    pub fn new() -> FrameClock {
        FrameClock {
            last_fps_tick: Instant::now(),
            last_tick: Instant::now(),
            target: 0,
            frames: 0,
            last_fps: 0,
        }
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.target = if fps == 0 { 0 } else { NANOS_PER_SEC / fps };
    }

    pub fn get_last_fps(&self) -> u32 {
        return self.last_fps;
    }

    pub fn tick(&mut self) {
        // FPS tracking.
        self.frames += 1;
        if as_nanoseconds(&self.last_fps_tick.elapsed()) > NANOS_PER_SEC {
            println!("FPS: {}", self.frames);
            self.last_fps = self.frames;
            self.last_fps_tick = Instant::now();
            self.frames = 0;
        }
        // Sleep logic.
        let duration = as_nanoseconds(&self.last_tick.elapsed());
        if duration < self.target {
            let diff = (self.target - duration) as u32;
            if self.target < 10000000 {
                // Spin instead of sleeping above 100FPS.
                let spin_start = Instant::now();
                while as_nanoseconds(&spin_start.elapsed()) < diff as u64 {}
            } else {
                sleep(Duration::new(0, diff));
            }
        }
        self.last_tick = Instant::now();
    }
}
