// This file is licensed under the Apache License Version 2.0, and is a modified version of
// https://github.com/polyfractal/bounded-spsc-queue. A copy of that license can be found here: in
// the path licenses/polyfractal/bounded-spsc-queue.
//
// This file was modified to use the new allocator API.

use ::alloc::{alloc, sync::Arc};
use core::{
    alloc::Layout,
    cell::Cell,
    mem, ptr,
    sync::atomic::{AtomicUsize, Ordering},
    usize,
};

const CACHELINE_LEN: usize = 64;
const CACHELINE: usize = CACHELINE_LEN / std::mem::size_of::<usize>();

/// The internal memory buffer used by the queue.
///
/// Buffer holds a pointer to allocated memory which represents the bounded
/// ring buffer, as well as a head and tail atomicUsize which the producer and consumer
/// use to track location in the ring.
#[repr(C)]
pub struct Buffer<T> {
    /// A pointer to the allocated ring buffer
    buffer: *mut T,

    /// The bounded size as specified by the user.  If the queue reaches capacity, it will block
    /// until values are poppped off.
    capacity: usize,

    /// The allocated size of the ring buffer, in terms of number of values (not physical memory).
    /// This will be the next power of two larger than `capacity`
    allocated_size: usize,
    _padding1: [usize; CACHELINE - 3],

    /// Consumer cacheline:

    /// Index position of the current head
    head: AtomicUsize,
    shadow_tail: Cell<usize>,
    _padding2: [usize; CACHELINE - 2],

    /// Producer cacheline:

    /// Index position of current tail
    tail: AtomicUsize,
    shadow_head: Cell<usize>,
    _padding3: [usize; CACHELINE - 2],
}

/// A handle to the queue which allows consuming values from the buffer
pub struct Consumer<T> {
    buffer: Arc<Buffer<T>>,
}

/// A handle to the queue which allows adding values onto the buffer
pub struct Producer<T> {
    buffer: Arc<Buffer<T>>,
}

unsafe impl<T: Send> Send for Consumer<T> {}
unsafe impl<T: Send> Send for Producer<T> {}

impl<T> Buffer<T> {
    /// Attempt to pop a value off the buffer.
    ///
    /// If the buffer is empty, this method will not block.  Instead, it will return `None`
    /// signifying the buffer was empty.  The caller may then decide what to do next (e.g.
    /// spin-wait, sleep, process something else, etc)
    pub fn try_pop(&self) -> Option<T> {
        let current_head = self.head.load(Ordering::Relaxed);

        if current_head == self.shadow_tail.get() {
            self.shadow_tail.set(self.tail.load(Ordering::Acquire));
            if current_head == self.shadow_tail.get() {
                return None;
            }
        }

        let v = unsafe { ptr::read(self.load(current_head)) };
        self.head.store(current_head.wrapping_add(1), Ordering::Release);
        Some(v)
    }

    /// Attempts to pop (and discard) at most `n` values off the buffer.
    ///
    /// Returns the amount of values successfully skipped.
    ///
    /// # Safety
    ///
    /// *WARNING:* This will leak at most `n` values from the buffer, i.e. the destructors of the
    /// objects skipped over will not be called. This function is intended to be used on buffers
    /// that contain non-`Drop` data, such as a `Buffer<f32>`.
    pub fn skip_n(&self, n: usize) -> usize {
        let current_head = self.head.load(Ordering::Relaxed);

        self.shadow_tail.set(self.tail.load(Ordering::Acquire));
        if current_head == self.shadow_tail.get() {
            return 0;
        }
        let mut diff = self.shadow_tail.get().wrapping_sub(current_head);
        if diff > n {
            diff = n
        }
        self.head.store(current_head.wrapping_add(diff), Ordering::Release);
        diff
    }

