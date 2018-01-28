use gl;
use std::mem;
use std::cmp;
use std::ptr;

use render::enums::buffer_type::*;

pub struct Buffer<T> {
    vbo: u32,
    dirty: bool,
    buffer_min: usize,
    buffer_max: usize,
    buffer_capacity: usize,
    buffer_type: BufferType,
    items: Vec<T>,
}

impl<T> Buffer<T> {
    const ELEMENT_SIZE: usize = mem::size_of::<T>();
    const DEFAULT_CAPACITY: usize = 16;
    const DEFAULT_SIZE: usize = Buffer::<T>::ELEMENT_SIZE * Buffer::<T>::DEFAULT_CAPACITY;

    pub fn new(buffer_type: BufferType) -> Buffer<T> {
        let items: Vec<T> = Vec::<T>::with_capacity(Buffer::<T>::DEFAULT_CAPACITY);
        let mut vbo = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type.to_gl_enum(), vbo);
            gl::BufferData(
                buffer_type.to_gl_enum(),
                Buffer::<T>::DEFAULT_SIZE as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
        }
        Buffer {
            vbo: vbo,
            dirty: false,
            buffer_min: 0,
            buffer_max: 0,
            buffer_capacity: Buffer::<T>::DEFAULT_CAPACITY,
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
            gl::BindBuffer(self.buffer_type.to_gl_enum(), self.vbo);
        }
    }

    pub fn sync(&mut self) {
        unsafe {
            if self.dirty {
                gl::BindBuffer(self.buffer_type.to_gl_enum(), self.vbo);
                self.dirty = false;
                if self.buffer_capacity < self.items.capacity() {
                    let length = (Buffer::<T>::ELEMENT_SIZE * self.items.capacity()) as isize;
                    let offset = self.items.as_ptr() as *const _;
                    gl::BufferData(
                        self.buffer_type.to_gl_enum(),
                        length,
                        offset,
                        gl::DYNAMIC_DRAW,
                    );
                    self.buffer_capacity = self.items.capacity();
                } else {
                    let start = (Buffer::<T>::ELEMENT_SIZE * self.buffer_min) as isize;
                    let length = (Buffer::<T>::ELEMENT_SIZE * (self.buffer_max - self.buffer_min)) as isize;
                    let offset = self.items.as_ptr().offset(self.buffer_min as isize) as *const _;
                    gl::BufferSubData(self.buffer_type.to_gl_enum(), start, length, offset);
                }
            }
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo as *const _);
        }
    }
}
