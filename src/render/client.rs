use crate::color::RGBA8;
use crate::render::message::*;
use crate::texture::*;
use crate::types::*;
use crate::utility::swap_spsc;
use crate::utility::unordered_tracker::*;
use std::path::Path;
use std::ptr;

pub struct RenderClient {
    render_producer: swap_spsc::Producer<RenderState>,
    texture_atlas: TextureAtlas,
    font_atlas: FontAtlas,
    batch_tracker: UnorderedTracker<BatchToken>,
    font_count: usize,
}

impl RenderClient {
    pub fn new(render_producer: swap_spsc::Producer<RenderState>) -> RenderClient {
        RenderClient {
            render_producer: render_producer,
            texture_atlas: TextureAtlas::new(),
            font_atlas: FontAtlas::new(),
            batch_tracker: UnorderedTracker::new(),
            font_count: 0,
        }
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self, desc: &BatchSettings) -> BatchToken {
        let state = self.render_producer.get();
        state.batches.push(BatchState::default());
        state.batch_changes.push(BatchMessage::Create {
            desc: *desc,
        });
        let batch_key = self.batch_tracker.add();
        BatchToken::new(batch_key)
    }

    pub fn batch_remove(&mut self, batch: &BatchToken) {
        let batch_index = self.batch_tracker.remove(batch.key());
        let state = self.render_producer.get();
        state.batches.swap_remove(batch_index);
        state.batch_changes.push(BatchMessage::Remove {
            index: batch_index,
        });
    }

    pub fn batch_update(&mut self, batch: &BatchToken, desc: &BatchSettings) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        state.batch_changes.push(BatchMessage::Update {
            index: batch_index,
            desc: *desc,
        });
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_set(&mut self, batch: &BatchToken, descs: &Vec<Sprite>) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch = &mut state.batches[batch_index];
        batch.dirty_sprites = true;
        unsafe {
            batch.sprites.set_len(0);
            batch.sprites.reserve(descs.len());
            ptr::copy_nonoverlapping(descs.as_ptr(), batch.sprites.as_mut_ptr(), descs.len());
            batch.sprites.set_len(descs.len());
        }
    }

    pub fn sprite_clear(&mut self, batch: &BatchToken) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch = &mut state.batches[batch_index];
        batch.dirty_sprites = true;
        unsafe { batch.sprites.set_len(0) };
    }

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, path: &str) -> FontToken {
        self.font_atlas.add_font_path(path);
        self.font_count += 1;
        FontToken::new(self.font_count)
    }

    pub fn string_set(&mut self, batch: &BatchToken, descs: &Vec<Text>) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch = &mut state.batches[batch_index];
        batch.dirty_strings = true;
        unsafe { batch.strings.set_len(0) };
        for desc in descs {
            self.font_atlas.rasterize(desc, &mut batch.strings);
        }
        state.font_atlas = self.font_atlas.sync();
    }

    pub fn string_clear(&mut self, batch: &BatchToken) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch = &mut state.batches[batch_index];
        batch.dirty_strings = true;
        unsafe { batch.strings.set_len(0) };
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, path: &Path) -> Texture {
        let uv = self.texture_atlas.add(Image::from_path(path));
        let state = self.render_producer.get();
        state.texture_atlas = self.texture_atlas.sync();
        Texture(uv)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        let state = self.render_producer.get();
        state.window.title = Some(String::from(title));
    }

    pub fn window_clear_color(&mut self, clear_color: RGBA8) {
        let state = self.render_producer.get();
        state.window.clear_color = Some(clear_color);
    }

    pub fn commit(&mut self) {
        if self.render_producer.try_next() {
            let state = self.render_producer.get();
            while state.batches.len() < self.batch_tracker.len() {
                state.batches.push(BatchState::default());
            }
        }
    }
}
