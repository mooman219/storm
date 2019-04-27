use cgmath::*;
use utility::ordered_tracker::*;

/// Handle to reference a layer with.
#[derive(Copy, Clone, Debug)]
pub struct LayerReference {
    key: Key<LayerReference>,
}

impl LayerReference {
    pub(crate) fn new(key: Key<LayerReference>) -> LayerReference {
        LayerReference {
            key: key,
        }
    }

    pub(crate) fn key(&self) -> Key<LayerReference> {
        self.key
    }
}

/// Configuration description for a sprite.
#[derive(Copy, Clone, Debug)]
pub struct LayerDescription {
    pub translation: Vector2<f32>,
    pub scale: f32,
    pub visible: bool,
}

impl Default for LayerDescription {
    fn default() -> LayerDescription {
        LayerDescription {
            translation: Vector2::new(0f32, 0f32),
            scale: 1f32,
            visible: true,
        }
    }
}
