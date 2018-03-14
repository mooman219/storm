use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use time::convert::*;

pub struct Clock {
    last_tick: Instant,
    target: u64,
}

impl Clock {
    pub fn new(tps: u64) -> Clock {
        Clock {
            last_tick: Instant::now(),
            target: Clock::tps_to_target(tps),
        }
    }

    fn tps_to_target(tps: u64) -> u64 {
        if tps == 0 {
            0
        } else {
            NANOS_PER_SEC / tps
        }
    }

    /// Sets the target ticks per second for the clock.
    pub fn set_tps(&mut self, tps: u64) {
        self.target = Clock::tps_to_target(tps);
    }

    /// To meet the target TPS, this function will sleep (or spin) until it's
    /// time for the next tick. The duration spent outside of this function is
    /// taken into account when calculating how long to sleep for.
    ///
    /// For example, if the target TPS is 100TPS, then each tick must take
    /// 10ms. If 4ms is spent outside of this function, then calling tick will
    /// sleep for 6ms.
    pub fn tick(&mut self) {
        // Sleep logic.
        let duration = as_nanoseconds(&self.last_tick.elapsed());
        if duration < self.target {
            let diff = (self.target - duration) as u32;
            if self.target < 16666667 {
                // Spin instead of sleeping above 60TPS.
                let spin_start = Instant::now();
                while as_nanoseconds(&spin_start.elapsed()) < diff as u64 {}
            } else {
                sleep(Duration::new(0, diff));
            }
        }
        self.last_tick = Instant::now();
    }
}
