use parking_lot::{Condvar, Mutex};

/// Simple Mutex + Condvar wait notify primitive. Can be used for waiting without spinning. This
/// should be wrapped as an Arc<Signal> and cloned.
#[repr(align(16))]
pub struct Signal {
    mutex: Mutex<bool>,
    cvar: Condvar,
}

impl Signal {
    /// Creates a new signal.
    pub fn new() -> Self {
        Self {
            mutex: Mutex::new(false),
            cvar: Condvar::new(),
        }
    }

    /// Wakes all threads awaiting this signal.
    pub fn notify(&self) {
        let mut lock = self.mutex.lock();
        *lock = true;
        self.cvar.notify_all();
    }

    /// Parks the current thread until notified. May spuriously wake on its own.
    pub fn wait(&self) {
        let mut lock = self.mutex.lock();
        if *lock {
            *lock = false;
            return;
        }
        self.cvar.wait(&mut lock);
    }
}
