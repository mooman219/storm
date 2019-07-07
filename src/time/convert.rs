use std::time::Duration;

/// The number of nanoseconds in a microsecond.
pub const NANOS_PER_MICRO: u64 = 1_000;
/// The number of nanoseconds in a millisecond.
pub const NANOS_PER_MILLI: u64 = 1_000_000;
/// The number of nanoseconds in seconds.
pub const NANOS_PER_SEC: u64 = 1_000_000_000;
/// The number of microseconds per second.
pub const MICROS_PER_SEC: u64 = 1_000_000;
/// The number of milliseconds per second.
pub const MILLIS_PER_SEC: u64 = 1_000;
/// The number of seconds in a minute.
pub const SECS_PER_MINUTE: u64 = 60;
/// The number of seconds in an hour.
pub const SECS_PER_HOUR: u64 = 3_600;
/// The number of (non-leap) seconds in days.
pub const SECS_PER_DAY: u64 = 86_400;

pub const SECOND: Duration = Duration::from_secs(1);
pub const MILLISECOND: Duration = Duration::from_millis(1);
pub const MICROSECOND: Duration = Duration::from_micros(1);
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
