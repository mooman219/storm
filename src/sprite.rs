use cgmath::*;
use color::*;
use layer::*;
use texture::*;
use utility::indexed_empty_map::*;

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
    pub pos: Vector3<f32>,
    pub size: Vector2<f32>,
    pub color: Color,
    pub texture: TextureReference,
}

impl Default for SpriteDescription {
    fn default() -> SpriteDescription {
        SpriteDescription {
            pos: Vector3::new(0f32, 0f32, 0f32),
            size: Vector2::new(1f32, 1f32),
            color: WHITE,
            texture: DEFAULT_TEXTURE,
        }
    }
}
