use super::{ClearSubLayer, TransformSubLayer};
use crate::render::buffer::Buffer;
use crate::render::raw::BufferBindingTarget;
use crate::render::OpenGLState;
use crate::types::{ClearMode, Sprite};
use cgmath::*;

/// Layers represent draw calls and hold configuration associated with drawing to the screen.
pub struct SpriteLayer {
    clear: ClearSubLayer,
    transform: TransformSubLayer,
    is_visible: bool,
    sprites: Buffer<Sprite>,
}

impl SpriteLayer {
    pub(crate) fn new() -> SpriteLayer {
        SpriteLayer {
            clear: ClearSubLayer::new(),
            transform: TransformSubLayer::new(),
            is_visible: true,
            sprites: Buffer::new(BufferBindingTarget::ArrayBuffer),
        }
    }

    /// Draws the layer to the screen.
    pub fn execute(&mut self) {
        self.clear.execute();
        if self.is_visible && self.sprites.len() > 0 {
            let ctx = OpenGLState::ctx();
            ctx.shader_ortho(self.transform.ortho_transform());
            self.sprites.draw();
        }
    }

    /// Clears the screen buffers according to the clear mode before draw is called.
    pub fn clear_mode(&mut self, clear_mode: Option<ClearMode>) {
        self.clear.set(clear_mode);
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
        self.transform.set(transform);
    }

    /// If the renderer should render this layer or not when draw is called.
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }
}
