use rand;
use std::sync::atomic;

pub struct RefKey {
    lock: u32,
}

#[derive(Copy, Clone)]
pub struct RefTokenFactory {
    lock: u32,
}

impl RefTokenFactory {
    pub fn new() -> (RefKey, RefTokenFactory) {
        let lock = rand::random::<u32>();
        let key = RefKey { lock: lock };
        let factory = RefTokenFactory { lock: lock };
        (key, factory)
    }

    pub fn create_token<T: Copy>(&self, value: T) -> RefToken<T> {
        let value_pointer = Box::into_raw(Box::new(value));
        let token = RefToken {
            lock: self.lock,
            pointer: value_pointer,
        };
        token
    }
}

#[derive(Clone)]
pub struct RefToken<T: Copy> {
    lock: u32,
    pointer: *mut T,
}

impl<T: Copy> RefToken<T> {
    pub fn get(&self, key: RefKey) -> T {
        if key.lock != self.lock {
            panic!("Invalid key to unlock token.");
        }
        unsafe { *self.pointer }
    }

    pub fn set(&mut self, key: RefKey, value: T) {
        if key.lock != self.lock {
            panic!("Invalid key to unlock token.");
        }
        unsafe {
            *self.pointer = value;
        }
    }
}
