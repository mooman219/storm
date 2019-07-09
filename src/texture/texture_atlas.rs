use crate::color;
use crate::texture::packer::*;
use crate::texture::*;
use cgmath::*;

pub struct TextureAtlas {
    packer: TexturePacker,
    dirty: bool,
}

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        let mut manager = TextureAtlas {
            packer: TexturePacker::new(),
            dirty: false,
        };
        manager.add(Image::from_color(color::WHITE, 1, 1));
        manager.sync();
        manager
    }

    pub fn add(&mut self, texture: Image) -> Vector4<u16> {
        let uv = self.packer.pack(&texture);
        self.dirty = true;
        info!("Loaded raw texture at {:?}", uv);
        uv
    }

    pub fn sync(&mut self) -> Option<Image> {
        if self.dirty {
            self.dirty = false;
            Some(self.packer.export())
        } else {
            None
        }
    }
}
