pub mod quad;
pub mod triangle;

use render::buffer::geometry::*;
use render::vertex::*;

pub trait Geometry {
    const VERTEX_COUNT: usize;
    type ShapeType: Geometry;
    type VertexType: Vertex;
    type IndiceType;

    fn generate_indicies(index: u16) -> Self::IndiceType;

    fn new_geometry_buffer() -> GeometryBuffer<Self::ShapeType> {
        GeometryBuffer::new()
    }
}
