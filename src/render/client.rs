use crate::render::message::*;
use crate::text::*;
use crate::texture::*;
use crate::types::*;
use crate::utility::swap_spsc;
use crate::utility::unordered_tracker::*;
use core::ptr;
use std::io::Read;

pub struct RenderClient {
    render_producer: swap_spsc::Producer<RenderState>,
    atlas: TextureAtlas,
    text_cache: TextCache,
    batch_tracker: UnorderedTracker<BatchToken>,
}

impl RenderClient {
    pub fn new(render_producer: swap_spsc::Producer<RenderState>) -> RenderClient {
        RenderClient {
            render_producer,
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
            batch_tracker: UnorderedTracker::new(),
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
        FontToken::new(self.text_cache.add_font_path(path))
    }

    pub fn string_set(&mut self, batch: &BatchToken, descs: &Vec<Text>) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch = &mut state.batches[batch_index];
        batch.dirty_strings = true;
        unsafe { batch.strings.set_len(0) };
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, &mut batch.strings);
        }
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

    pub fn texture_create<R: Read>(&mut self, bytes: R, format: TextureFormat) -> Texture {
        let image = Image::from_raw(bytes, format);
        let uv = self.atlas.add(image);
        Texture(uv)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        let state = self.render_producer.get();
        state.window.title = Some(String::from(title));
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        let state = self.render_producer.get();
        state.window.display_mode = Some(display_mode);
    }

    pub fn window_clear_color(&mut self, clear_color: RGBA8) {
        let state = self.render_producer.get();
        state.window.clear_color = Some(clear_color);
    }

    pub fn window_vsync(&mut self, vsync: Vsync) {
        let state = self.render_producer.get();
        state.window.vsync = Some(vsync);
    }

    pub fn commit(&mut self) {
        let state = self.render_producer.get();
        state.atlas = self.atlas.sync();
        if self.render_producer.try_next() {
            let state = self.render_producer.get();
            while state.batches.len() < self.batch_tracker.len() {
                state.batches.push(BatchState::default());
            }
        }
    }
}
