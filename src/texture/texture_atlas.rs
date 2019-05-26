use cgmath::*;
use color;
use std::path::Path;
use texture::packer::*;
use texture::*;
use types::*;

pub struct TextureManager {
    packer: TexturePacker,
    uv: Vec<Vector4<u16>>,
    dirty: bool,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        let mut manager = TextureManager {
            packer: TexturePacker::new(),
            uv: Vec::new(),
            dirty: false,
        };
        manager.add_texture(Image::from_default(color::WHITE, 8, 8));
        manager.sync();
        manager
    }

    pub fn add_texture(&mut self, texture: Image) {
        let uv = self.packer.pack(&texture);
        self.uv.push(uv);
        self.dirty = true;
        trace!("Loaded raw texture at {:?}", uv);
    }

    pub fn add_path(&mut self, path: &str) {
        let uv = self.packer.pack_path(Path::new(path));
        self.uv.push(uv);
        self.dirty = true;
        trace!("Loaded texture {} at {:?}", path, uv);
    }

    pub fn get_uv(&self, reference: &TextureReference) -> Vector4<u16> {
        self.uv[reference.key()]
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
