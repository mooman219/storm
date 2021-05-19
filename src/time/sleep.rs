use core::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub fn sleep(duration: Duration) {
    use crate::time::Instant;
    const SLEEP_ERROR: Duration = Duration::from_micros(1250);

    let start = Instant::now();
    if duration > SLEEP_ERROR {
        std::thread::sleep(duration - SLEEP_ERROR);
    }
    while start.elapsed() < duration {}
}

#[cfg(target_arch = "wasm32")]
pub fn sleep(duration: Duration) {
    use crate::time::Instant;

    let start = Instant::now();
    while start.elapsed() < duration {}
}
