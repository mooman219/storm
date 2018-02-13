use gl;
use render::buffer::*;
use std::cmp;
use std::mem;
use std::ptr;

pub struct DynamicBuffer<T> {
    vbo: u32,
    dirty: bool,
    buffer_min: usize,
    buffer_max: usize,
    buffer_capacity: usize,
    buffer_type: u32,
    items: Vec<T>,
}

impl<T> DynamicBuffer<T> {
    const DEFAULT_CAPACITY: usize = 16;

    pub fn new(buffer_type: u32) -> DynamicBuffer<T> {
        let items: Vec<T> = Vec::<T>::with_capacity(DynamicBuffer::<T>::DEFAULT_CAPACITY);
        let mut vbo = 0u32;
        unsafe {
            let default_size = (mem::size_of::<T>() * DynamicBuffer::<T>::DEFAULT_CAPACITY) as isize;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type, vbo);
            gl::BufferData(
                buffer_type,      // Buffer type
                default_size,     // Size
                ptr::null(),      // Initial data
                gl::DYNAMIC_DRAW, // Usage
            );
        }
        DynamicBuffer {
            vbo: vbo,
            dirty: false,
            buffer_min: 0,
            buffer_max: 0,
            buffer_capacity: DynamicBuffer::<T>::DEFAULT_CAPACITY,
            buffer_type: buffer_type,
            items: items,
        }
    }

    fn mark(&mut self, index: usize) {
        if self.dirty {
            self.buffer_min = cmp::min(self.buffer_min, index);
            self.buffer_max = cmp::max(self.buffer_max, index + 1);
        } else {
            self.dirty = true;
            self.buffer_min = index;
            self.buffer_max = index + 1;
        }
    }
}

impl<T> RawBuffer<T> for DynamicBuffer<T> {
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

    fn offset(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.vbo);
        }
    }

    fn sync(&mut self) {
        if self.dirty {
            self.dirty = false;
            unsafe {
                gl::BindBuffer(self.buffer_type, self.vbo);
                if self.buffer_capacity < self.items.capacity() {
                    let length = (mem::size_of::<T>() * self.items.capacity()) as isize;
                    let data = self.items.as_ptr() as *const _;
                    gl::BufferData(self.buffer_type, length, data, gl::DYNAMIC_DRAW);
                    self.buffer_capacity = self.items.capacity();
                } else {
                    let start = (mem::size_of::<T>() * self.buffer_min) as isize;
                    let length = (mem::size_of::<T>() * (self.buffer_max - self.buffer_min)) as isize;
                    let offset = self.items.as_ptr().offset(self.buffer_min as isize) as *const _;
                    gl::BufferSubData(self.buffer_type, start, length, offset);
                }
            }
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
