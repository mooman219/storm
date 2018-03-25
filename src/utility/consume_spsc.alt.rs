use std::cell::Cell;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::*;

// ////////////////////////////////////////////////////////
// Internal
// ////////////////////////////////////////////////////////

/// The internal memory buffer used by the replace spsc. If a read occurs during a write, None is
/// returned instead. If a write occurs during a read, the write waits for the read to complete
/// and fresh data is returned by the read. The worst case is a read always occuring during a
/// write, meaning None is always returned.
struct Buffer<T: Copy> {
    buffer: Cell<T>,
    is_empty: AtomicBool,
    is_reading: AtomicBool,
}

unsafe impl<T: Copy + Sync> Sync for Buffer<T> {}

impl<T: Copy> Buffer<T> {
    #[inline]
    pub fn read(&self) -> Option<T> {
        self.is_reading.store(true, Ordering::Release);
        let mut result = None;
        if !self.is_empty.load(Ordering::Acquire) {
            result = Some(self.buffer.get());
            self.is_empty.store(true, Ordering::Release);
        }
        self.is_reading.store(false, Ordering::Release);
        result
    }

    #[inline]
    pub fn write(&self, value: T) {
        self.is_empty.store(true, Ordering::Release);
        while self.is_reading.load(Ordering::Acquire) {}
        self.buffer.set(value);
        self.is_empty.store(false, Ordering::Release);
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
    /// use storm::utility::consume_spsc::*;
    ///
    /// let (producer, _) = make();
    ///
    /// producer.set(123);
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
    /// use storm::utility::consume_spsc::*;
    ///
    /// let (_, consumer) = make();
    ///
    /// // Attempt to pop a value from the buffer.
    /// let t: Option<u32> = consumer.consume();
    /// match t {
    ///     Some(v) => {},      // Successfully popped a value
    ///     None => {}          // Buffer empty, try again later
    /// }
    /// ```
    pub fn consume(&self) -> Option<T> {
        (*self.buffer).read()
    }
}
