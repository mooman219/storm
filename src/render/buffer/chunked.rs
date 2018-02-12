use gl;
use std::mem;
use std::ptr;

pub struct ChunkedBuffer<T> {
    vbo: u32,
    dirty: bool,
    buffer_type: u32,
    chunk_count: usize,
    chunk_length: usize,
    items: Vec<T>,
    map: *mut T,
}

impl<T> ChunkedBuffer<T> {
    pub fn new(buffer_type: u32, chunk_count: usize, chunk_length: usize) -> ChunkedBuffer<T> {
        let items = Vec::with_capacity(chunk_length);
        let capacity = chunk_count * chunk_length;
        let max_size = (capacity * mem::size_of::<T>()) as isize;
        let flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
        let mut vbo = 0u32;
        let mut map = 0 as *mut T;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type, vbo);
            gl::BufferStorage(
                buffer_type, // Buffer type
                max_size,    // Buffer size
                ptr::null(), // Initial data
                flags,       // Flags
            );
            map = gl::MapBufferRange(
                buffer_type, // Buffer type
                0,           // Offset
                max_size,    // Length
                flags,       // Flags
            ) as *mut _;
        }
        ChunkedBuffer {
            vbo: vbo,
            dirty: false,
            chunk_count: chunk_count,
            chunk_length: chunk_length,
            buffer_type: buffer_type,
            items: items,
            map: map,
        }
    }

    pub fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
        if index == self.chunk_length {
            panic!("Attempting to add element to full buffer.");
        }
        self.items.push(item);
        self.dirty = true;
        index
    }

    pub fn remove(&mut self, index: usize) {
        self.items.swap_remove(index);
        self.dirty = true;
    }

    pub fn update(&mut self, index: usize, item: T) {
        self.items[index] = item;
        self.dirty = true;
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
                // TODO: Proper sync logic
            }
        }
    }
}

impl<T> Drop for ChunkedBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo as *const _);
        }
    }
}
