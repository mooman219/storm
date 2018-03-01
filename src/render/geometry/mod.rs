pub mod quad;
pub mod triangle;

use render::buffer::geometry::*;
use render::vertex::*;

pub trait Geometry {
    const VERTEX_COUNT: usize; // Number of verticies for the geometry
    const VERTEX_OFFSET: usize; // Number of unique verticies for the geometry
    type ShapeType: Geometry;
    type VertexType: Vertex;
    type IndiceType;

    fn generate_indice(index: u16) -> Self::IndiceType;

    fn generate_indice_list(length: u16) -> Vec<Self::IndiceType> {
        let mut items = Vec::with_capacity(length as usize);
        for x in 0..length {
            items.push(Self::generate_indice(x));
        }
        items
    }

    fn new_geometry_buffer(capacity: usize) -> GeometryBuffer<Self::ShapeType> {
        GeometryBuffer::new(capacity)
    }
}
