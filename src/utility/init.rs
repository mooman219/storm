use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Once;
use std::sync::atomic::*;

//
// LazyStatic
//

/// Stores T on the heap.
pub struct LazyStatic<T: Sync> {
    store: UnsafeCell<*const T>,
    initializer: fn() -> T,
    is_loading: AtomicBool,
    is_ready: AtomicBool,
}

unsafe impl<T: Sync> Sync for LazyStatic<T> {}

impl<T: Sync> LazyStatic<T> {
    pub const fn new(initializer: fn() -> T) -> LazyStatic<T> {
        LazyStatic {
            store: UnsafeCell::new(0 as *const _),
            initializer: initializer,
            is_loading: AtomicBool::new(false),
            is_ready: AtomicBool::new(false),
        }
    }

    pub fn init(&self) {
        unsafe {
            let value = (self.initializer)();
            let pointer = self.store.get();
            *pointer = Box::into_raw(Box::new(value));
        }
    }
}

impl<T: Sync> Deref for LazyStatic<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // self.once.call_once();
        unsafe { &**self.store.get() }
    }
}
