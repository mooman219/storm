use core::mem;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Arc;

const CACHELINE_LEN: usize = 64; // Usually 64 words
const CACHELINE: usize = CACHELINE_LEN / std::mem::size_of::<usize>();

#[repr(C)]
pub struct Buffer<T> {
    shared: AtomicPtr<T>,
    populated: AtomicBool,
}

unsafe impl<T: Sync> Sync for Buffer<T> {}

pub struct Consumer<T> {
    buffer: Arc<Buffer<T>>,
    pointer: *mut T,
}

pub struct Producer<T> {
    buffer: Arc<Buffer<T>>,
    pointer: *mut T,
}

impl<T> Drop for Consumer<T> {
    fn drop(&mut self) {
        unsafe {
            mem::drop(Box::from_raw(self.pointer));
        }
    }
}

impl<T> Drop for Producer<T> {
    fn drop(&mut self) {
        unsafe {
            mem::drop(Box::from_raw(self.pointer));
        }
    }
}

unsafe impl<T: Send> Send for Consumer<T> {}
unsafe impl<T: Send> Send for Producer<T> {}

impl<T> Buffer<T> {
    // Consumer functions

    pub fn try_swap_consumer(&self, consumer: *mut T) -> Option<*mut T> {
        if self.populated.load(Ordering::Acquire) {
            let pointer = self.shared.swap(consumer, Ordering::Relaxed);
            self.populated.store(false, Ordering::Release);
            Some(pointer)
        } else {
            None
        }
    }

    // Producer functions

    pub fn try_swap_producer(&self, producer: *mut T) -> Option<*mut T> {
        if !self.populated.load(Ordering::Acquire) {
            let pointer = self.shared.swap(producer, Ordering::Relaxed);
            self.populated.store(true, Ordering::Release);
            Some(pointer)
        } else {
            None
        }
    }
}

pub fn make<T: Default>() -> (Producer<T>, Consumer<T>) {
    let shared_pointer = AtomicPtr::new(Box::into_raw(Box::new(T::default())));
    let consumer_pointer = Box::into_raw(Box::new(T::default()));
    let producer_pointer = Box::into_raw(Box::new(T::default()));

    let arc = Arc::new(Buffer {
        shared: shared_pointer,
        populated: AtomicBool::new(false),
    });

    (
        Producer {
            buffer: arc.clone(),
            pointer: producer_pointer,
        },
        Consumer {
            buffer: arc.clone(),
            pointer: consumer_pointer,
        },
    )
}

impl<T> Consumer<T> {
    #[inline]
    pub fn get(&mut self) -> &mut T {
        unsafe { &mut *self.pointer }
    }

    pub fn try_next(&mut self) -> bool {
        match (*self.buffer).try_swap_consumer(self.pointer) {
            Some(p) => {
                self.pointer = p;
                true
            }
            None => false,
        }
    }
}

impl<T> Producer<T> {
    #[inline]
    pub fn get(&mut self) -> &mut T {
        unsafe { &mut *self.pointer }
    }

    pub fn try_next(&mut self) -> bool {
        match (*self.buffer).try_swap_producer(self.pointer) {
            Some(p) => {
                self.pointer = p;
                true
            }
            None => false,
        }
    }
}
