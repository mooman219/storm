use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;

static mut GLOBAL: WasmHasNoThreads = WasmHasNoThreads {
    perf: None,
};

struct WasmHasNoThreads {
    perf: Option<web_sys::Performance>,
}

unsafe impl Sync for WasmHasNoThreads {}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    now: Duration,
}

impl Instant {
    pub fn now() -> Instant {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let now = unsafe {
            if let Some(perf) = &GLOBAL.perf {
                perf.now()
            } else {
                let perf = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
                    .expect("Getting performance from global")
                    .unchecked_into::<web_sys::Performance>();
                let now = perf.now();
                GLOBAL.perf = Some(perf);
                now
            }
        };
        let now =
            Duration::from_millis(now.trunc() as u64) + Duration::from_nanos((now.fract() * 1.0e6) as u64);
        Instant {
            now,
        }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        if let Some(result) = self.checked_sub(earlier.now) {
            result.now
        } else {
            panic!("Supplied instant is later than self")
        }
    }

    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        if let Some(result) = self.checked_sub(earlier.now) {
            Some(result.now)
        } else {
            None
        }
    }

    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        if let Some(result) = self.checked_duration_since(earlier) {
            result
        } else {
            Duration::default()
        }
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        if let Some(result) = self.now.checked_add(duration) {
            Some(Instant {
                now: result,
            })
        } else {
            None
        }
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        if let Some(result) = self.now.checked_sub(duration) {
            Some(Instant {
                now: result,
            })
        } else {
            None
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self {
        self.checked_add(rhs).expect("Overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self {
        self.checked_sub(rhs).expect("Overflow when subtracting duration from instant")
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}
