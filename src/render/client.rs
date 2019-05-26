use texture::*;
use types::*;
use utility::bucket_spsc;
use utility::control;
use utility::unordered_tracker::*;

pub struct RenderClient {
    render_producer: bucket_spsc::Producer<RenderState>,
    texture_count: usize,
    font_count: usize,
}

impl RenderClient {
    pub fn new(render_producer: bucket_spsc::Producer<RenderState>) -> RenderClient {
        RenderClient {
            render_producer: render_producer,
            texture_count: 0,
            font_count: 0,
        }
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self, desc: &BatchDescription) -> BatchReference {}

    pub fn batch_remove(&mut self, batch: &BatchReference) {}

    pub fn batch_update(&mut self, batch: &BatchReference, desc: &BatchDescription) {}

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_set(&mut self, batch: &BatchReference, descs: &Vec<SpriteDescription>) {}

    pub fn sprite_clear(&mut self, batch: &BatchReference) {}

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, path: &str) -> FontReference {
        self.font_count += 1;
        FontReference::new(self.font_count)
    }

    pub fn string_set(&mut self, batch: &BatchReference, descs: &Vec<StringDescription>) {}

    pub fn string_clear(&mut self, batch: &BatchReference) {}

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, path: &str) -> TextureReference {
        self.texture_count += 1;
        TextureReference::new(self.texture_count)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {}

    pub fn commit(&mut self) {}
}
