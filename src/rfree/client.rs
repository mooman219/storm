use batch::*;
use render::*;
use sprite::*;
use string::*;
use texture::*;
use utility::bucket_spsc;
use utility::control;
use utility::unordered_tracker::*;

pub struct RenderClient {
    render_producer: bucket_spsc::Producer<RenderState>,
    render_control: control::Producer,
    batch_tracker: UnorderedTracker<BatchReference>,
    texture_count: usize,
    font_count: usize,
}

impl RenderClient {
    pub fn new(
        render_producer: bucket_spsc::Producer<RenderState>,
        render_control: control::Producer,
    ) -> RenderClient {
        RenderClient {
            render_producer: render_producer,
            render_control: render_control,
            batch_tracker: UnorderedTracker::new(),
            texture_count: 0,
            font_count: 0,
        }
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self, desc: &BatchDescription) -> BatchReference {
        let batch_key = self.batch_tracker.add();
        let state = self.render_producer.get();
        let mut batch_state = BatchState::default();
        batch_state.desc = *desc;
        batch_state.dirty_desc = true;
        state.batches.push(batch_state);
        state.batch_changes.push(BatchMessage::Create);
        BatchReference::new(batch_key)
    }

    pub fn batch_remove(&mut self, batch: &BatchReference) {
        let batch_index = self.batch_tracker.remove(batch.key());
        let state = self.render_producer.get();
        state.batches.swap_remove(batch_index);
        state.batch_changes.push(BatchMessage::Remove {
            index: batch_index,
        });
    }

    pub fn batch_update(&mut self, batch: &BatchReference, desc: &BatchDescription) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        state.batches[batch_index].desc = *desc;
        state.batches[batch_index].dirty_desc = true;
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_add(&mut self, batch: &BatchReference, desc: &SpriteDescription) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch_state = &mut state.batches[batch_index];
        batch_state.dirty_sprites = true;
        batch_state.sprites.push(SpriteMessage {
            desc: *desc,
        });
    }

    pub fn sprite_clear(&mut self, batch: &BatchReference) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch_state = &mut state.batches[batch_index];
        batch_state.dirty_sprites = true;
        unsafe {
            batch_state.sprites.set_len(0);
        }
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, path: &str) -> TextureReference {
        self.texture_count += 1;
        let state = self.render_producer.get();
        state.textures.push(TextureMessage::Load {
            path: String::from(path),
        });
        TextureReference::new(self.texture_count)
    }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, path: &str) -> FontReference {
        self.font_count += 1;
        let state = self.render_producer.get();
        state.fonts.push(FontMessage::Load {
            path: String::from(path),
        });
        FontReference::new(self.font_count)
    }

    pub fn string_add(&mut self, batch: &BatchReference, text: &str, desc: &StringDescription) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch_state = &mut state.batches[batch_index];
        batch_state.dirty_strings = true;
        batch_state.strings.push(StringMessage {
            text: String::from(text),
            desc: *desc,
        });
    }

    pub fn string_clear(&mut self, batch: &BatchReference) {
        let batch_index = self.batch_tracker.get(batch.key());
        let state = self.render_producer.get();
        let batch_state = &mut state.batches[batch_index];
        batch_state.dirty_strings = true;
        unsafe {
            batch_state.strings.set_len(0);
        }
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        let state = self.render_producer.get();
        state.window.push(WindowMessage::Title {
            title: String::from(title),
        });
    }

    pub fn commit(&mut self) {
        self.render_producer.spin_next();
        self.render_control.notify();
    }
}
