use gl;
use render::buffer::*;
use render::enums::*;
use std::mem;
use std::ptr;
use time;
use time::timer::*;

pub struct ChunkedBuffer<T> {
    vbo: u32,
    dirty: bool,
    buffer_type: BufferType,
    current_chunk: usize,
    capacity: usize,
    items: Vec<T>,
    fence: Option<gl::types::GLsync>,
    map: *mut T,
    timer_sync: Timer,
}

const CHUNK_COUNT: usize = 4;
const CHUNK_MASK: usize = CHUNK_COUNT - 1;

impl<T> RawBuffer<T> for ChunkedBuffer<T> {
    fn new(buffer_type: BufferType, capacity: usize) -> ChunkedBuffer<T> {
        // Validate input
        if capacity == 0 {
            panic!("Capacity must be greater than 0.");
        }
        // Prepare data
        let items = Vec::with_capacity(capacity);
        let flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
        let mut vbo = 0u32;
        let map;
        // Call into opengl
        unsafe {
            let capacity = CHUNK_COUNT * capacity;
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
            current_chunk: 3,
            capacity: capacity,
            items: items,
            fence: None,
            map: map,
            timer_sync: Timer::new("[R] Chunked Sync"),
        }
    }

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
        self.current_chunk * self.capacity
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
            // Increment the current chunk, wrapping if needed.
            self.current_chunk = (self.current_chunk + 1) & CHUNK_MASK;
            unsafe {
                // Handle locking.
                if self.current_chunk == 0 {
                    match self.fence {
                        Some(fence) => {
                            let mut wait_flags = 0;
                            let mut wait_duration = 0;
                            loop {
                                match gl::ClientWaitSync(fence, wait_flags, wait_duration) {
                                    gl::ALREADY_SIGNALED | gl::CONDITION_SATISFIED => break,
                                    gl::WAIT_FAILED => panic!("Failed to wait for fence."),
                                    _ => {
                                        wait_flags = gl::SYNC_FLUSH_COMMANDS_BIT;
                                        wait_duration = time::convert::NANOS_PER_SEC;
                                    },
                                }
                            }
                            gl::DeleteSync(fence);
                        },
                        None => {},
                    }
                    self.fence = Some(gl::FenceSync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0));
                }
                // Write to the current chunk.
                let pointer = self.map.offset(self.offset_index() as isize);
                ptr::copy_nonoverlapping(self.items.as_ptr(), pointer, self.len());
            }
            // Timing finish.
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
