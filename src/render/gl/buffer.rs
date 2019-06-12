use crate::render::gl::raw::*;
use crate::render::gl::vertex::*;
use std::mem;
use std::ptr;

pub struct Buffer<T: VertexDescription + Copy> {
    vbo: u32,
    vertices: usize,
    buffer_type: BufferBindingTarget,
    vertex_array: VertexArray<T>,
}

impl<T: VertexDescription + Copy> Buffer<T> {
    pub fn new(buffer_type: BufferBindingTarget) -> Buffer<T> {
        let vbo = gen_buffer();
        bind_buffer(buffer_type, vbo);
        buffer_data(
            buffer_type,             // Buffer type
            0,                       // Size
            ptr::null(),             // Initial data
            BufferUsage::StaticDraw, // Usage
        );
        let vertex_array = VertexArray::new();
        Buffer {
            vbo: vbo,
            vertices: 0,
            buffer_type: buffer_type,
            vertex_array: vertex_array,
        }
    }

    pub fn len(&self) -> usize {
        self.vertices
    }

    pub fn set(&mut self, items: &Vec<T>) {
        self.vertices = items.len();
        if self.vertices > 0 {
            let size = (mem::size_of::<T>() * items.len()) as isize;
            let data = items.as_ptr() as *const _;
            bind_buffer(self.buffer_type, self.vbo);
            buffer_data(self.buffer_type, size, data, BufferUsage::StaticDraw);
        }
    }

    pub fn draw(&self) {
        self.vertex_array.bind();
        draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, self.vertices as i32);
    }
}
