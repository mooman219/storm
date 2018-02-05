use render::geometry::*;
use render::vertex::*;

#[repr(C)]
pub struct Triangle<T: Vertex> {
    top: T,
    left: T,
    right: T,
}

impl<T: Vertex> Triangle<T> {
    pub fn new(top: T, left: T, right: T) -> Triangle<T> {
        Triangle {
            top: top,
            left: left,
            right: right,
        }
    }
}

impl<T: Vertex> Geometry for Triangle<T> {
    const VERTEX_COUNT: usize = 3;
    type ShapeType = Self;
    type VertexType = T;
    type IndiceType = [u16; 3];

    fn generate_indicies(index: u16) -> Self::IndiceType {
        let index = index * 3;
        [index + 0, index + 1, index + 2]
    }
}
