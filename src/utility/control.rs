use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

pub struct Watcher {
    control: Weak<AtomicBool>,
}

impl Watcher {
    pub fn alive(&self) -> bool {
        match self.control.upgrade() {
            Some(_) => true,
            None => false,
        }
    }
}

pub struct Probe {
    control: Arc<AtomicBool>,
}

impl Probe {
    pub fn finalize(self) {
        (*self.control).store(false, Ordering::Relaxed);
    }
}

pub fn make_probe() -> (Watcher, Probe) {
    let source = Arc::new(AtomicBool::new(true));
    let flag = Arc::downgrade(&source);
    (
        Watcher {
            control: flag,
        },
        Probe {
            control: source,
        },
    )
}
