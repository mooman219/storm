use cgmath::*;
use render::color::*;
use render::geometry::*;
use render::vertex::*;
use render::vertex::shape::*;

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
    const VERTEX_OFFSET: usize = 3;
    type ShapeType = Self;
    type VertexType = T;
    type IndiceType = [u16; 3];

    fn generate_indice(index: u16) -> Self::IndiceType {
        let index = index * 3;
        [index + 0, index + 1, index + 2]
    }
}

// ////////////////////////////////////////////////////////
// Default implementations
// ////////////////////////////////////////////////////////

impl Triangle<ShapeVertex> {
    pub fn new_iso(pos: Vector2<f32>, height: f32, color: Color) -> Triangle<ShapeVertex> {
        let half = height.abs() / 2f32;
        // Points must be in the correct order for culling. Arrange the points
        // differently depending on the height.
        if height < 0f32 {
            Self::new(
                ShapeVertex::new(pos.x - half, pos.y, color),
                ShapeVertex::new(pos.x, pos.y + height, color),
                ShapeVertex::new(pos.x + half, pos.y, color),
            )
        } else {
            Self::new(
                ShapeVertex::new(pos.x, pos.y + height, color),
                ShapeVertex::new(pos.x - half, pos.y, color),
                ShapeVertex::new(pos.x + half, pos.y, color),
            )
        }
    }
}
