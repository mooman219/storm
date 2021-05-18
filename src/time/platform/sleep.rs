use core::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}

#[cfg(target_arch = "wasm32")]
pub fn sleep(duration: Duration) {}
