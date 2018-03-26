pub mod consumer;
pub mod producer;

use cgmath::*;
use render::color::*;

pub struct RenderFrame {
    pub quads: Vec<QuadMessage>,
    pub triangles: Vec<TriangleMessage>,
    pub translation: Option<Vector2<f32>>,
    pub scale: Option<f32>,
}

impl RenderFrame {
    pub fn new() -> RenderFrame {
        RenderFrame {
            quads: Vec::new(),
            triangles: Vec::new(),
            translation: None,
            scale: None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum QuadMessage {
    Create {
        pos: Vector2<f32>,
        size: Vector2<f32>,
        color: Color,
    },
    Update {
        id: usize,
        pos: Vector2<f32>,
        size: Vector2<f32>,
        color: Color,
    },
    Remove {
        id: usize,
    },
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TriangleMessage {
    Create {
        pos: Vector2<f32>,
        height: f32,
        color: Color,
    },
    Update {
        id: usize,
        pos: Vector2<f32>,
        height: f32,
        color: Color,
    },
    Remove {
        id: usize,
    },
}
