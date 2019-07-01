use std::cell::Cell;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const CACHELINE_LEN: usize = 64; // Usually 64 words
const CACHELINE: usize = CACHELINE_LEN / std::mem::size_of::<usize>();

#[repr(C)]
pub struct Buffer<T> {
    // Consumer cacheline:
    head: AtomicUsize,        // 1 word
    shadow_tail: Cell<usize>, // 1 word
    _pad1: [usize; CACHELINE - 2],
    // Producer cacheline:
    tail: AtomicUsize,        // 1 word
    shadow_head: Cell<usize>, // 1 word
    _pad2: [usize; CACHELINE - 2],
    // Shared cacheline:
    buffer: Box<[UnsafeCell<T>]>, // 2 words
    capacity: usize,              // 1 word
    allocated_size: usize,        // 1 word
    _pad3: [usize; CACHELINE - 4],
}

unsafe impl<T: Sync> Sync for Buffer<T> {}

pub struct Consumer<T> {
    buffer: Arc<Buffer<T>>,
    head_pointer: *mut T,
}

pub struct Producer<T> {
    buffer: Arc<Buffer<T>>,
    tail_pointer: *mut T,
}

unsafe impl<T: Send> Send for Consumer<T> {}
unsafe impl<T: Send> Send for Producer<T> {}

impl<T> Buffer<T> {
    // Shared

    pub fn size(&self) -> usize {
        self.tail.load(Ordering::Acquire) - self.head.load(Ordering::Acquire) - 1
    }

    pub fn capacity(&self) -> usize {
        self.capacity - 2
    }

    // Consumer functions

    pub fn try_next_head(&self) -> Option<*mut T> {
        let next_head = self.head.load(Ordering::Relaxed) + 1;
        if next_head == self.shadow_tail.get() {
            self.shadow_tail.set(self.tail.load(Ordering::Acquire));
            if next_head == self.shadow_tail.get() {
                return None;
            }
        }
        self.head.store(next_head, Ordering::Release);
        Some(self.buffer[next_head & (self.allocated_size - 1)].get())
    }

    // Producer functions

    pub fn try_next_tail(&self) -> Option<*mut T> {
        let next_tail = self.tail.load(Ordering::Relaxed) + 1;
        if self.shadow_head.get() + self.capacity <= next_tail {
            self.shadow_head.set(self.head.load(Ordering::Relaxed));
            if self.shadow_head.get() + self.capacity <= next_tail {
                return None;
            }
        }
        self.tail.store(next_tail, Ordering::Release);
        Some(self.buffer[next_tail & (self.allocated_size - 1)].get())
    }
}

pub fn make<T: Default>(capacity: usize) -> (Producer<T>, Consumer<T>) {
    let adjusted_capacity = capacity + 2;
    let allocated_size = adjusted_capacity.next_power_of_two();

    let mut vec = Vec::with_capacity(allocated_size);
    for _ in 0..allocated_size {
        vec.push(UnsafeCell::new(T::default()));
    }
    let buffer = vec.into_boxed_slice();
    let head_pointer = buffer[0].get();
    let tail_pointer = buffer[1].get();

    let arc = Arc::new(Buffer {
        // Consumer:
        head: AtomicUsize::new(0), // 1 word
        shadow_tail: Cell::new(1), // 1 word
        _pad1: [0; CACHELINE - 2],
        // Producer:
        tail: AtomicUsize::new(1), // 1 word
        shadow_head: Cell::new(0), // 1 word
        _pad2: [0; CACHELINE - 2],
        // Shared
        buffer,                      // 2 words
        capacity: adjusted_capacity, // 1 word
        allocated_size,              // 1 word
        _pad3: [0; CACHELINE - 4],
    });

    (
        Producer {
            buffer: arc.clone(),
            tail_pointer,
        },
        Consumer {
            buffer: arc.clone(),
            head_pointer,
        },
    )
}

impl<T> Consumer<T> {
    #[inline]
    pub fn get(&mut self) -> &mut T {
        unsafe { &mut *self.head_pointer }
    }

    pub fn try_next(&mut self) -> bool {
        match (*self.buffer).try_next_head() {
            Some(p) => {
                self.head_pointer = p;
                true
            },
            None => false,
        }
    }

    pub fn capacity(&self) -> usize {
        (*self.buffer).capacity()
    }

    pub fn size(&self) -> usize {
        (*self.buffer).size()
    }
}

impl<T> Producer<T> {
    #[inline]
    pub fn get(&mut self) -> &mut T {
        unsafe { &mut *self.tail_pointer }
    }

    pub fn try_next(&mut self) -> bool {
        match (*self.buffer).try_next_tail() {
            Some(p) => {
                self.tail_pointer = p;
                true
            },
            None => false,
        }
    }

    pub fn capacity(&self) -> usize {
        (*self.buffer).capacity()
    }

    pub fn size(&self) -> usize {
        (*self.buffer).size()
    }
}

// ////////////////////////////////////////////////////////////////////////////
// Tests
// ////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use test::black_box;
    use test::Bencher;

    #[test]
    fn buffer_size() {
        assert_eq!(::std::mem::size_of::<Buffer<()>>(), 3 * CACHELINE_LEN);
    }

    #[test]
    fn producer_next() {
        let (mut p, _) = super::make::<usize>(5);
        assert!(p.try_next());
        assert!(p.capacity() == 5);
        assert!(p.size() == 1);
    }

    #[test]
    fn producer_next_full() {
        let (mut p, _) = super::make::<usize>(1);
        assert!(p.try_next());
        assert!(!p.try_next());
        assert!(p.capacity() == 1);
        assert!(p.size() == 1);
    }

    #[test]
    fn consumer_next_empty() {
        let (_, mut c) = super::make::<usize>(1);
        assert!(!c.try_next());
        assert!(c.capacity() == 1);
        assert!(c.size() == 0);
    }

    #[test]
    fn consumer_next() {
        let (mut p, mut c) = super::make::<usize>(1);
        assert!(p.try_next());
        assert!(c.try_next());
        assert!(c.capacity() == 1);
        assert!(c.size() == 0);
    }

    #[test]
    fn wrapping() {
        let (mut p, mut c) = super::make::<usize>(1);
        for _ in 0..10 {
            assert!(p.try_next());
            assert!(c.try_next());
            assert!(c.capacity() == 1);
            assert!(c.size() == 0);
        }
    }

    // ////////////////////////////////////////////////////////////////////////////
    // Benches
    // ////////////////////////////////////////////////////////////////////////////

    const ITERATIONS: usize = 1000;

    #[bench]
    fn bench_cycle(bench: &mut Bencher) {
        let (mut p, mut c) = super::make::<usize>(10);

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                black_box(p.try_next());
                black_box(c.try_next());
            }
        });
    }
}
