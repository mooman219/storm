use layer::*;
use render::*;
use sprite::*;
use std::mem;
use text::*;
use texture::*;
use utility::indexed_empty_map::*;

struct LayerSlot {
    reference: LayerReference,
    sprite: IndexedEmptyMap<SpriteReference>,
    text: IndexedEmptyMap<TextReference>,
}

pub struct RenderClient {
    render_batch: Vec<RenderMessage>,
    render_producer: bounded_spsc::Producer<Vec<RenderMessage>>,
    render_control: control::Producer,
    layers: Vec<LayerSlot>,
    layer_count: usize,
    texture_count: usize,
    font_count: usize,
}

impl RenderClient {
    pub fn new(
        render_producer: bounded_spsc::Producer<Vec<RenderMessage>>,
        render_control: control::Producer,
    ) -> RenderClient {
        RenderClient {
            render_batch: Vec::new(),
            render_producer: render_producer,
            render_control: render_control,
            layers: Vec::new(),
            layer_count: 0,
            texture_count: 0,
            font_count: 0,
        }
    }

    // ////////////////////////////////////////////////////////
    // Layer
    // ////////////////////////////////////////////////////////

    fn layer_search(&self, layer: &LayerReference) -> Result<usize, usize> {
        self.layers.binary_search_by(|slot| slot.reference.cmp(layer))
    }

    fn layer_get(&self, layer: &LayerReference) -> usize {
        match self.layer_search(layer) {
            Ok(index) => index,
            _ => panic!("Given layer does not exist."),
        }
    }

    pub fn layer_create(&mut self, depth: usize, desc: &LayerDescription) -> LayerReference {
        self.layer_count += 1;
        let layer = LayerReference::new(depth, self.layer_count);
        let lookup = match self.layer_search(&layer) {
            Ok(_) => panic!("Given layer already exists."),
            Err(index) => index,
        };
        let slot = LayerSlot {
            reference: layer,
            sprite: IndexedEmptyMap::new(),
            text: IndexedEmptyMap::new(),
        };
        self.layers.insert(lookup, slot);

        self.render_batch.push(RenderMessage::LayerCreate {
            layer: lookup,
            desc: *desc,
        });
        layer
    }

    pub fn layer_update(&mut self, layer_ref: &LayerReference, desc: &LayerDescription) {
        let layer_index = self.layer_get(layer_ref);

        self.render_batch.push(RenderMessage::LayerUpdate {
            layer: layer_index,
            desc: *desc,
        });
    }

    pub fn layer_remove(&mut self, layer_ref: &LayerReference) {
        let layer_index = self.layer_get(layer_ref);
        self.layers.remove(layer_index);

        self.render_batch.push(RenderMessage::LayerRemove {
            layer: layer_index,
        });
    }

    pub fn layer_clear(&mut self, layer_ref: &LayerReference) {
        let layer_index = self.layer_get(layer_ref);
        let sprites = &mut self.layers[layer_index].sprite;
        sprites.clear();

        self.render_batch.push(RenderMessage::LayerClear {
            layer: layer_index,
        });
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_create(&mut self, layer: &LayerReference, desc: &SpriteDescription) -> SpriteReference {
        let layer_index = self.layer_get(layer);
        let sprites = &mut self.layers[layer_index].sprite;
        let sprite_index = sprites.add();
        let sprite = SpriteReference::new(sprite_index, *layer);

        self.render_batch.push(RenderMessage::SpriteCreate {
            layer: layer_index,
            desc: *desc,
        });
        sprite
    }

    pub fn sprite_update(&mut self, sprite: &SpriteReference, desc: &SpriteDescription) {
        // TODO: Only update if the sprite actually changed.
        let layer_index = self.layer_get(sprite.layer());
        let sprites = &mut self.layers[layer_index].sprite;
        let sprite_index = sprites.get(sprite.key());

        self.render_batch.push(RenderMessage::SpriteUpdate {
            layer: layer_index,
            sprite: sprite_index,
            desc: *desc,
        });
    }

    pub fn sprite_remove(&mut self, sprite_ref: &SpriteReference) {
        let layer_index = self.layer_get(sprite_ref.layer());
        let sprites = &mut self.layers[layer_index].sprite;
        let sprite_index = sprites.remove(sprite_ref.key());

        self.render_batch.push(RenderMessage::SpriteRemove {
            layer: layer_index,
            sprite: sprite_index,
        });
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, path: &str) -> TextureReference {
        self.texture_count += 1;

        self.render_batch.push(RenderMessage::TextureLoad {
            path: String::from(path),
        });
        TextureReference::new(self.texture_count)
    }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, path: &str) -> FontReference {
        self.font_count += 1;
        self.render_batch.push(RenderMessage::FontLoad {
            path: String::from(path),
        });
        FontReference::new(self.font_count)
    }

    pub fn text_create(
        &mut self,
        layer_ref: &LayerReference,
        text: &str,
        desc: &TextDescription,
    ) -> TextReference {
        let layer_index = self.layer_get(layer_ref);
        let texts = &mut self.layers[layer_index].text;
        let text_index = texts.add();
        let sprite = TextReference::new(text_index, *layer_ref);

        self.render_batch.push(RenderMessage::TextCreate {
            layer_index: layer_index,
            text: String::from(text),
            desc: *desc,
        });
        sprite
    }

    pub fn text_update(&mut self, text_ref: &TextReference, text: &str, desc: &TextDescription) {
        // TODO: Only update if the text actually changed.
        let layer_index = self.layer_get(text_ref.layer());
        let texts = &mut self.layers[layer_index].text;
        let text_index = texts.get(text_ref.key());

        self.render_batch.push(RenderMessage::TextUpdate {
            layer_index: layer_index,
            text_index: text_index,
            text: String::from(text),
            desc: *desc,
        });
    }

    pub fn text_remove(&mut self, text_ref: &TextReference) {
        let layer_index = self.layer_get(text_ref.layer());
        let texts = &mut self.layers[layer_index].text;
        let text_index = texts.remove(text_ref.key());

        self.render_batch.push(RenderMessage::TextRemove {
            layer_index: layer_index,
            text_index: text_index,
        });
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        self.render_batch.push(RenderMessage::WindowTitle {
            title: String::from(title),
        });
    }

    pub fn commit(&mut self) {
        // Only send a frame if there's actually frame data to send. The notify happens either way to
        // accommodate resizing.
        if self.render_batch.len() > 0 {
            let mut batch = Vec::new();
            mem::swap(&mut batch, &mut self.render_batch);
            self.render_producer.push(batch);
        }
        self.render_control.notify();
    }
}
