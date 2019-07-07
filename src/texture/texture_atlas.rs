use crate::color;
use crate::texture::packer::*;
use crate::texture::*;
use cgmath::*;
use std::path::Path;

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
        manager.add_texture(Image::from_default(color::WHITE, 1, 1));
        manager.sync();
        manager
    }

    pub fn add_texture(&mut self, texture: Image) -> Vector4<u16> {
        let uv = self.packer.pack(&texture);
        self.dirty = true;
        info!("Loaded raw texture at {:?}", uv);
        uv
    }

    pub fn add_path(&mut self, path: &str) -> Vector4<u16> {
        let uv = self.packer.pack_path(Path::new(path));
        self.dirty = true;
        info!("Loaded texture {} at {:?}", path, uv);
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
