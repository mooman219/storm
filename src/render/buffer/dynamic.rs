use render::raw::*;
use std::cmp;
use std::mem;
use std::ptr;
use utility::storage::*;

pub struct DynamicBuffer<T: Copy> {
    vbo: u32,
    dirty: bool,
    dirty_min: usize,
    dirty_max: usize,
    capacity: usize,
    buffer_type: BufferBindingTarget,
    items: Storage<T>,
}

const DEFAULT_CAPACITY: usize = 512;

impl<T: Copy> DynamicBuffer<T> {
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

    pub fn new(buffer_type: BufferBindingTarget) -> DynamicBuffer<T> {
        let default_size = (mem::size_of::<T>() * DEFAULT_CAPACITY) as isize;
        let vbo = gen_buffer();
        bind_buffer(buffer_type, vbo);
        buffer_data(
            buffer_type,              // Buffer type
            default_size,             // Size
            ptr::null(),              // Initial data
            BufferUsage::DynamicDraw, // Usage
        );
        DynamicBuffer {
            vbo: vbo,
            dirty: false,
            dirty_min: 0,
            dirty_max: 0,
            capacity: DEFAULT_CAPACITY,
            buffer_type: buffer_type,
            items: Storage::new(),
        }
    }

    pub fn set_flattened(&mut self, items: &Vec<Vec<T>>) {
        unsafe {
            self.items.clear_without_drop();
            for item in items {
                self.items.push_range(item);
            }
        }
        self.dirty = true;
        self.dirty_min = 0;
        self.dirty_max = self.items.len();
    }

    pub fn set(&mut self, items: &Vec<T>) {
        unsafe {
            self.items.clear_without_drop();
            self.items.push_range(items);
        }
        self.dirty = true;
        self.dirty_min = 0;
        self.dirty_max = self.items.len();
    }

    pub fn push(&mut self, item: T) {
        let index = self.items.len();
        self.items.push(item);
        self.mark(index);
    }

    pub fn swap_remove(&mut self, index: usize) {
        self.items.swap_remove(index);
        self.mark(index);
    }

    pub fn update(&mut self, index: usize, item: T) {
        unsafe {
            self.items.update_without_drop(index, item);
        }
        self.mark(index);
    }

    pub fn clear(&mut self) {
        self.dirty = false;
        unsafe {
            self.items.clear_without_drop();
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn bind(&self) {
        bind_buffer(self.buffer_type, self.vbo);
    }

    pub fn sync(&mut self) {
        if self.dirty {
            self.dirty = false;
            bind_buffer(self.buffer_type, self.vbo);
            if self.capacity < self.items.cap() {
                let length = (mem::size_of::<T>() * self.items.cap()) as isize;
                let data = self.items.ptr() as *const _;
                buffer_data(self.buffer_type, length, data, BufferUsage::DynamicDraw);
                self.capacity = self.items.cap();
            } else {
                let start = (mem::size_of::<T>() * self.dirty_min) as isize;
                let length = (mem::size_of::<T>() * (self.dirty_max - self.dirty_min)) as isize;
                let data = unsafe { self.items.ptr().add(self.dirty_min) as *const _ };
                buffer_sub_data(self.buffer_type, start, length, data);
            }
        }
    }
}

impl<T: Copy> Drop for DynamicBuffer<T> {
    fn drop(&mut self) {
        delete_buffer(self.vbo);
    }
}
