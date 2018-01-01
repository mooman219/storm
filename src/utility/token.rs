use rand;
use std::sync::atomic;

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
        let value_pointer = Box::into_raw(Box::new(value));
        let token = RefToken {
            lock: self.lock,
            pointer: value_pointer,
        };
        token
    }
}

//
// RefToken
//

pub struct RefToken<T: Copy> {
    lock: u32,
    pointer: *mut T,
}

impl<T: Copy> RefToken<T> {
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
