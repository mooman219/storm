mod sprite;

pub use sprite::*;

use crate::render::raw;
use crate::render::OpenGLState;
use crate::types::ClearMode;
use crate::RGBA8;
use cgmath::*;

pub struct ClearSubLayer {
    clear_color: Option<RGBA8>,
    clear_mode: Option<u32>,
}

impl ClearSubLayer {
    pub(crate) fn new() -> ClearSubLayer {
        ClearSubLayer {
            clear_color: None,
            clear_mode: None,
        }
    }

    pub fn execute(&mut self) {
        let ctx = OpenGLState::ctx();
        if let Some(clear_mode) = self.clear_mode {
            if let Some(clear_color) = self.clear_color {
                ctx.gl.clear_color(clear_color);
            }
            ctx.gl.clear(clear_mode);
        }
    }

    pub fn set(&mut self, clear_mode: Option<ClearMode>) {
        self.clear_mode = match clear_mode {
            Some(mode) => {
                let mut raw = 0;
                if let Some(clear_color) = mode.color {
                    self.clear_color = Some(clear_color);
                    raw |= raw::ClearMode::COLOR;
                }
                if mode.depth {
                    raw |= raw::ClearMode::DEPTH;
                }
                if raw != 0 {
                    Some(raw)
                } else {
                    None
                }
            }
            None => None,
        };
    }
}

const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

pub struct TransformSubLayer {
    transform: Matrix4<f32>,
    logical_size: Vector2<f32>,
    ortho: Matrix4<f32>,
    ortho_transform: Matrix4<f32>,
}

impl TransformSubLayer {
    pub(crate) fn new() -> TransformSubLayer {
        TransformSubLayer {
            transform: IDENTITY_MATRIX,
            logical_size: Vector2::zero(),
            ortho: Matrix4::zero(),
            ortho_transform: Matrix4::zero(),
        }
    }

    pub fn set(&mut self, transform: Matrix4<f32>) {
        self.transform = transform;
        self.ortho_transform = self.ortho * transform;
    }

    pub fn ortho_transform(&mut self) -> &Matrix4<f32> {
        let ctx = OpenGLState::ctx();
        if ctx.logical_size() != self.logical_size {
            self.logical_size = ctx.logical_size();
            self.ortho = ctx.ortho();
            self.ortho_transform = ctx.ortho() * self.transform;
        }
        &self.ortho_transform
    }
}
