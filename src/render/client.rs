use layer::*;
use render::*;
use sprite::*;
use text::*;
use texture::*;
use utility::bucket_spsc;
use utility::control;
use utility::ordered_tracker::*;
use utility::unordered_tracker::*;

struct LayerSlot {
    depth: usize,
    sprite: UnorderedTracker<SpriteReference>,
    text: UnorderedTracker<TextReference>,
}

pub struct RenderClient {
    render_batch: Vec<RenderMessage>,
    render_producer: bucket_spsc::Producer<Vec<RenderMessage>>,
    render_control: control::Producer,
    layer_tracker: OrderedTracker<LayerReference>,
    layers: Vec<LayerSlot>,
    texture_count: usize,
    font_count: usize,
}

impl RenderClient {
    pub fn new(
        render_producer: bucket_spsc::Producer<Vec<RenderMessage>>,
        render_control: control::Producer,
    ) -> RenderClient {
        RenderClient {
            render_batch: Vec::new(),
            render_producer: render_producer,
            render_control: render_control,
            layer_tracker: OrderedTracker::new(),
            layers: Vec::new(),
            texture_count: 0,
            font_count: 0,
        }
    }

    // ////////////////////////////////////////////////////////
    // Layer
    // ////////////////////////////////////////////////////////

    fn layer_search(&self, depth: usize) -> Result<usize, usize> {
        self.layers.binary_search_by(|slot| slot.depth.cmp(&depth))
    }

    fn layer_get(&self, layer: &LayerReference) -> usize {
        self.layer_tracker.get(layer.key())
    }

    pub fn layer_create(&mut self, depth: usize, desc: &LayerDescription) -> LayerReference {
        let insert = match self.layer_search(depth) {
            Ok(index) => index,
            Err(index) => index,
        };
        self.layers.insert(
            insert,
            LayerSlot {
                depth: depth,
                sprite: UnorderedTracker::new(),
                text: UnorderedTracker::new(),
            },
        );
        self.render_producer.get().push(RenderMessage::LayerCreate {
            layer: insert,
            desc: *desc,
        });
        LayerReference::new(self.layer_tracker.insert(insert))
    }

    pub fn layer_update(&mut self, layer_ref: &LayerReference, desc: &LayerDescription) {
        let layer_index = self.layer_get(layer_ref);

        self.render_producer.get().push(RenderMessage::LayerUpdate {
            layer: layer_index,
            desc: *desc,
        });
    }

    pub fn layer_remove(&mut self, layer_ref: &LayerReference) {
        let layer_index = self.layer_get(layer_ref);
        self.layers.remove(layer_index);

        self.render_producer.get().push(RenderMessage::LayerRemove {
            layer: layer_index,
        });
    }

    pub fn layer_clear(&mut self, layer_ref: &LayerReference) {
        let layer_index = self.layer_get(layer_ref);
        let sprites = &mut self.layers[layer_index].sprite;
        sprites.clear();

        self.render_producer.get().push(RenderMessage::LayerClear {
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

        self.render_producer.get().push(RenderMessage::SpriteCreate {
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

        self.render_producer.get().push(RenderMessage::SpriteUpdate {
            layer: layer_index,
            sprite: sprite_index,
            desc: *desc,
        });
    }

    pub fn sprite_remove(&mut self, sprite_ref: &SpriteReference) {
        let layer_index = self.layer_get(sprite_ref.layer());
        let sprites = &mut self.layers[layer_index].sprite;
        let sprite_index = sprites.remove(sprite_ref.key());

        self.render_producer.get().push(RenderMessage::SpriteRemove {
            layer: layer_index,
            sprite: sprite_index,
        });
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, path: &str) -> TextureReference {
        self.texture_count += 1;

        self.render_producer.get().push(RenderMessage::TextureLoad {
            path: String::from(path),
        });
        TextureReference::new(self.texture_count)
    }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, path: &str) -> FontReference {
        self.font_count += 1;
        self.render_producer.get().push(RenderMessage::FontLoad {
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

        self.render_producer.get().push(RenderMessage::TextCreate {
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

        self.render_producer.get().push(RenderMessage::TextUpdate {
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

        self.render_producer.get().push(RenderMessage::TextRemove {
            layer_index: layer_index,
            text_index: text_index,
        });
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        self.render_producer.get().push(RenderMessage::WindowTitle {
            title: String::from(title),
        });
    }

    pub fn commit(&mut self) {
        self.render_producer.spin_next();
        self.render_control.notify();
    }
}
