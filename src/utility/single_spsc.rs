use std::cell::Cell;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::*;

/// The internal memory buffer used by the single spsc.
struct Buffer<T: Copy> {
    buffer: Cell<T>,
    is_empty: AtomicBool,
    is_reading: AtomicBool,
}

unsafe impl<T: Copy + Sync> Sync for Buffer<T> {}

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

impl<T: Copy> Buffer<T> {
    pub fn try_pop(&self) -> Option<T> {
        self.is_reading.store(true, Ordering::Release);
        let result = if !self.is_empty.load(Ordering::Acquire) {
            self.is_empty.store(true, Ordering::Release);
            Some(self.buffer.get())
        } else {
            None
        };
        self.is_reading.store(false, Ordering::Release);
        result
    }

    pub fn push(&self, value: T) {
        self.is_empty.store(true, Ordering::Release);
        while self.is_reading.load(Ordering::Acquire) {
            // Implementation note: pop and try_push cannot be implemented due to the potential
            // for deadlock. A call to push and pop for example may get into a state where push is
            // waiting for is_reading to be false and pop is waiting for is_empty to be false.
        }
        self.buffer.set(value);
        self.is_empty.store(false, Ordering::Release);
    }
}

pub fn make<T: Copy>() -> (Producer<T>, Consumer<T>) {
    // This is the only place where a buffer can be created.
    let arc = Arc::new(Buffer {
        // The initial state is EMPTY, so the uninitialized memory will never be served.
        buffer: unsafe { Cell::new(mem::uninitialized()) },
        // Mark the buffer as empty.
        is_empty: AtomicBool::new(true),
        // Mark the buffer was not reading.
        is_reading: AtomicBool::new(false),
    });
    (Producer { buffer: arc.clone() }, Consumer { buffer: arc.clone() })
}

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
    /// let (producer, _) = make();
    ///
    /// producer.push(123);
    /// ```
    pub fn push(&self, value: T) {
        (*self.buffer).push(value);
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
    /// use single_spsc::*;
    ///
    /// let (_, consumer) = make();
    ///
    /// // Attempt to pop a value from the buffer.
    /// let t = consumer.try_pop();
    /// match t {
    ///     Some(v) => {},      // Successfully popped a value
    ///     None => {}          // Buffer empty, try again later
    /// }
    /// ```
    pub fn try_pop(&self) -> Option<T> {
        (*self.buffer).try_pop()
    }
}
