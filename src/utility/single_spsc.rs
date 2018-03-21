use std::cell::Cell;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::*;

struct Buffer<T: Copy> {
    buffer: Cell<T>,
    is_empty: AtomicBool,
    is_reading: AtomicBool,
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
    pub fn consume(&self) -> Option<T> {
        self.is_reading.store(true, Ordering::Relaxed);
        let result = if !self.is_empty.load(Ordering::Relaxed) {
            let result = Some(self.buffer.get());
            self.is_empty.store(true, Ordering::Relaxed);
            result
        } else {
            None
        };
        self.is_reading.store(false, Ordering::Relaxed);
        result
    }

    pub fn set(&self, value: T) {
        self.is_empty.store(true, Ordering::Relaxed);
        while self.is_reading.load(Ordering::Relaxed) {}
        self.buffer.set(value);
        self.is_empty.store(false, Ordering::Relaxed);
    }
}

pub fn make<T: Copy>() -> (Producer<T>, Consumer<T>) {
    // This is the only place where a buffer can be created.
    let arc = Arc::new(Buffer {
        buffer: unsafe { Cell::new(mem::uninitialized()) },
        is_empty: AtomicBool::new(true),
        is_reading: AtomicBool::new(false),
    });
    (Producer { buffer: arc.clone() }, Consumer { buffer: arc.clone() })
}

impl<T: Copy> Producer<T> {
    pub fn push(&self, value: T) {
        (*self.buffer).set(value);
    }
}

impl<T: Copy> Consumer<T> {
    pub fn pop(&self) -> Option<T> {
        (*self.buffer).consume()
    }
}
