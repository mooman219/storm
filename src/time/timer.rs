use std::time::Instant;
use time::convert::*;

pub struct Timer {
    label: &'static str,
    start: Instant,
    last_display: Instant,
    duration: u64,
    invocations: u64,
}

impl Timer {
    pub fn new(label: &'static str) -> Timer {
        Timer {
            label: label,
            start: Instant::now(),
            last_display: Instant::now(),
            duration: 0,
            invocations: 0,
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.duration += as_nanoseconds(&self.start.elapsed());
        self.invocations += 1;
        if as_nanoseconds(&self.last_display.elapsed()) > NANOS_PER_SEC {
            self.last_display = Instant::now();
            let average = (self.duration as f32) / (self.invocations as f32);
            println!("{}: {} ns / {} tps", self.label, average, self.invocations);
            self.duration = 0;
            self.invocations = 0;
        }
    }
}
