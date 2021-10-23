use super::TransformLayer;
use crate::prelude::Sprite;
use crate::render::buffer::Buffer;
use crate::render::raw::{BufferBindingTarget, TextureUnit};
use crate::render::OpenGLState;
use crate::Texture;
use crate::RGBA8;
use cgmath::Matrix4;

/// Simple layer which draws sprites to the screen.
pub struct SpriteLayer {
    transform: TransformLayer,
    sprites: Buffer<Sprite>,
    atlas: Texture<RGBA8>,
}

impl SpriteLayer {
    /// Creates a new sprite layer. Layers represent draw calls and hold configuration associated
    /// with drawing to the screen.
    pub(crate) fn new() -> SpriteLayer {
        let ctx = OpenGLState::ctx();
        SpriteLayer {
            transform: TransformLayer::new(),
            sprites: Buffer::new(BufferBindingTarget::ArrayBuffer),
            atlas: ctx.default_texture(),
        }
    }

    /// Draws the layer to the screen.
    pub fn draw(&mut self) {
        if self.sprites.len() > 0 {
            let ctx = OpenGLState::ctx();
            self.atlas.bind(TextureUnit::Alpha);
            ctx.sprite.bind();
            ctx.sprite.set_ortho(self.transform.ortho_transform());
            ctx.sprite.set_texture(TextureUnit::Alpha);
            self.sprites.draw();
        }
    }

    pub fn set_atlas(&mut self, handle: &Texture<RGBA8>) {
        self.atlas = handle.clone();
    }

    /// Sets the sprites that will be drawn.
    pub fn set_sprites(&mut self, sprites: &Vec<Sprite>) {
        self.sprites.set(sprites);
    }

    /// Clears all the sprites, drawing nothing.
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }

    /// Gets the transform settings for this layer.
    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.transform.set(transform);
    }
}
