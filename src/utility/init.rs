use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::atomic::*;

//
// LazyStatic
//

/// Stores T on the heap.
pub struct LazyStatic<T: Sync> {
    store: UnsafeCell<*const T>,
    initializer: fn() -> T,
    is_pending: AtomicBool,
    is_ready: AtomicBool,
}

unsafe impl<T: Sync> Sync for LazyStatic<T> {}

impl<T: Sync> LazyStatic<T> {
    pub const fn new(initializer: fn() -> T) -> LazyStatic<T> {
        LazyStatic {
            store: UnsafeCell::new(0 as *const _),
            initializer: initializer,
            is_pending: AtomicBool::new(false),
            is_ready: AtomicBool::new(false),
        }
    }

    fn init(&self) {
        if self.is_ready.load(Ordering::Relaxed) {
            return;
        }
        let value = (self.initializer)();
        let pointer = self.store.get();
        unsafe {
            *pointer = Box::into_raw(Box::new(value));
        }
        self.is_ready.store(true, Ordering::Relaxed);
    }
}

impl<T: Sync> Deref for LazyStatic<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: Safe init code
        self.init();
        unsafe { &**self.store.get() }
    }
}
