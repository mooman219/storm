use crate::{ColorDescription, Texture};
use cgmath::*;

const MAX_INTEGER: u16 = u16::MAX;
const MAX_FLOAT: f32 = u16::MAX as f32 + 1.0;

/// Token to reference a texture with. Has basic configuration settings.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct TextureSection(pub Vector4<u16>);

impl Default for TextureSection {
    fn default() -> TextureSection {
        TextureSection::full()
    }
}

impl TextureSection {
    /// Coordinates relative to the top left corner of the texture. (0, 0) is the top left of the
    /// texture, and (width, height) is the bottom right of the texture.
    pub fn from_texture<T: ColorDescription>(
        texture: &Texture<T>,
        left: u32,
        right: u32,
        top: u32,
        bottom: u32,
    ) -> TextureSection {
        let left = left as f32;
        let right = right as f32;
        let top = top as f32;
        let bottom = bottom as f32;
        let h_size = MAX_FLOAT / (texture.width() as f32);
        let v_size = MAX_FLOAT / (texture.height() as f32);
        let h_nudge = h_size * 0.25;
        let v_nudge = v_size * 0.25;
        TextureSection(Vector4::new(
            (left * h_size + h_nudge) as u16,   // Left
            (right * h_size - h_nudge) as u16,  // Right
            (top * v_size + v_nudge) as u16,    // Top
            (bottom * v_size - v_nudge) as u16, // Bottom
        ))
    }

    /// Creates a texture section that encompases the whole texture.
    pub fn full() -> TextureSection {
        TextureSection(Vector4::new(0, MAX_INTEGER, 0, MAX_INTEGER))
    }

    /// Mirrors the texture along the Y axis. Creates a new texture.
    pub fn mirror_y(&self) -> TextureSection {
        TextureSection(Vector4::new(self.0.y, self.0.x, self.0.z, self.0.w))
    }

    /// Mirrors the texture along the X axis. Creates a new texture.
    pub fn mirror_x(&self) -> TextureSection {
        TextureSection(Vector4::new(self.0.x, self.0.y, self.0.w, self.0.z))
    }
}
