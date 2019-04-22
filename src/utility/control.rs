use parking_lot::Condvar;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Producer {
    cvar: Arc<(Mutex<bool>, Condvar)>,
}

impl Producer {
    pub fn notify(&self) {
        let &(ref lock, ref cvar) = &*self.cvar;
        let _ = lock.lock();
        cvar.notify_all();
    }
}

pub struct Consumer {
    cvar: Arc<(Mutex<bool>, Condvar)>,
}

impl Consumer {
    pub fn wait(&self) {
        let &(ref lock, ref cvar) = &*self.cvar;
        let mut started = lock.lock();
        cvar.wait(&mut started);
    }
}

impl Clone for Consumer {
    fn clone(&self) -> Self {
        Consumer {
            cvar: self.cvar.clone(),
        }
    }
}

pub fn make() -> (Producer, Consumer) {
    // This is the only place where a buffer is created.
    let arc = Arc::new((Mutex::new(false), Condvar::new()));
    (
        Producer {
            cvar: arc.clone(),
        },
        Consumer {
            cvar: arc.clone(),
        },
    )
}
