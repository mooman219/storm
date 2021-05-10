use crate::texture::PIXEL_SIZE;
use cgmath::*;

/// Enumeration for all the loadable texture formats. Currently only PNG is supported.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureFormat {
    PNG,
}

/// Token to reference a texture with. Has basic configuration settings.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Texture(pub(crate) Vector4<u16>);

impl Default for Texture {
    /// A default texture reference for a basic white square.
    fn default() -> Texture {
        Texture(Vector4::new(0, PIXEL_SIZE as u16, 0, PIXEL_SIZE as u16))
    }
}

impl Texture {
    /// Mirrors the texture along the Y axis. Creates a new texture.
    pub fn mirror_y(&self) -> Texture {
        Texture(Vector4::new(self.0.y, self.0.x, self.0.z, self.0.w))
    }

    /// Mirrors the texture along the X axis. Creates a new texture.
    pub fn mirror_x(&self) -> Texture {
        Texture(Vector4::new(self.0.x, self.0.y, self.0.w, self.0.z))
    }

    /// Returns a sub texture from the given texture. Values are in pixels. The top left of the
    /// texture has the coordinates of 0, 0. This ignore any mirroring on the underlying texture.
    ///
    /// Returns an error if the size is 0, or the bounds of the sub texture are outside of the
    /// original texture.
    pub fn sub_texture(
        &self,
        minx: u16,
        miny: u16,
        width: u16,
        height: u16,
    ) -> Result<Texture, &'static str> {
        if width == 0 || height == 0 {
            Err("Size must be greater than 0")?
        }

        // UV Layout: xmin xmax ymin ymax
        let bounds = Vector4::new(
            u16::min(self.0.x, self.0.y), // Left
            u16::max(self.0.x, self.0.y), // Right
            u16::min(self.0.z, self.0.w), // Top
            u16::max(self.0.z, self.0.w), // Bottom
        );
        let subset = Vector4::new(
            bounds.x + (minx) * (PIXEL_SIZE as u16),          // Left
            bounds.x + (minx + width) * (PIXEL_SIZE as u16),  // Right
            bounds.z + (miny) * (PIXEL_SIZE as u16),          // Top
            bounds.z + (miny + height) * (PIXEL_SIZE as u16), // Bottom
        );

        if subset.x > bounds.y || subset.y > bounds.y || subset.z > bounds.w || subset.w > bounds.w {
            Err("Requested subtexture is outside the bounds of the source texture.")?
        }

        Ok(Texture(subset))
    }
}
