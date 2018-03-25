use std::cell::Cell;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::*;

// ////////////////////////////////////////////////////////
// Internal
// ////////////////////////////////////////////////////////

/// The internal memory buffer used by the replace spsc. If a read occurs during a write, stale
/// data is returned instead. If a write occurs during a read, the write waits for the read to
/// complete and fresh data is returned by the read. The worst case is a read always occuring
/// during a write, meaning stale data is always returned.
struct Buffer<T: Copy> {
    buffer_read: Cell<T>,
    buffer: Cell<T>,
    is_reading: AtomicBool,
    is_writing: AtomicBool,
}

unsafe impl<T: Copy + Sync> Sync for Buffer<T> {}

impl<T: Copy> Buffer<T> {
    #[inline]
    pub fn read(&self) -> T {
        self.is_reading.store(true, Ordering::Release);
        if self.is_writing.load(Ordering::Acquire) {
            self.is_reading.store(false, Ordering::Release);
            self.buffer_read.get()
        } else {
            let result = self.buffer.get();
            self.is_reading.store(false, Ordering::Release);
            self.buffer_read.set(result);
            result
        }
    }

    #[inline]
    pub fn write(&self, value: T) {
        self.is_writing.store(true, Ordering::Release);
        while self.is_reading.load(Ordering::Acquire) {}
        self.buffer.set(value);
        self.is_writing.store(false, Ordering::Release);
    }
}

pub fn make<T: Copy>(initial: T) -> (Producer<T>, Consumer<T>) {
    // This is the only place where a buffer can be created.
    let arc = Arc::new(Buffer {
        buffer_read: Cell::new(initial),
        buffer: unsafe { Cell::new(mem::uninitialized()) },
        is_reading: AtomicBool::new(false),
        is_writing: AtomicBool::new(true),
    });
    (Producer { buffer: arc.clone() }, Consumer { buffer: arc.clone() })
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

impl<T> !Sync for Producer<T> {}
impl<T> !Sync for Consumer<T> {}

impl<T: Copy> Producer<T> {
    /// Push a value onto the buffer.
    ///
    /// If the buffer is non-full, the operation will execute immediately. If the buffer is
    /// populated, this operation overwrites the stored value. If the buffer is contested by a
    /// read from the consumer, it will spin until the read is finished.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate storm;
    /// use storm::utility::replace_spsc::*;
    ///
    /// let (producer, _) = make(0u32);
    ///
    /// producer.set(1u32);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate storm;
    /// use storm::utility::replace_spsc::*;
    ///
    /// let (_, consumer) = make(1u32);
    ///
    /// let t = consumer.get();
    /// ```
    pub fn get(&self) -> T {
        (*self.buffer).read()
    }
}