    /// Pop a value off the buffer.
    ///
    /// This method will block until the buffer is non-empty.  The waiting strategy is a simple
    /// spin-wait and will repeatedly call `try_pop()` until a value is available.  If you do not
    /// want a spin-wait burning CPU, you should call `try_pop()` directly and implement a different
    /// waiting strategy.
    pub fn pop(&self) -> T {
        loop {
            match self.try_pop() {
                None => {}
                Some(v) => return v,
            }
        }
    }

    /// Attempt to push a value onto the buffer.
    ///
    /// If the buffer is full, this method will not block.  Instead, it will return `Some(v)`, where
    /// `v` was the value attempting to be pushed onto the buffer.  If the value was successfully
    /// pushed onto the buffer, `None` will be returned signifying success.
    pub fn try_push(&self, v: T) -> Option<T> {
        let current_tail = self.tail.load(Ordering::Relaxed);

        if self.shadow_head.get() + self.capacity <= current_tail {
            self.shadow_head.set(self.head.load(Ordering::Relaxed));
            if self.shadow_head.get() + self.capacity <= current_tail {
                return Some(v);
            }
        }

        unsafe {
            self.store(current_tail, v);
        }
        self.tail.store(current_tail.wrapping_add(1), Ordering::Release);
        None
    }

    /// Push a value onto the buffer.
    ///
    /// This method will block until the buffer is non-full.  The waiting strategy is a simple
    /// spin-wait and will repeatedly call `try_push()` until the value can be added.  If you do not
    /// want a spin-wait burning CPU, you should call `try_push()` directly and implement a
    /// different waiting strategy.
    pub fn push(&self, v: T) {
        let mut t = v;
        loop {
            match self.try_push(t) {
                Some(rv) => t = rv,
                None => return,
            }
        }
    }

    /// Load a value out of the buffer
    ///
    /// # Safety
    ///
    /// This method assumes the caller has:
    /// - Initialized a valid block of memory
    /// - Specified an index position that contains valid data
    ///
    /// The caller can use either absolute or monotonically increasing index positions, since
    /// buffer wrapping is handled inside the method.
    #[inline]
    unsafe fn load(&self, pos: usize) -> &T {
        &*self.buffer.offset((pos & (self.allocated_size - 1)) as isize)
    }

    /// Store a value in the buffer
    ///
    /// # Safety
    ///
    /// This method assumes the caller has:
    /// - Initialized a valid block of memory
    #[inline]
    unsafe fn store(&self, pos: usize, v: T) {
        let end = self.buffer.offset((pos & (self.allocated_size - 1)) as isize);
        ptr::write(&mut *end, v);
    }
}

/// Handles deallocation of heap memory when the buffer is dropped
impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        // Pop the rest of the values off the queue.  By moving them into this scope,
        // we implicitly call their destructor

        while let Some(_) = self.try_pop() {}

        unsafe {
            let layout =
                Layout::from_size_align(self.allocated_size * mem::size_of::<T>(), mem::align_of::<T>())
                    .unwrap();
            alloc::dealloc(self.buffer as *mut u8, layout);
        }
    }
}

/// Creates a new SPSC Queue, returning a Producer and Consumer handle
///
/// Capacity specifies the size of the bounded queue to create.  Actual memory usage
/// will be `capacity.next_power_of_two() * size_of::<T>()`, since ringbuffers with
/// power of two sizes are more efficient to operate on (can use a bitwise AND to index
/// into the ring instead of a more expensive modulo operator).
///
/// Of course, a SPSC queue is really only useful if you plan to use it in a multi-threaded
/// environment.  The Producer and Consumer can both be sent to a thread, providing a fast, bounded
/// one-way communication channel between those threads.
///
/// # Panics
///
/// If the requested queue size is larger than available memory (e.g.
/// `capacity.next_power_of_two() * size_of::<T>() > available memory` ), this function will abort
/// with an OOM panic.
pub fn make<T>(capacity: usize) -> (Producer<T>, Consumer<T>) {
    let ptr = unsafe { allocate_buffer(capacity) };

    let arc = Arc::new(Buffer {
        buffer: ptr,
        capacity,
        allocated_size: capacity.next_power_of_two(),
        _padding1: [0; CACHELINE - 3],

        head: AtomicUsize::new(0),
        shadow_tail: Cell::new(0),
        _padding2: [0; CACHELINE - 2],

        tail: AtomicUsize::new(0),
        shadow_head: Cell::new(0),
        _padding3: [0; CACHELINE - 2],
    });

    (
        Producer {
            buffer: arc.clone(),
        },
        Consumer {
            buffer: arc.clone(),
        },
    )
}

