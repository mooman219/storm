use crate::render::buffer::Buffer;
use crate::render::raw;
use crate::render::raw::BufferBindingTarget;
use crate::render::OpenGLState;
use crate::types::{ClearMode, Sprite};
use crate::RGBA8;
use cgmath::*;

const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

/// Layers represent draw calls and hold configuration associated with drawing to the screen.
pub struct SpriteLayer {
    transform: Matrix4<f32>,
    logical_size: Vector2<f32>,
    ortho: Matrix4<f32>,
    ortho_transform: Matrix4<f32>,
    clear_color: Option<RGBA8>,
    clear_mode: Option<u32>,
    is_visible: bool,
    sprites: Buffer<Sprite>,
}

impl SpriteLayer {
    pub(crate) fn new() -> SpriteLayer {
        SpriteLayer {
            transform: IDENTITY_MATRIX,
            logical_size: Vector2::zero(),
            ortho: Matrix4::zero(),
            ortho_transform: Matrix4::zero(),
            clear_color: None,
            clear_mode: None,
            is_visible: true,
            sprites: Buffer::new(BufferBindingTarget::ArrayBuffer),
        }
    }

    pub fn draw(&mut self) {
        let ctx = OpenGLState::ctx();
        if let Some(clear_mode) = self.clear_mode {
            if let Some(clear_color) = self.clear_color {
                ctx.gl.clear_color(clear_color);
            }
            ctx.gl.clear(clear_mode);
        }
        if self.is_visible && self.sprites.len() > 0 {
            if ctx.logical_size() != self.logical_size {
                self.logical_size = ctx.logical_size();
                self.ortho = ctx.ortho();
                self.ortho_transform = ctx.ortho() * self.transform;
            }
            ctx.shader_ortho(&self.ortho_transform);
            self.sprites.draw();
        }
    }

    /// Clears the screen buffers according to the clear mode before draw is called.
    pub fn clear_mode(&mut self, clear_mode: Option<ClearMode>) {
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

    /// Sets the sprites that will be drawn.
    pub fn set_sprites(&mut self, sprites: &Vec<Sprite>) {
        self.sprites.set(sprites);
    }

    /// Clears all the sprites, drawing nothing.
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }

    /// Sets the transformation matrix used when drawing this.
    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.transform = transform;
        self.ortho_transform = self.ortho * transform;
    }

    /// If the renderer should render this layer or not when draw is called.
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }
}
