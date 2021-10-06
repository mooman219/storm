mod sprite;
mod text;

pub use sprite::*;
pub use text::*;

use crate::render::OpenGLState;
use cgmath::*;

const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

/// Holds transform parameters for a layer.
pub struct TransformLayer {
    transform: Matrix4<f32>,
    logical_size: Vector2<f32>,
    ortho: Matrix4<f32>,
    ortho_transform: Matrix4<f32>,
}

impl TransformLayer {
    pub(crate) fn new() -> TransformLayer {
        TransformLayer {
            transform: IDENTITY_MATRIX,
            logical_size: Vector2::zero(),
            ortho: Matrix4::zero(),
            ortho_transform: Matrix4::zero(),
        }
    }

    pub(crate) fn ortho_transform(&mut self) -> &Matrix4<f32> {
        let ctx = OpenGLState::ctx();
        if ctx.logical_size() != self.logical_size {
            self.logical_size = ctx.logical_size();
            self.ortho = ctx.ortho();
            self.ortho_transform = ctx.ortho() * self.transform;
        }
        &self.ortho_transform
    }

    /// Sets the transformation matrix used when drawing this.
    pub fn set(&mut self, transform: Matrix4<f32>) {
        self.transform = transform;
        self.ortho_transform = self.ortho * transform;
    }
}
