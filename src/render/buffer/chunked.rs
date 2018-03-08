use gl;
use render::buffer::*;
use render::enums::*;
use std::mem;
use std::ptr;
use time::timer::*;

pub struct ChunkedBuffer<T> {
    vbo: u32,
    dirty: bool,
    buffer_type: BufferType,
    current_chunk: usize,
    chunk_count: usize,
    chunk_length: usize,
    items: Vec<T>,
    map: *mut T,
    timer_sync: Timer,
}

impl<T> ChunkedBuffer<T> {
    pub fn new(buffer_type: BufferType, chunk_count: usize, chunk_length: usize) -> ChunkedBuffer<T> {
        // Validate input
        if chunk_count == 0 {
            panic!("Chunk count must be greater than 0.");
        }
        if chunk_length == 0 {
            panic!("Chunk length must be greater than 0.");
        }
        // Prepare data
        let items = Vec::with_capacity(chunk_length);
        let flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
        let mut vbo = 0u32;
        let map;
        // Call into opengl
        unsafe {
            let capacity = chunk_count * chunk_length;
            let max_size = (capacity * mem::size_of::<T>()) as isize;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(buffer_type as u32, vbo);
            gl::BufferStorage(
                buffer_type as u32, // Buffer type
                max_size,           // Buffer size
                ptr::null(),        // Initial data
                flags,              // Flags
            );
            map = gl::MapBufferRange(
                buffer_type as u32, // Buffer type
                0,                  // Offset
                max_size,           // Length
                flags,              // Flags
            ) as *mut _;
        }
        // Finish
        ChunkedBuffer {
            vbo: vbo,
            dirty: false,
            buffer_type: buffer_type,
            current_chunk: chunk_count - 1,
            chunk_count: chunk_count,
            chunk_length: chunk_length,
            items: items,
            map: map,
            timer_sync: Timer::new("Chunked - Sync"),
        }
    }
}

impl<T> RawBuffer<T> for ChunkedBuffer<T> {
    fn add(&mut self, item: T) -> usize {
        let index = self.items.len();
        if index == self.chunk_length {
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
        self.current_chunk * self.chunk_length
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
            self.current_chunk = (self.current_chunk + 1) % self.chunk_count;
            unsafe {
                let pointer = self.map.offset(self.offset_index() as isize);
                ptr::copy_nonoverlapping(self.items.as_ptr(), pointer, self.len());
                // TODO: Proper sync logic
            }
            self.timer_sync.stop();
        }
    }
}

impl<T> Drop for ChunkedBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(self.buffer_type as u32, self.vbo);
            gl::UnmapBuffer(self.buffer_type as u32);
            gl::DeleteBuffers(1, &self.vbo as *const _);
        }
    }
}

// https://github.com/nvMcJohn/apitest/blob/a3f38b1c15ca160c883cddaa141c5376db56a5e6/src/framework/bufferlock.cpp
// https://github.com/nvMcJohn/apitest/blob/master/src/solutions/dynamicstreaming/gl/mappersistent.cpp
// https://github.com/nvMcJohn/apitest/blob/master/src/solutions/untexturedobjects/gl/mappersistent.cpp