/// Allocates a memory buffer on the heap and returns a pointer to it
unsafe fn allocate_buffer<T>(capacity: usize) -> *mut T {
    let adjusted_size = capacity.next_power_of_two();
    let size = adjusted_size.checked_mul(mem::size_of::<T>()).expect("capacity overflow");

    let layout = Layout::from_size_align(size, mem::align_of::<T>()).unwrap();
    let ptr = alloc::alloc(layout);
    if ptr.is_null() {
        alloc::handle_alloc_error(layout)
    } else {
        ptr as *mut T
    }
}

impl<T> Producer<T> {
    /// Push a value onto the buffer.
    ///
    /// If the buffer is non-full, the operation will execute immediately.  If the buffer is full,
    /// this method will block until the buffer is non-full.  The waiting strategy is a simple
    /// spin-wait. If you do not want a spin-wait burning CPU, you should call `try_push()`
    /// directly and implement a different waiting strategy.
    #[inline]
    pub fn push(&self, v: T) {
        (*self.buffer).push(v);
    }

    /// Attempt to push a value onto the buffer.
    ///
    /// This method does not block.  If the queue is not full, the value will be added to the
    /// queue and the method will return `None`, signifying success.  If the queue is full,
    /// this method will return `Some(v)``, where `v` is your original value.
    #[inline]
    pub fn try_push(&self, v: T) -> Option<T> {
        (*self.buffer).try_push(v)
    }

    /// Returns the total capacity of this queue
    ///
    /// This value represents the total capacity of the queue when it is full.  It does not
    /// represent the current usage.  For that, call `size()`.
    pub fn capacity(&self) -> usize {
        (*self.buffer).capacity
    }

    /// Returns the current size of the queue
    ///
    /// This value represents the current size of the queue.  This value can be from 0-`capacity`
    /// inclusive.
    pub fn size(&self) -> usize {
        (*self.buffer).tail.load(Ordering::Acquire) - (*self.buffer).head.load(Ordering::Acquire)
    }

    /// Returns the available space in the queue
    ///
    /// This value represents the number of items that can be pushed onto the queue before it
    /// becomes full.
    pub fn free_space(&self) -> usize {
        self.capacity() - self.size()
    }
}

impl<T> Consumer<T> {
    /// Pop a value off the queue.
    ///
    /// If the buffer contains values, this method will execute immediately and return a value.
    /// If the buffer is empty, this method will block until a value becomes available.  The
    /// waiting strategy is a simple spin-wait. If you do not want a spin-wait burning CPU, you
    /// should call `try_push()` directly and implement a different waiting strategy.
    #[inline]
    pub fn pop(&self) -> T {
        (*self.buffer).pop()
    }

    /// Attempt to pop a value off the queue.
    ///
    /// This method does not block.  If the queue is empty, the method will return `None`.  If
    /// there is a value available, the method will return `Some(v)`, where `v` is the value
    /// being popped off the queue.
    #[inline]
    pub fn try_pop(&self) -> Option<T> {
        (*self.buffer).try_pop()
    }

