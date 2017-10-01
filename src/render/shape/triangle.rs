use render::enums::draw_mode::*;
use render::shape::*;
use render::vertex::*;

#[repr(C)]
pub struct Triangle<T: Vertex> {
    x: T,
    y: T,
    z: T,
}

impl<T: Vertex> Triangle<T> {
    pub fn new(x: T, y: T, z: T) -> Triangle<T> {
        Triangle { x: x, y: y, z: z }
    }
}

impl<T: Vertex> Shape for Triangle<T> {
    type ShapeType = Triangle<T>;
    type VertexType = T;
    const DRAW_MODE: DrawMode = DrawMode::Triangles;
    const INDICIES: &'static [u8] = &[0, 1, 2];
}
