use crate::colors::*;
use crate::texture::packer::Packer;
use crate::texture::*;
use cgmath::*;

const MAX: u32 = 65536;
const SIZE: u32 = 4096;
pub(crate) const PIXEL_SIZE: u32 = MAX / SIZE;

pub struct TextureAtlas {
    packer: Packer,
    atlas: Image,
    dirty: bool,
}

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        let mut atlas = TextureAtlas {
            packer: Packer::new(SIZE, SIZE),
            atlas: Image::from_color(WHITE, SIZE, SIZE),
            dirty: false,
        };
        atlas.add(Image::from_color(WHITE, 1, 1));
        atlas
    }

    pub fn add(&mut self, texture: Image) -> Vector4<u16> {
        if let Some(rect) = self.packer.pack(texture.width(), texture.height()) {
            self.atlas.set_texture(rect.x, rect.y, &texture);
            self.dirty = true;
            Vector4::new(
                (rect.x * PIXEL_SIZE) as u16,            // Left
                ((rect.x + rect.w) * PIXEL_SIZE) as u16, // Right
                (rect.y * PIXEL_SIZE) as u16,            // Top
                ((rect.y + rect.h) * PIXEL_SIZE) as u16, // Bottom
            )
        } else {
            panic!("Unable to fit texture into atlas.");
        }
    }

    pub fn sync(&mut self) -> Option<Image> {
        if self.dirty {
            self.dirty = false;
            Some(self.atlas.clone())
        } else {
            None
        }
    }
}
