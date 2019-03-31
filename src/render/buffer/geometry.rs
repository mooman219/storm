use render::buffer::dynamic::*;
use render::buffer::*;
use render::geometry::*;
use render::raw::*;
use render::vertex::*;
use std::ptr;

pub struct GeometryBuffer<T: Geometry> {
    element_buffer: DynamicBuffer<T::IndiceType>,
    vertex_buffer: DynamicBuffer<T>,
    vertex_array: VertexArray<T::VertexType>,
}

// TODO: We need a Dynamic Indexed Object Buffer

impl<T: Geometry> GeometryBuffer<T> {
    pub fn new(capacity: usize) -> GeometryBuffer<T> {
        // Vertex Buffer Object
        let vertex_buffer = DynamicBuffer::new(BufferBindingTarget::ArrayBuffer, capacity);
        // Vertex Array Object
        let vertex_array = VertexArray::new();
        // Element Buffer Object
        let element_buffer = DynamicBuffer::new(BufferBindingTarget::ElementArrayBuffer, capacity);
        // Return
        GeometryBuffer {
            element_buffer: element_buffer,
            vertex_buffer: vertex_buffer,
            vertex_array: vertex_array,
        }
    }

    pub fn add(&mut self, element: T) -> usize {
        let length = self.vertex_buffer.add(element);
        if length > self.element_buffer.len() {
            self.element_buffer.add(T::generate_indice(length as u16));
        }
        length
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
        self.element_buffer.sync();
    }

    pub fn draw(&mut self) {
        let vertices = self.vertex_buffer.len() * T::VERTEX_COUNT;
        self.vertex_array.bind();
        draw_elements_base_vertex(
            DrawMode::Triangles,       // Draw mode
            vertices as i32,           // Number of vertices
            IndiceType::UnsignedShort, // Size of indices
            ptr::null(),               // Offset of indices
            0,                         // Base vertex offset
        );
    }
}
