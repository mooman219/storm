use gl;
use render::buffer::*;
use render::enums::*;
use std::mem;
use std::ptr;
use time::timer::*;

pub struct FixedBuffer<T> {
    vbo: u32,
    dirty: bool,
    buffer_type: BufferType,
    capacity: usize,
    items: Vec<T>,
    timer_sync: Timer,
}

impl<T> FixedBuffer<T> {
    pub fn new(buffer_type: BufferType, capacity: usize) -> FixedBuffer<T> {
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
            buffer_type: buffer_type,
            capacity: capacity,
            items: items,
            timer_sync: Timer::new("Render: Fixed Sync"),
        }
    }
}

impl<T> RawBuffer<T> for FixedBuffer<T> {
    fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
        if index == self.capacity {
            panic!("Attempting to add element to full buffer.");
        }
        self.items.push(item);
        self.dirty = true;
        index
    }

    fn remove(&mut self, index: usize) {
        self.items.swap_remove(index);
        self.dirty = true;
    }

    fn update(&mut self, index: usize, item: T) {
        self.items[index] = item;
        self.dirty = true;
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
            self.timer_sync.start();
            self.dirty = false;
            unsafe {
                let size = (self.items.len() * mem::size_of::<T>()) as isize;
                let data = self.items.as_ptr() as *const _;
                gl::BindBuffer(self.buffer_type as u32, self.vbo);
                gl::BufferSubData(
                    self.buffer_type as u32, // Buffer type
                    0,                       // Offset
                    size,                    // Size
                    data,                    // Data
                );
            }
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
