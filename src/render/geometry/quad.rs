use cgmath::*;
use render::color::*;
use render::geometry::*;
use render::vertex::color::*;
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

impl<T: Vertex> Geometry for Quad<T> {
    const VERTEX_COUNT: usize = 6;
    const VERTEX_OFFSET: usize = 4;
    type ShapeType = Self;
    type VertexType = T;
    type IndiceType = [u16; 6];

    fn generate_indice(index: u16) -> Self::IndiceType {
        let index = index * 4;
        [index + 0, index + 1, index + 2, index + 2, index + 1, index + 3]
    }
}

// ////////////////////////////////////////////////////////
// Default implementations
// ////////////////////////////////////////////////////////

impl Quad<ColorVertex> {
    pub fn new_rect(pos: Vector3<f32>, size: Vector2<f32>, color: Color) -> Quad<ColorVertex> {
        Self::new(
            ColorVertex::new(pos.x, pos.y + size.y, pos.z, color),
            ColorVertex::new(pos.x, pos.y, pos.z, color),
            ColorVertex::new(pos.x + size.x, pos.y + size.y, pos.z, color),
            ColorVertex::new(pos.x + size.x, pos.y, pos.z, color),
        )
    }
}
