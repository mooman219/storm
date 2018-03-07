pub mod consumer;
pub mod producer;

use cgmath::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::vertex::shape::*;

pub struct RenderFrame {
    pub create_quad: Vec<CreateQuadMessage>,
    pub create_triangle: Vec<CreateTriangleMessage>,
    pub translation: Option<SetTranslationMessage>,
}

impl RenderFrame {
    pub fn new() -> RenderFrame {
        RenderFrame {
            create_quad: Vec::new(),
            create_triangle: Vec::new(),
            translation: None,
        }
    }
}

pub struct CreateQuadMessage {
    pub quad: Quad<ShapeVertex>,
}

pub struct CreateTriangleMessage {
    pub triangle: Triangle<ShapeVertex>,
}

#[derive(Copy, Clone)]
pub struct SetTranslationMessage {
    pub translation: Vector3<f32>,
}
