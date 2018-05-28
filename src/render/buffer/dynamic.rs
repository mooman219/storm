use gl;
use render::buffer::*;
use render::enums::*;
use std::cmp;
use std::mem;
use std::ptr;
use time::timer::*;

pub struct DynamicBuffer<T> {
    vbo: u32,
    dirty: bool,
    dirty_min: usize,
    dirty_max: usize,
    capacity: usize,
    buffer_type: BufferType,
    items: Vec<T>,
    timer_sync: Timer,
}

impl<T> DynamicBuffer<T> {
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

impl<T> RawBuffer<T> for DynamicBuffer<T> {
    fn new(buffer_type: BufferType, capacity: usize) -> DynamicBuffer<T> {
        let items: Vec<T> = Vec::<T>::with_capacity(capacity);
        let mut vbo = 0u32;
        unsafe {
            let default_size = (mem::size_of::<T>() * capacity) as isize;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type as u32, vbo);
            gl::BufferData(
                buffer_type as u32, // Buffer type
                default_size, // Size
                ptr::null(), // Initial data
                gl::DYNAMIC_DRAW, // Usage
            );
        }
        DynamicBuffer {
            vbo: vbo,
            dirty: false,
            dirty_min: 0,
            dirty_max: 0,
            capacity: capacity,
            buffer_type: buffer_type,
            items: items,
            timer_sync: Timer::new("[R] Dynamic Sync"),
        }
    }

    fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
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
                gl::BindBuffer(self.buffer_type as u32, self.vbo);
                if self.capacity < self.items.capacity() {
                    let length = (mem::size_of::<T>() * self.items.capacity()) as isize;
                    let data = self.items.as_ptr() as *const _;
                    gl::BufferData(self.buffer_type as u32, length, data, gl::DYNAMIC_DRAW);
                    self.capacity = self.items.capacity();
                } else {
                    let start = (mem::size_of::<T>() * self.dirty_min) as isize;
                    let length = (mem::size_of::<T>() * (self.dirty_max - self.dirty_min)) as isize;
                    let offset = self.items.as_ptr().offset(self.dirty_min as isize) as *const _;
                    gl::BufferSubData(self.buffer_type as u32, start, length, offset);
                }
            }
            // Timing finish.
            self.timer_sync.stop();
        }
    }
}

impl<T> Drop for DynamicBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo as *const _);
        }
    }
}
