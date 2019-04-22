use layer::*;
use sprite::*;
use text::*;

pub enum RenderMessage {
    // Layer
    LayerCreate {
        layer: usize,
        desc: LayerDescription,
    },
    LayerUpdate {
        layer: usize,
        desc: LayerDescription,
    },
    LayerRemove {
        layer: usize,
    },
    LayerClear {
        layer: usize,
    },

    // Sprite
    SpriteCreate {
        layer: usize,
        desc: SpriteDescription,
    },
    SpriteUpdate {
        layer: usize,
        sprite: usize,
        desc: SpriteDescription,
    },
    SpriteRemove {
        layer: usize,
        sprite: usize,
    },

    // Texture
    TextureLoad {
        path: String,
    },

    // Text
    FontLoad {
        path: String,
    },
    TextCreate {
        layer_index: usize,
        text: String,
        desc: TextDescription,
    },
    TextUpdate {
        layer_index: usize,
        text_index: usize,
        text: String,
        desc: TextDescription,
    },
    TextRemove {
        layer_index: usize,
        text_index: usize,
    },

    // Window
    WindowTitle {
        title: String,
    },
}
