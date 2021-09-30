use crate::Texture;
use cgmath::*;

const MAX: u32 = 65536;

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
    pub fn from_texture(texture: &Texture, left: u32, right: u32, top: u32, bottom: u32) -> TextureSection {
        let h_size = MAX / texture.width();
        let v_size = MAX / texture.height();
        let h_nudge = h_size >> 4;
        let v_nudge = v_size >> 4;
        TextureSection(Vector4::new(
            ((left * h_size) + h_nudge) as u16,   // Left
            ((right * h_size) - h_nudge) as u16,  // Right
            ((top * v_size) + v_nudge) as u16,    // Top
            ((bottom * v_size) - v_nudge) as u16, // Bottom
        ))
    }

    /// Creates a texture section that encompases the whole texture.
    pub fn full() -> TextureSection {
        TextureSection(Vector4::new(0, u16::MAX, 0, u16::MAX))
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
