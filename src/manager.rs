use layer::*;
use message::*;
use sprite::*;
use texture::*;
use utility::indexmap::*;

struct LayerSlot {
    reference: LayerReference,
    sprites: IndexMap,
}

pub struct StateManager {
    layers: Vec<LayerSlot>,
    texture_count: usize,
    layer_count: usize,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            layers: Vec::new(),
            texture_count: 0,
            layer_count: 0,
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

    pub fn layer_create(&mut self, depth: usize, desc: &LayerDescription) -> (RenderMessage, LayerReference) {
        self.layer_count += 1;
        let layer = LayerReference {
            depth: depth,
            key: self.layer_count,
        };
        let lookup = match self.layer_search(&layer) {
            Ok(index) => panic!("Given layer already exists."),
            Err(index) => index,
        };
        let slot = LayerSlot {
            reference: layer,
            sprites: IndexMap::new(),
        };
        self.layers.insert(lookup, slot);
        let message = RenderMessage::LayerCreate {
            layer: lookup,
            desc: *desc,
        };
        (message, layer)
    }

    pub fn layer_update(&mut self, layer: &LayerReference, desc: &LayerDescription) -> RenderMessage {
        let lookup = self.layer_get(layer);
        RenderMessage::LayerUpdate {
            layer: lookup,
            desc: *desc,
        }
    }

    pub fn layer_remove(&mut self, layer: &LayerReference) -> RenderMessage {
        let lookup = self.layer_get(layer);
        self.layers.remove(lookup);
        RenderMessage::LayerRemove { layer: lookup }
    }

    pub fn layer_clear(&mut self, layer: &LayerReference) -> RenderMessage {
        let lookup = self.layer_get(layer);
        let mut sprites = self.layers[lookup].sprites;
        sprites.clear();
        RenderMessage::LayerClear { layer: lookup }
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_create(
        &mut self,
        layer: &LayerReference,
        desc: &SpriteDescription,
    ) -> (RenderMessage, SpriteReference) {
        let lookup = self.layer_get(layer);
        let mut sprites = self.layers[lookup].sprites;
        let key = sprites.add();
        let sprite = SpriteReference {
            key: key,
            layer: *layer,
        };
        let message = RenderMessage::SpriteCreate {
            layer: lookup,
            desc: *desc,
        };
        (message, sprite)
    }

    pub fn sprite_update(&mut self, sprite: &SpriteReference, desc: &SpriteDescription) -> RenderMessage {
        let lookup = self.layer_get(sprite.layer());
        let mut sprites = self.layers[lookup].sprites;
        let key = sprites.get(sprite.key());
        RenderMessage::SpriteUpdate {
            layer: lookup,
            sprite: key,
            desc: *desc,
        }
    }

    pub fn sprite_remove(&mut self, sprite: &SpriteReference) -> RenderMessage {
        let lookup = self.layer_get(sprite.layer());
        let mut sprites = self.layers[lookup].sprites;
        let key = sprites.get(sprite.key());
        sprites.remove(sprite.key());
        RenderMessage::SpriteRemove {
            layer: lookup,
            sprite: key,
        }
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_lookup(&self, texture: TextureReference) -> usize {
        texture.key
    }

    pub fn texture_create(&mut self) -> TextureReference {
        self.texture_count += 1;
        TextureReference {
            key: self.texture_count,
        }
    }
}
