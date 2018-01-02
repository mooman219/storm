use rand;
use std::sync::atomic::*;
use std::ptr::*;

//
// RefKey
//

pub struct RefKey {
    lock: u32,
}

unsafe impl Send for RefKey {}
impl !Sync for RefKey {}

//
// RefTokenFactory
//

#[derive(Copy, Clone)]
pub struct RefTokenFactory {
    lock: u32,
}

unsafe impl Send for RefTokenFactory {}
unsafe impl Sync for RefTokenFactory {}

impl RefTokenFactory {
    pub fn new() -> (RefKey, RefTokenFactory) {
        let lock = rand::random::<u32>();
        let key = RefKey { lock: lock };
        let factory = RefTokenFactory { lock: lock };
        (key, factory)
    }

    pub fn create_token<T: Copy>(&self, value: T) -> RefToken<T> {
        RefToken::new(value, self.lock)
    }
}

//
// RefToken
//

pub struct RefToken<T: Copy> {
    lock: u32,
    pointer: *mut T,
    counter: Shared<AtomicUsize>,
}

unsafe impl<T: Copy> Send for RefToken<T> {}
impl<T: Copy> !Sync for RefToken<T> {}

impl<T: Copy> RefToken<T> {
    fn new(value: T, lock: u32) -> RefToken<T> {
        let value_pointer = Box::into_raw(Box::new(value));
        let counter_pointer = Box::into_raw(Box::new(AtomicUsize::new(1)));
        RefToken {
            lock: lock,
            pointer: value_pointer,
            counter: Shared::new(counter_pointer).expect("Unable to allocate reference counter."),
        }
    }

    pub fn get(&self, key: &RefKey) -> T {
        self.validate(key);
        unsafe { *self.pointer }
    }

    pub fn set(&mut self, key: &RefKey, value: T) {
        self.validate(key);
        unsafe {
            *self.pointer = value;
        }
    }

    #[inline]
    fn validate(&self, key: &RefKey) {
        // Ensure the key we're given matches the lock we hold.
        if key.lock != self.lock {
            panic!("Invalid key to unlock token.");
        }
    }
}

impl<T: Copy> Clone for RefToken<T> {
    fn clone(&self) -> RefToken<T> {
        RefToken {
            lock: self.lock,
            pointer: self.pointer,
            counter: self.counter,
        }
    }
}
