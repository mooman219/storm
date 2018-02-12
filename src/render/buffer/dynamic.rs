use gl;
use std::mem;
use std::cmp;
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
        let default_size = (mem::size_of::<T>() * DynamicBuffer::<T>::DEFAULT_CAPACITY) as isize;
        let items: Vec<T> = Vec::<T>::with_capacity(DynamicBuffer::<T>::DEFAULT_CAPACITY);
        let mut vbo = 0u32;
        unsafe {
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

    pub fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
        self.items.push(item);
        self.mark(index);
        index
    }

    pub fn remove(&mut self, index: usize) {
        self.items.swap_remove(index);
        self.mark(index);
    }

    pub fn update(&mut self, index: usize, item: T) {
        self.items[index] = item;
        self.mark(index);
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

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.vbo);
        }
    }

    pub fn sync(&mut self) {
        unsafe {
            if self.dirty {
                gl::BindBuffer(self.buffer_type, self.vbo);
                self.dirty = false;
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
