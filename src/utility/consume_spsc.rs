use std::cell::Cell;
use std::mem;
use std::sync::atomic::*;
use std::sync::Arc;

// ////////////////////////////////////////////////////////
// Internal
// ////////////////////////////////////////////////////////

const BUFFER_SIZE: usize = 32;

/// The internal memory buffer used by the replace spsc. It's unlikely, but during a read, a write
/// could happen inbetween the atomic load and the dereference. This is unlikely because BUFFER_SIZE
/// writes would have to happen during that time. If a write it timed properly, there's a chance
/// that the written value could be returned at most twice.
struct Buffer<T: Copy> {
    is_empty: AtomicBool,
    read: AtomicPtr<T>,
    current: Cell<usize>,
    buffer: [T; BUFFER_SIZE],
}

impl<T: Copy> Buffer<T> {
    fn new() -> Buffer<T> {
        let this = Buffer {
            is_empty: AtomicBool::new(true),
            read: AtomicPtr::new(0 as *mut T),
            current: Cell::new(0),
            buffer: unsafe { mem::uninitialized() },
        };
        this
    }

    pub fn read(&self) -> Option<T> {
        // It's unlikely, but a write could happen inbetween the atomic load and the dereference.
        // This is unlikely because BUFFER_SIZE writes would have to happen during that time.
        if self.is_empty.load(Ordering::Acquire) {
            None
        } else {
            self.is_empty.store(true, Ordering::Release);
            // There's a chance a write could happen here, meaning read will return the same value
            // twice because the is_empty flag would not be set false. If we set the flag later,
            // there's a chance we miss the write which is worse and returning it twice.
            Some(unsafe { *self.read.load(Ordering::Acquire) })
        }
    }

    pub fn write(&self, value: T) {
        let x = self.current.get();
        let pointer = self.buffer.as_ptr().wrapping_add(x) as *mut T;
        unsafe {
            pointer.write(value);
        }
        self.read.store(pointer, Ordering::Release);
        self.is_empty.store(false, Ordering::Release);
        self.current.set((x + 1) & (BUFFER_SIZE - 1));
    }
}

pub fn make<T: Copy>() -> (Producer<T>, Consumer<T>) {
    // This is the only place where a buffer is created.
    let arc = Arc::new(Buffer::new());
    (
        Producer {
            buffer: arc.clone(),
        },
        Consumer {
            buffer: arc.clone(),
        },
    )
}

// ////////////////////////////////////////////////////////
// Public
// ////////////////////////////////////////////////////////

/// A handle which allows adding values onto the buffer.
pub struct Producer<T: Copy> {
    buffer: Arc<Buffer<T>>,
}

/// A handle which allows consuming values from the buffer.
pub struct Consumer<T: Copy> {
    buffer: Arc<Buffer<T>>,
}

unsafe impl<T: Copy + Send> Send for Producer<T> {}
unsafe impl<T: Copy + Send> Send for Consumer<T> {}

impl<T: Copy> Producer<T> {
    /// Push a value onto the buffer.
    ///
    /// If the buffer is non-full, the operation will execute immediately. If the buffer is
    /// populated, this operation overwrites the stored value. If the buffer is contested by a
    /// read from the consumer, it will spin until the read is finished.
    pub fn set(&self, value: T) {
        (*self.buffer).write(value);
    }
}

impl<T: Copy> Consumer<T> {
    /// Attempt to pop a value from the buffer.
    ///
    /// This method does not block.  If the buffer is empty, the method will return `None`. If
    /// there is a value available, the method will return `Some(v)`, where `v` is the value being
    /// consumed from the buffer.
    pub fn consume(&self) -> Option<T> {
        (*self.buffer).read()
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
    fn cycle() {
        let (producer, consumer) = make();

        assert_eq!(consumer.consume(), None);

        producer.set(0u32);
        assert_eq!(consumer.consume(), Some(0u32));
        assert_eq!(consumer.consume(), None);

        producer.set(0u32);
        producer.set(1u32);
        assert_eq!(consumer.consume(), Some(1u32));
        assert_eq!(consumer.consume(), None);
    }

    // ////////////////////////////////////////////////////////////////////////////
    // Benches
    // ////////////////////////////////////////////////////////////////////////////

    const ITERATIONS: usize = 1000;

    #[bench]
    fn bench_cycle(bench: &mut Bencher) {
        let (p, c) = make();

        bench.iter(|| {
            for x in 0..ITERATIONS {
                black_box(p.set(x));
                black_box(c.consume());
            }
        });
    }
}
