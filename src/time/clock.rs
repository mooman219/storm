use crate::time::convert::*;
use crate::time::platform::sleep;
use core::time::Duration;
use instant::Instant;

pub struct Clock {
    last_tick: Instant,
    target: u64,
    delta: f32,
}

impl Clock {
    pub fn new(tps: u64) -> Clock {
        Clock {
            last_tick: Instant::now(),
            target: Clock::tps_to_target(tps),
            delta: 0f32,
        }
    }

    fn tps_to_target(tps: u64) -> u64 {
        if tps == 0 {
            0
        } else {
            NANOS_PER_SEC / tps
        }
    }

    #[inline]
    fn duration_to_delta(duration: &Duration) -> f32 {
        (as_nanoseconds(duration) as f64 / NANOS_PER_SEC as f64) as f32
    }

    /// Sets the target ticks per second for the clock.
    pub fn set_tps(&mut self, tps: u64) {
        self.target = Clock::tps_to_target(tps);
    }

    #[inline]
    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    /// To meet the target TPS, this function will sleep (or spin) until it's
    /// time for the next tick. The duration spent outside of this function is
    /// taken into account when calculating how long to sleep for.
    ///
    /// For example, if the target TPS is 100TPS, then each tick must take
    /// 10ms. If 4ms is spent outside of this function, then calling tick will
    /// sleep for 6ms.
    pub fn tick(&mut self) {
        // Find the duration since ticked last.
        let duration = as_nanoseconds(&self.last_tick.elapsed());
        // OS Sleep logic.
        if duration < self.target {
            let diff = self.target - duration;
            if diff > 1_250_000 {
                sleep(Duration::new(0, (diff - 1_250_000) as u32));
            }
        }
        // Spin sleep logic.
        let duration = as_nanoseconds(&self.last_tick.elapsed());
        if duration < self.target {
            let diff = self.target - duration;
            let spin_start = Instant::now();
            while as_nanoseconds(&spin_start.elapsed()) < diff {}
        }
        self.delta = Clock::duration_to_delta(&self.last_tick.elapsed());
        self.last_tick = Instant::now();
    }
}
