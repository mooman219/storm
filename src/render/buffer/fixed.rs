use gl;
use render::buffer::*;
use render::enums::*;
use std::cmp;
use std::mem;
use std::ptr;
use time::timer::*;

pub struct FixedBuffer<T> {
    vbo: u32,
    dirty: bool,
    dirty_min: usize,
    dirty_max: usize,
    buffer_type: BufferType,
    capacity: usize,
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
    fn new(buffer_type: BufferType, capacity: usize) -> FixedBuffer<T> {
        // Validate input
        if capacity == 0 {
            panic!("Capacity must be greater than 0.");
        }
        // Prepare data
        let items = Vec::with_capacity(capacity);
        let mut vbo = 0u32;
        // Call into opengl
        unsafe {
            let max_size = (capacity * mem::size_of::<T>()) as isize;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type as u32, vbo);
            gl::BufferData(
                buffer_type as u32, // Buffer type
                max_size,           // Size
                ptr::null(),        // Initial data
                gl::DYNAMIC_DRAW,   // Usage
            );
        }
        // Finish
        FixedBuffer {
            vbo: vbo,
            dirty: false,
            dirty_min: 0,
            dirty_max: 0,
            buffer_type: buffer_type,
            capacity: capacity,
            items: items,
            timer_sync: Timer::new("[R] Fixed Sync"),
        }
    }

    fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
        if index == self.capacity {
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

    fn offset_index(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type as u32, self.vbo);
        }
    }

    fn sync(&mut self) {
        if self.dirty {
            // Timing start.
            self.timer_sync.start();
            // Sync state.
            self.dirty = false;
            unsafe {
                let offset = (mem::size_of::<T>() * self.dirty_min) as isize;
                let size = (mem::size_of::<T>() * (self.dirty_max - self.dirty_min)) as isize;
                let data = self.items.as_ptr().wrapping_add(self.dirty_min) as *const _;
                gl::BindBuffer(self.buffer_type as u32, self.vbo);
                gl::BufferSubData(
                    self.buffer_type as u32, // Buffer type
                    offset,                  // Offset
                    size,                    // Size
                    data,                    // Data
                );
            }
            // Timing finish.
            self.timer_sync.stop();
        }
    }
}

impl<T> Drop for FixedBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo as *const _);
        }
    }
}
