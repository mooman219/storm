pub mod quad;
pub mod triangle;

use render::buffer::shape_buffer::*;
use render::vertex::*;

pub trait Shape {
    const VERTEX_COUNT: usize;
    type ShapeType: Shape;
    type VertexType: Vertex;
    type IndiceType;

    fn generate_indicies(index: u8) -> Self::IndiceType;

    fn new_shape_buffer() -> ShapeBuffer<Self::ShapeType> {
        ShapeBuffer::new()
    }
}
