use cgmath::*;
use color::*;
use layer::*;
use texture::*;
use utility::unordered_tracker::*;

/// Handle to reference an uploaded sprite with.
#[derive(Copy, Clone, Debug)]
pub struct SpriteReference {
    key: Key<SpriteReference>,
    layer: LayerReference,
}

impl SpriteReference {
    pub(crate) fn new(key: Key<SpriteReference>, layer: LayerReference) -> SpriteReference {
        SpriteReference {
            key: key,
            layer: layer,
        }
    }

    pub(crate) fn key(&self) -> Key<SpriteReference> {
        self.key
    }

    pub(crate) fn layer(&self) -> &LayerReference {
        &self.layer
    }
}

/// Configuration description for a sprite.
#[derive(Copy, Clone, Debug)]
pub struct SpriteDescription {
    /// Units are measured in pixels.
    pub pos: Vector3<f32>,
    /// Units are measured in pixels.
    pub size: Vector2<f32>,
    pub color: Color,
    pub texture: TextureReference,
    /// Rotation is measured in turns from [0, 1).
    pub rotation: f32,
}

impl Default for SpriteDescription {
    fn default() -> SpriteDescription {
        SpriteDescription {
            pos: Vector3::new(0f32, 0f32, 0f32),
            size: Vector2::new(100f32, 100f32),
            color: BLACK,
            texture: DEFAULT_TEXTURE,
            rotation: 0.0,
        }
    }
}

impl SpriteDescription {
    /// Units are measured in pixels.
    pub fn pos(&mut self, pos: Vector3<f32>) -> &mut SpriteDescription {
        self.pos = pos;
        self
    }

    /// Units are measured in pixels.
    pub fn size(&mut self, size: Vector2<f32>) -> &mut SpriteDescription {
        self.size = size;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut SpriteDescription {
        self.color = color;
        self
    }

    pub fn texture(&mut self, texture: TextureReference) -> &mut SpriteDescription {
        self.texture = texture;
        self
    }

    /// Rotation is measured in turns from [0, 1).
    pub fn rotation(&mut self, rotation: f32) -> &mut SpriteDescription {
        self.rotation = rotation;
        self
    }
}
