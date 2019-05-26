use render::gl::raw::*;
use std::mem;
use std::ptr;
use utility::storage::*;

pub struct DynamicBuffer<T: Copy> {
    vbo: u32,
    dirty: bool,
    buffer_type: BufferBindingTarget,
    items: Storage<T>,
}

impl<T: Copy> DynamicBuffer<T> {
    pub fn new(buffer_type: BufferBindingTarget) -> DynamicBuffer<T> {
        let vbo = gen_buffer();
        bind_buffer(buffer_type, vbo);
        buffer_data(
            buffer_type,             // Buffer type
            0,                       // Size
            ptr::null(),             // Initial data
            BufferUsage::StaticDraw, // Usage
        );
        DynamicBuffer {
            vbo: vbo,
            dirty: false,
            buffer_type: buffer_type,
            items: Storage::new(),
        }
    }

    #[inline]
    pub fn set(&mut self, items: Vec<T>) {
        unsafe {
            self.items.clear_without_drop();
            self.items.push_range(items);
        }
        self.dirty = true;
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.dirty = true;
    }

    #[inline]
    pub fn push_range(&mut self, items: Vec<T>) {
        self.items.push_range(items);
        self.dirty = true;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.dirty = false;
        unsafe {
            self.items.clear_without_drop();
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn bind(&self) {
        bind_buffer(self.buffer_type, self.vbo);
    }

    #[inline]
    pub fn sync(&mut self) {
        if self.dirty {
            self.dirty = false;
            let length = (mem::size_of::<T>() * self.items.len()) as isize;
            let data = self.items.ptr() as *const _;
            bind_buffer(self.buffer_type, self.vbo);
            buffer_data(self.buffer_type, length, data, BufferUsage::StaticDraw);
        }
    }
}

impl<T: Copy> Drop for DynamicBuffer<T> {
    fn drop(&mut self) {
        delete_buffer(self.vbo);
    }
}
