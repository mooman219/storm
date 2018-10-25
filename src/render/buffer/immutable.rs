use render::buffer::*;
use render::raw::*;
use std::marker::PhantomData;
use std::mem;

pub struct ImmutableBuffer<T> {
    vbo: u32,
    length: usize,
    buffer_type: BufferBindingTarget,
    phantom: PhantomData<T>,
}

impl<T> ImmutableBuffer<T> {
    pub fn from_vec(buffer_type: BufferBindingTarget, items: Vec<T>) -> ImmutableBuffer<T> {
        let size = (mem::size_of::<T>() * items.len()) as isize;
        let data = items.as_ptr() as *const _;
        let vbo = gen_buffer();
        bind_buffer(buffer_type, vbo);
        buffer_data(
            buffer_type,             // Buffer type
            size,                    // Size
            data,                    // Initial data
            BufferUsage::StaticDraw, // Usage
        );
        ImmutableBuffer {
            vbo: vbo,
            length: items.len(),
            buffer_type: buffer_type,
            phantom: PhantomData,
        }
    }
}

impl<T> RawBuffer<T> for ImmutableBuffer<T> {
    fn new(_: BufferBindingTarget, _: usize) -> Self {
        panic!("Must provide initial data with ImmutableBuffer::from_vec.");
    }

    fn add(&mut self, _: T) -> usize {
        panic!("Cannot add immutable buffers.");
    }

    fn remove(&mut self, _: usize) {
        panic!("Cannot remove immutable buffers.");
    }

    fn update(&mut self, _: usize, _: T) {
        panic!("Cannot update immutable buffers.");
    }

    fn offset_index(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        self.length
    }

    fn bind(&self) {
        bind_buffer(self.buffer_type, self.vbo);
    }

    fn sync(&mut self) {
        // We're always in sync.
    }
}

impl<T> Drop for ImmutableBuffer<T> {
    fn drop(&mut self) {
        delete_buffer(self.vbo);
    }
}