    /// Attempts to pop (and discard) at most `n` values off the buffer.
    ///
    /// Returns the amount of values successfully skipped.
    ///
    /// # Safety
    ///
    /// *WARNING:* This will leak at most `n` values from the buffer, i.e. the destructors of the
    /// objects skipped over will not be called. This function is intended to be used on buffers
    /// that contain non-`Drop` data, such as a `Buffer<f32>`.
    pub fn skip_n(&self, n: usize) -> usize {
        (*self.buffer).skip_n(n)
    }
    /// Returns the total capacity of this queue
    ///
    /// This value represents the total capacity of the queue when it is full.  It does not
    /// represent the current usage.  For that, call `size()`.
    pub fn capacity(&self) -> usize {
        (*self.buffer).capacity
    }

    /// Returns the current size of the queue
    ///
    /// This value represents the current size of the queue.  This value can be from 0-`capacity`
    /// inclusive.
    pub fn size(&self) -> usize {
        (*self.buffer).tail.load(Ordering::Acquire) - (*self.buffer).head.load(Ordering::Acquire)
    }
}

// ////////////////////////////////////////////////////////////////////////////
// Tests
// ////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::{Buffer, CACHELINE_LEN};
    use std::thread;

    #[test]
    fn buffer_size() {
        assert_eq!(::std::mem::size_of::<Buffer<()>>(), 3 * CACHELINE_LEN);
    }

    #[test]
    fn producer_push() {
        let (p, _) = super::make(10);

        for i in 0..9 {
            p.push(i);
            assert!(p.capacity() == 10);
            assert!(p.size() == i + 1);
        }
    }

    #[test]
    fn consumer_pop() {
        let (p, c) = super::make(10);

        for i in 0..9 {
            p.push(i);
            assert!(p.capacity() == 10);
            assert!(p.size() == i + 1);
        }

        for i in 0..9 {
            assert!(c.size() == 9 - i);
            let t = c.pop();
            assert!(c.capacity() == 10);
            assert!(c.size() == 9 - i - 1);
            assert!(t == i);
        }
    }

    #[test]
    fn consumer_skip() {
        let (p, c) = super::make(10);

        for i in 0..9 {
            p.push(i);
            assert!(p.capacity() == 10);
            assert!(p.size() == i + 1);
        }
        assert!(c.size() == 9);
        assert!(c.skip_n(5) == 5);
        assert!(c.size() == 4);
        for i in 0..4 {
            assert!(c.size() == 4 - i);
            let t = c.pop();
            assert!(c.capacity() == 10);
            assert!(c.size() == 4 - i - 1);
            assert!(t == i + 5);
        }
        assert!(c.size() == 0);
        assert!(c.skip_n(5) == 0);
    }

    #[test]
    fn consumer_skip_whole_buf() {
        let (p, c) = super::make(9);

        for i in 0..9 {
            p.push(i);
            assert!(p.capacity() == 9);
            assert!(p.size() == i + 1);
        }
        assert!(c.size() == 9);
        assert!(c.skip_n(9) == 9);
        assert!(c.size() == 0);
    }

    #[test]
    fn try_push() {
        let (p, _) = super::make(10);

        for i in 0..10 {
            p.push(i);
            assert!(p.capacity() == 10);
            assert!(p.size() == i + 1);
        }

        match p.try_push(10) {
            Some(v) => {
                assert!(v == 10);
            }
            None => assert!(false, "Queue should not have accepted another write!"),
        }
    }

    #[test]
    fn try_poll() {
        let (p, c) = super::make(10);

        match c.try_pop() {
            Some(_) => assert!(false, "Queue was empty but a value was read!"),
            None => {}
        }

        p.push(123);

        match c.try_pop() {
            Some(v) => assert!(v == 123),
            None => assert!(false, "Queue was not empty but poll() returned nothing!"),
        }

        match c.try_pop() {
            Some(_) => assert!(false, "Queue was empty but a value was read!"),
            None => {}
        }
    }

    #[test]
    fn threaded() {
        let (p, c) = super::make(500);

        thread::spawn(move || {
            for i in 0..100000 {
                p.push(i);
            }
        });

        for i in 0..100000 {
            let t = c.pop();
            assert!(t == i);
        }
    }
}
