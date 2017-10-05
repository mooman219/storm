pub mod quad;
pub mod shape_buffer;
pub mod triangle;

use render::shape::shape_buffer::*;
use render::vertex::*;

pub trait Shape {
    type ShapeType: Shape;
    type VertexType: Vertex;
    type IndiceType;

    fn generate_indicies(index: u8) -> Self::IndiceType;

    fn new_shape_buffer() -> ShapeBuffer<Self::ShapeType> {
        ShapeBuffer::new()
    }
}