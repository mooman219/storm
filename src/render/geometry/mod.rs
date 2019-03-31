mod quad;
mod triangle;

pub use render::geometry::quad::*;
pub use render::geometry::triangle::*;

use render::buffer::geometry::*;
use render::vertex::*;

pub trait Geometry {
    const VERTEX_COUNT: usize; // Number of verticies for the geometry
    const VERTEX_OFFSET: usize; // Number of unique verticies for the geometry
    type ShapeType: Geometry;
    type VertexType: Vertex;
    type IndiceType;

    fn generate_indice(index: u16) -> Self::IndiceType;

    fn new_geometry_buffer(capacity: usize) -> GeometryBuffer<Self::ShapeType> {
        GeometryBuffer::new(capacity)
    }
}
