use gl;
use render::*;
use render::buffer::*;
use render::buffer::fixed::*;
use render::buffer::immutable::*;
use render::geometry::*;
use render::vertex::*;

pub struct GeometryBuffer<T: Geometry> {
    element_buffer: ImmutableBuffer<T::IndiceType>,
    vertex_buffer: FixedBuffer<T>,
    vertex_array: VertexArray<T::VertexType>,
}

// TODO: This can be a shape buffer since we're only drawing triangles.
impl<T: Geometry> GeometryBuffer<T> {
    pub fn new(capacity: usize) -> GeometryBuffer<T> {
        // Vertex Buffer Object
        let vertex_buffer = FixedBuffer::new(BufferType::ArrayBuffer, capacity);
        // Vertex Array Object
        let vertex_array = VertexArray::new();
        // Element Buffer Object
        let element_buffer = ImmutableBuffer::new(
            BufferType::ElementArrayBuffer,
            T::generate_indice_list(capacity as u16),
        );
        // Return
        GeometryBuffer {
            element_buffer: element_buffer,
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

    pub fn sync(&mut self) {
        self.vertex_buffer.sync();
    }

    pub fn draw(&mut self) {
        unsafe {
            let vertices = self.vertex_buffer.len() * T::VERTEX_COUNT;
            let offset_index = (self.vertex_buffer.offset_index() * T::VERTEX_OFFSET) as i32;
            self.vertex_array.bind();
            gl::DrawElementsBaseVertex(
                DrawMode::Triangles as u32, // Draw mode
                vertices as i32,            // Number of vertices
                gl::UNSIGNED_SHORT,         // Size of indices
                0 as *const _,              // Offset of indices
                offset_index,               // Base vertex offset
            );
        }
    }
}
