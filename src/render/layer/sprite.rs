use super::{ClearLayer, TransformLayer};
use crate::render::buffer::Buffer;
use crate::render::raw::BufferBindingTarget;
use crate::render::OpenGLState;
use crate::types::Sprite;

/// Simple layer which draws sprites to the screen.
pub struct SpriteLayer {
    clear: ClearLayer,
    transform: TransformLayer,
    sprites: Buffer<Sprite>,
}

impl SpriteLayer {
    pub(crate) fn new() -> SpriteLayer {
        SpriteLayer {
            clear: ClearLayer::new(),
            transform: TransformLayer::new(),
            sprites: Buffer::new(BufferBindingTarget::ArrayBuffer),
        }
    }

    /// Draws the layer to the screen.
    pub fn draw(&mut self) {
        self.clear.execute();
        if self.sprites.len() > 0 {
            let ctx = OpenGLState::ctx();
            ctx.shader_ortho(self.transform.ortho_transform());
            self.sprites.draw();
        }
    }

    /// Sets the sprites that will be drawn.
    pub fn set_sprites(&mut self, sprites: &Vec<Sprite>) {
        self.sprites.set(sprites);
    }

    /// Clears all the sprites, drawing nothing.
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }

    /// Gets the clear settings for this layer.
    pub fn clear(&mut self) -> &mut ClearLayer {
        &mut self.clear
    }

    /// Gets the transform settings for this layer.
    pub fn transform(&mut self) -> &mut TransformLayer {
        &mut self.transform
    }
}
