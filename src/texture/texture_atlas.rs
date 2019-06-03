use cgmath::*;
use color;
use std::path::Path;
use texture::packer::*;
use texture::*;

pub struct TextureAtlas {
    packer: TexturePacker,
    uv: Vec<Vector4<u16>>,
    dirty: bool,
}

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        let mut manager = TextureAtlas {
            packer: TexturePacker::new(),
            uv: Vec::new(),
            dirty: false,
        };
        manager.add_texture(Image::from_default(color::WHITE, 4, 4));
        manager.sync();
        manager
    }

    pub fn add_texture(&mut self, texture: Image) -> Vector4<u16> {
        let uv = self.packer.pack(&texture);
        self.dirty = true;
        trace!("Loaded raw texture at {:?}", uv);
        uv
    }

    pub fn add_path(&mut self, path: &str) -> Vector4<u16> {
        let uv = self.packer.pack_path(Path::new(path));
        self.dirty = true;
        trace!("Loaded texture {} at {:?}", path, uv);
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
