use std::mem;
use std::sync::Arc;
use std::sync::atomic::*;

struct Buffer<T: Copy> {
    buffer: *mut T,
    flag: AtomicUsize,
}

unsafe impl<T: Copy + Sync> Sync for Buffer<T> {}

pub struct Producer<T: Copy> {
    buffer: Arc<Buffer<T>>,
}

pub struct Consumer<T: Copy> {
    buffer: Arc<Buffer<T>>,
}

unsafe impl<T: Copy + Send> Send for Producer<T> {}
unsafe impl<T: Copy + Send> Send for Consumer<T> {}

impl<T> !Sync for Producer<T> {}
impl<T> !Sync for Consumer<T> {}

impl<T: Copy> Buffer<T> {
    pub const EMPTY: usize = 0;
    pub const FULL: usize = 1;
    pub const PENDING: usize = 2;

    pub fn load(&self) -> Option<T> {
        if self.flag
            .compare_and_swap(Self::FULL, Self::PENDING, Ordering::Acquire) == Self::FULL
        {
            let result = unsafe { Some(*self.buffer) };
            self.flag.store(Self::EMPTY, Ordering::Release);
            result
        } else {
            None
        }
    }

    pub fn store(&self, value: T) {
        // If the flag is FULL, then it updated to EMPTY. Break.
        // If the flag is EMPTY, then nothing needs to be done. Break.
        // If the flag is PENDING, then the value is in use. Spin.
        while self.flag
            .compare_and_swap(Self::FULL, Self::EMPTY, Ordering::Acquire) == Self::PENDING
        {}
        unsafe {
            *self.buffer = value;
        }
        self.flag.store(Self::FULL, Ordering::Release);
    }
}

impl<T: Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.buffer);
        }
    }
}

pub fn make<T: Copy>() -> (Producer<T>, Consumer<T>) {
    let arc = Arc::new(Buffer {
        buffer: unsafe { Box::into_raw(Box::new(mem::uninitialized())) },
        flag: AtomicUsize::new(0),
    });
    (Producer { buffer: arc.clone() }, Consumer { buffer: arc.clone() })
}

impl<T: Copy> Producer<T> {
    pub fn push(&self, value: T) {
        (*self.buffer).store(value);
    }
}

impl<T: Copy> Consumer<T> {
    pub fn pop(&self) -> Option<T> {
        (*self.buffer).load()
    }
}
