use render::shape::*;
use render::vertex::*;

#[repr(C)]
pub struct Quad<T: Vertex> {
    top_left: T,
    bottom_left: T,
    top_right: T,
    bottom_right: T,
}

impl<T: Vertex> Quad<T> {
    pub fn new(top_left: T, bottom_left: T, top_right: T, bottom_right: T) -> Quad<T> {
        Quad {
            top_left: top_left,
            bottom_left: bottom_left,
            top_right: top_right,
            bottom_right: bottom_right,
        }
    }
}

impl<T: Vertex> Shape for Quad<T> {
    type ShapeType = Quad<T>;
    type VertexType = T;
    type IndiceType = [u8; 6];

    fn generate_indicies(index: u8) -> Self::IndiceType {
        [
            index + 0,
            index + 1,
            index + 2,
            index + 2,
            index + 1,
            index + 3,
        ]
    }
}
