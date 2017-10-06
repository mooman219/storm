use render::shape::*;
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

impl<T: Vertex> Shape for Triangle<T> {
    type ShapeType = Triangle<T>;
    type VertexType = T;
    type IndiceType = [u8; 3];

    fn generate_indicies(index: u8) -> Self::IndiceType {
        let index = index * 3;
        [index + 0, index + 1, index + 2]
    }
}
