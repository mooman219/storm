pub mod quad;
pub mod shape_buffer;
pub mod triangle;

use render::enums::draw_mode::*;
use render::shape::shape_buffer::*;
use render::vertex::*;

pub trait Shape {
    type ShapeType: Shape;
    type VertexType: Vertex;

    const DRAW_MODE: DrawMode;
    const INDICIES: &'static[u8];

    fn new_shape_buffer() -> ShapeBuffer<Self::ShapeType> {
        ShapeBuffer::new()
    }
}