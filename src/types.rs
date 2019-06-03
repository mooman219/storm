use cgmath::*;
use color::*;
use utility::unordered_tracker::*;

// ////////////////////////////////////////////////////////
// Batch
// ////////////////////////////////////////////////////////

/// Handle to reference a layer with.
#[derive(Copy, Clone, Debug)]
pub struct BatchReference {
    key: Key<BatchReference>,
}

impl BatchReference {
    pub(crate) fn new(key: Key<BatchReference>) -> BatchReference {
        BatchReference {
            key: key,
        }
    }

    pub(crate) fn key(&self) -> Key<BatchReference> {
        self.key
    }
}

/// Configuration description for a sprite.
#[derive(Copy, Clone, Debug)]
pub struct BatchDescription {
    pub translation: Vector2<f32>,
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
    pub visible: bool,
}

impl Default for BatchDescription {
    fn default() -> BatchDescription {
        BatchDescription {
            translation: Vector2::new(0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
            visible: true,
        }
    }
}

impl BatchDescription {
    pub fn translation(&mut self, translation: Vector2<f32>) -> &mut BatchDescription {
        self.translation = translation;
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut BatchDescription {
        self.scale = scale;
        self
    }

    pub fn rotation(&mut self, rotation: f32) -> &mut BatchDescription {
        self.rotation = rotation;
        self
    }

    pub fn visible(&mut self, visible: bool) -> &mut BatchDescription {
        self.visible = visible;
        self
    }
}

// ////////////////////////////////////////////////////////
// Sprite
// ////////////////////////////////////////////////////////

/// Configuration description for a sprite.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SpriteDescription {
    pos: Vector3<f32>,
    size: Vector2<u16>,
    uv: Vector4<u16>,
    color: RGBA8,
    rotation: u8,
}

impl Default for SpriteDescription {
    fn default() -> SpriteDescription {
        SpriteDescription {
            pos: Vector3::new(0.0, 0.0, 0.0),
            size: Vector2::new(100, 100),
            uv: Vector4::new(0, 4, 0, 4),
            color: BLACK,
            rotation: 0,
        }
    }
}

// {
//     /// Units are measured in pixels.
//     pub pos: Vector3<f32>,
//     /// Units are measured in pixels.
//     pub size: Vector2<f32>,
//     pub color: RGBA8,
//     pub texture: TextureReference,
//     /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into
// the     /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
//     pub rotation: f32,
// }

impl SpriteDescription {
    /// The Vector4's are in the order of (xmin, xmax, ymin, ymax).
    pub(crate) fn new_raw(
        pos: Vector3<f32>,
        size: Vector2<f32>,
        uv: Vector4<u16>,
        color: RGBA8,
        rotation: f32,
    ) -> SpriteDescription {
        SpriteDescription {
            pos: pos,
            size: {
                let x = (size.x as u32) & 0xFFFF;
                let y = (size.y as u32) & 0xFFFF;
                Vector2::new(x as u16, y as u16)
            },
            uv: uv,
            color: color,
            rotation: (rotation.fract() * 255f32) as u8,
        }
    }

    pub fn new(
        pos: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureReference,
        color: RGBA8,
        rotation: f32,
    ) -> SpriteDescription {
        Self::new_raw(pos, size, texture.uv, color, rotation)
    }

    /// Offset the position. Units are measured in pixels.
    pub fn pos(&mut self, pos: Vector3<f32>) -> &mut SpriteDescription {
        self.pos += pos;
        self
    }

    /// Offset the size. Units are measured in pixels.
    pub fn size(&mut self, size: Vector2<f32>) -> &mut SpriteDescription {
        self.size += {
            let x = (size.x as u32) & 0xFFFF;
            let y = (size.y as u32) & 0xFFFF;
            Vector2::new(x as u16, y as u16)
        };
        self
    }

    /// Offset the rotation. Rotation is measured in turns from [0, 1).
    pub fn rotation(&mut self, rotation: f32) -> &mut SpriteDescription {
        // self.rotation += (rotation.fract() * 255f32) as u8;
        self.rotation += 1;
        self
    }

    pub fn get_pos(&self) -> Vector3<f32> {
        self.pos
    }

    /// Set the color.
    pub fn set_color(&mut self, color: RGBA8) -> &mut SpriteDescription {
        self.color = color;
        self
    }

    /// Set the texture.
    pub fn set_texture(&mut self, texture: TextureReference) -> &mut SpriteDescription {
        self.uv = texture.uv;
        self
    }
}

// ////////////////////////////////////////////////////////
// String
// ////////////////////////////////////////////////////////

/// A default font reference for a basic white square.
pub const DEFAULT_FONT: FontReference = FontReference {
    key: 0,
};

/// Handle to reference an uploaded font with.
#[derive(Copy, Clone, Debug)]
pub struct FontReference {
    key: usize,
}

impl FontReference {
    pub(crate) fn new(key: usize) -> FontReference {
        FontReference {
            key: key,
        }
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }
}

/// Configuration description for text.
#[derive(Clone, Debug)]
pub struct StringDescription {
    pub string: String,
    pub pos: Vector3<f32>,
    pub max_width: Option<f32>,
    pub scale: u32,
    pub color: RGBA8,
    pub font: FontReference,
}

impl Default for StringDescription {
    fn default() -> StringDescription {
        StringDescription {
            string: String::new(),
            pos: Vector3::new(0f32, 0f32, 0f32),
            max_width: None,
            scale: 24,
            color: BLACK,
            font: DEFAULT_FONT,
        }
    }
}

impl StringDescription {
    pub fn string(&mut self, string: String) -> &mut StringDescription {
        self.string = string;
        self
    }

    pub fn pos(&mut self, pos: Vector3<f32>) -> &mut StringDescription {
        self.pos = pos;
        self
    }

    pub fn max_width(&mut self, max_width: Option<f32>) -> &mut StringDescription {
        self.max_width = max_width;
        self
    }

    pub fn scale(&mut self, scale: u32) -> &mut StringDescription {
        self.scale = scale;
        self
    }

    pub fn color(&mut self, color: RGBA8) -> &mut StringDescription {
        self.color = color;
        self
    }

    pub fn font(&mut self, font: FontReference) -> &mut StringDescription {
        self.font = font;
        self
    }
}

// ////////////////////////////////////////////////////////
// Texture
// ////////////////////////////////////////////////////////

/// A default texture reference for a basic white square.
pub const DEFAULT_TEXTURE: TextureReference = TextureReference {
    uv: Vector4::new(0, 4, 0, 4),
};

/// Handle to reference an uploaded texture with.
#[derive(Copy, Clone, Debug)]
pub struct TextureReference {
    uv: Vector4<u16>,
}

impl TextureReference {
    pub(crate) fn new(uv: Vector4<u16>) -> TextureReference {
        TextureReference {
            uv: uv,
        }
    }

    pub(crate) fn uv(&self) -> Vector4<u16> {
        self.uv
    }
}
