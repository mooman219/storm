use render::buffer::*;
use render::raw::*;
use std::cmp;
use std::mem;
use std::ptr;
use time::*;

pub struct FixedBuffer<T> {
    vbo: u32,
    dirty: bool,
    dirty_min: usize,
    dirty_max: usize,
    buffer_type: BufferBindingTarget,
    items: Vec<T>,
    timer_sync: Timer,
}

impl<T> FixedBuffer<T> {
    fn mark(&mut self, index: usize) {
        if self.dirty {
            self.dirty_min = cmp::min(self.dirty_min, index);
            self.dirty_max = cmp::max(self.dirty_max, index + 1);
        } else {
            self.dirty = true;
            self.dirty_min = index;
            self.dirty_max = index + 1;
        }
    }
}

impl<T> RawBuffer<T> for FixedBuffer<T> {
    fn new(buffer_type: BufferBindingTarget, capacity: usize) -> FixedBuffer<T> {
        // Validate input
        if capacity == 0 {
            panic!("Capacity must be greater than 0.");
        }
        // Prepare data
        let items = Vec::with_capacity(capacity);
        let max_size = (capacity * mem::size_of::<T>()) as isize;
        let vbo = gen_buffer();
        bind_buffer(buffer_type, vbo);
        buffer_data(
            buffer_type,              // Buffer type
            max_size,                 // Size
            ptr::null(),              // Initial data
            BufferUsage::DynamicDraw, // Usage
        );
        // Finish
        FixedBuffer {
            vbo: vbo,
            dirty: false,
            dirty_min: 0,
            dirty_max: 0,
            buffer_type: buffer_type,
            items: items,
            timer_sync: Timer::new("[R] Fixed Sync"),
        }
    }

    fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
        if index == self.items.capacity() {
            panic!("Attempting to add element to full buffer.");
        }
        self.items.push(item);
        self.mark(index);
        index
    }

    fn remove(&mut self, index: usize) {
        self.items.swap_remove(index);
        self.mark(index);
    }

    fn update(&mut self, index: usize, item: T) {
        self.items[index] = item;
        self.mark(index);
    }

    fn clear(&mut self) {
        let length = self.items.len();
        self.mark(0);
        self.mark(length);
        self.items.clear();
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn bind(&self) {
        bind_buffer(self.buffer_type, self.vbo);
    }

    fn sync(&mut self) {
        if self.dirty {
            // Timing start.
            self.timer_sync.start();
            // Sync state.
            self.dirty = false;
            let offset = (mem::size_of::<T>() * self.dirty_min) as isize;
            let size = (mem::size_of::<T>() * (self.dirty_max - self.dirty_min)) as isize;
            let data = self.items.as_ptr().wrapping_add(self.dirty_min) as *const _;
            bind_buffer(self.buffer_type, self.vbo);
            buffer_sub_data(
                self.buffer_type, // Buffer type
                offset,           // Offset
                size,             // Size
                data,             // Data
            );
            // Timing finish.
            self.timer_sync.stop();
        }
    }
}

impl<T> Drop for FixedBuffer<T> {
    fn drop(&mut self) {
        delete_buffer(self.vbo);
    }
}
