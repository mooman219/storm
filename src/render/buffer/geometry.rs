use gl;

use render::enums::buffer_type::*;
use render::enums::draw_mode::*;
use render::geometry::*;
use render::vertex::*;

use render::buffer::raw::*;

pub struct GeometryBuffer<T: Geometry> {
    element_buffer: RawBuffer<T::IndiceType>,
    vertex_buffer: RawBuffer<T>,
    vao: u32,
}

impl<T: Geometry> GeometryBuffer<T> {
    pub fn new() -> GeometryBuffer<T> {
        // Element Buffer Object
        let element_buffer = RawBuffer::new(BufferType::ElementArrayBuffer);
        // Vertex Buffer Object
        let vertex_buffer = RawBuffer::new(BufferType::ArrayBuffer);
        // Vertex Array Object
        let mut vao = 0u32;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        T::VertexType::configure_vertex_attribute();
        // Return
        GeometryBuffer {
            element_buffer: element_buffer,
            vertex_buffer: vertex_buffer,
            vao: vao,
        }
    }

    pub fn add(&mut self, element: T) -> usize {
        let index = self.element_buffer.len() as u16;
        self.element_buffer.add(T::generate_indicies(index));
        self.vertex_buffer.add(element)
    }

    pub fn remove(&mut self, index: usize) {
        self.vertex_buffer.remove(index);
        // No need to update the element buffer.
    }

    pub fn update(&mut self, index: usize, element: T) {
        self.vertex_buffer.update(index, element);
    }

    pub fn sync(&mut self) {
        self.element_buffer.sync();
        self.vertex_buffer.sync();
    }

    pub fn draw(&self) {
        unsafe {
            let vertices = self.vertex_buffer.len() * T::VERTEX_COUNT;
            gl::BindVertexArray(self.vao);
            self.element_buffer.bind();
            gl::DrawElements(
                DrawMode::Triangles.to_gl_enum(),
                vertices as i32,
                gl::UNSIGNED_SHORT,
                0 as *const _,
            );
        }
    }
}

impl<T: Geometry> Drop for GeometryBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao as *const _);
        }
    }
}
