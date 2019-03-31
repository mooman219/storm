use cgmath::*;
use layer::*;
use render::color::*;
use texture::*;
use utility::indexmap::*;

#[derive(Copy, Clone, Debug)]
pub struct SpriteReference {
    pub(crate) key: IndexToken,
    pub(crate) layer: LayerReference,
}

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
