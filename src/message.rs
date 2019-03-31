use cgmath::*;
use layer::*;
use sprite::*;

pub use glutin::MouseButton as CursorButton;
pub use glutin::VirtualKeyCode as Key;

/// These are represented as an enumeration to preserve ordering when stored
/// in a vector and read sequentially.
#[derive(Copy, Clone)]
pub enum InputMessage {
    // Represents keyboard events.
    KeyPressed(Key),
    KeyReleased(Key),

    // Represents cursor events.
    CursorPressed(CursorButton, Vector2<f64>),
    CursorReleased(CursorButton, Vector2<f64>),
    CursorLeft,
    CursorEntered,
}

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

    // Window
    WindowTitle {
        title: String,
    },
}
