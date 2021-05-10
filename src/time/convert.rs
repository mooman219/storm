use core::time::Duration;

/// The number of nanoseconds per microsecond.
pub const NANOS_PER_MICRO: u64 = 1_000;
/// The number of nanoseconds per millisecond.
pub const NANOS_PER_MILLI: u64 = 1_000_000;
/// The number of nanoseconds per second.
pub const NANOS_PER_SEC: u64 = 1_000_000_000;
/// The number of microseconds per second.
pub const MICROS_PER_SEC: u64 = 1_000_000;
/// The number of milliseconds per second.
pub const MILLIS_PER_SEC: u64 = 1_000;
/// The number of seconds per minute.
pub const SECS_PER_MINUTE: u64 = 60;
/// The number of seconds per hour.
pub const SECS_PER_HOUR: u64 = 3_600;
/// The number of (non-leap) seconds in days.
pub const SECS_PER_DAY: u64 = 86_400;
// A const duration representing a second.
pub const SECOND: Duration = Duration::from_secs(1);
// A const duration representing a millisecond.
pub const MILLISECOND: Duration = Duration::from_millis(1);
// A const duration representing a microsecond.
pub const MICROSECOND: Duration = Duration::from_micros(1);
// A const duration representing a nanosecond.
pub const NANOSECOND: Duration = Duration::from_nanos(1);

#[inline]
pub fn as_days(duration: &Duration) -> u64 {
    let secs = duration.as_secs();
    secs / SECS_PER_DAY
}

#[inline]
pub fn as_hours(duration: &Duration) -> u64 {
    let secs = duration.as_secs();
    secs / SECS_PER_HOUR
}

#[inline]
pub fn as_minutes(duration: &Duration) -> u64 {
    let secs = duration.as_secs();
    secs / SECS_PER_MINUTE
}

#[inline]
pub fn as_seconds(duration: &Duration) -> u64 {
    duration.as_secs()
}

#[inline]
pub fn as_milliseconds(duration: &Duration) -> u64 {
    let mut secs = duration.as_secs();
    let mut nanos = duration.subsec_nanos() as u64;
    secs *= MILLIS_PER_SEC;
    nanos /= NANOS_PER_MILLI;
    secs + nanos
}

#[inline]
pub fn as_microseconds(duration: &Duration) -> u64 {
    let mut secs = duration.as_secs();
    let mut nanos = duration.subsec_nanos() as u64;
    secs *= MICROS_PER_SEC;
    nanos /= NANOS_PER_MICRO;
    secs + nanos
}

#[inline]
pub fn as_nanoseconds(duration: &Duration) -> u64 {
    let mut secs = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;
    secs *= NANOS_PER_SEC;
    secs + nanos
}
