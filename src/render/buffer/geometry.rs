use render::buffer::dynamic::*;
use render::buffer::*;
use render::raw::*;
use render::vertex::*;

pub struct GeometryBuffer<T: Vertex> {
    vertex_buffer: DynamicBuffer<T>,
    vertex_array: VertexArray<T>,
}

impl<T: Vertex> GeometryBuffer<T> {
    pub fn new(capacity: usize) -> GeometryBuffer<T> {
        let vertex_buffer = DynamicBuffer::new(BufferBindingTarget::ArrayBuffer, capacity);
        let vertex_array = VertexArray::new();
        GeometryBuffer {
            vertex_buffer: vertex_buffer,
            vertex_array: vertex_array,
        }
    }

    pub fn add(&mut self, element: T) -> usize {
        self.vertex_buffer.add(element)
    }

    pub fn remove(&mut self, index: usize) {
        self.vertex_buffer.remove(index);
    }

    pub fn update(&mut self, index: usize, element: T) {
        self.vertex_buffer.update(index, element);
    }

    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
    }

    pub fn sync(&mut self) {
        self.vertex_buffer.sync();
    }

    pub fn draw(&mut self) {
        let vertices = self.vertex_buffer.len();
        self.vertex_array.bind();
        draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, vertices as i32);
    }
}
