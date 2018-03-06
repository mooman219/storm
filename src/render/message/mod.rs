pub mod consumer;
pub mod producer;

use cgmath::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::vertex::shape::*;

pub struct RenderFrame {
    pub create_quad: Vec<CreateQuadMessage>,
    pub create_triangle: Vec<CreateTriangleMessage>,
    pub translation: SetTranslationMessage,
}

impl RenderFrame {
    pub fn new() -> RenderFrame {
        RenderFrame {
            create_quad: Vec::new(),
            create_triangle: Vec::new(),
            translation: SetTranslationMessage {
                set: true,
                translation: Vector3::new(0f32, 0f32, 0f32),
            },
        }
    }
}

pub struct CreateQuadMessage {
    pub quad: Quad<ShapeVertex>,
}

pub struct CreateTriangleMessage {
    pub triangle: Triangle<ShapeVertex>,
}

pub struct SetTranslationMessage {
    pub set: bool,
    pub translation: Vector3<f32>,
}
