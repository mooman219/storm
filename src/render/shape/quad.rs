use render::enums::draw_mode::*;
use render::shape::*;
use render::vertex::*;

#[repr(C)]
pub struct Quad<T: Vertex> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: Vertex> Quad<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Quad<T> {
        Quad {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl<T: Vertex> Shape for Quad<T> {
    type ShapeType = Quad<T>;
    type VertexType = T;
    const DRAW_MODE: DrawMode = DrawMode::Triangles;
    const INDICIES: &'static [u8] = &[0, 1, 2, 2, 1, 3];
}
