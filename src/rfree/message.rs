use batch::*;
use sprite::*;
use string::*;

pub struct BatchState {
    pub sprites: Vec<SpriteMessage>,
    pub strings: Vec<StringMessage>,
    pub desc: BatchDescription,
    pub dirty_sprites: bool,
    pub dirty_strings: bool,
    pub dirty_desc: bool,
}

impl Default for BatchState {
    fn default() -> BatchState {
        BatchState {
            sprites: Vec::new(),
            strings: Vec::new(),
            desc: BatchDescription::default(),
            dirty_sprites: false,
            dirty_strings: false,
            dirty_desc: false,
        }
    }
}

pub struct RenderState {
    pub batches: Vec<BatchState>,
    pub batch_changes: Vec<BatchMessage>,
    pub textures: Vec<TextureMessage>,
    pub fonts: Vec<FontMessage>,
    pub window: Vec<WindowMessage>,
}

impl Default for RenderState {
    fn default() -> RenderState {
        RenderState {
            batches: Vec::new(),
            batch_changes: Vec::new(),
            textures: Vec::new(),
            fonts: Vec::new(),
            window: Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SpriteMessage {
    pub desc: SpriteDescription,
}

#[derive(Clone, Debug)]
pub struct StringMessage {
    pub text: String,
    pub desc: StringDescription,
}

#[derive(Copy, Clone, Debug)]
pub enum BatchMessage {
    Create,
    Remove {
        index: usize,
    },
}

#[derive(Clone, Debug)]
pub enum TextureMessage {
    Load {
        path: String,
    },
}

#[derive(Clone, Debug)]
pub enum FontMessage {
    Load {
        path: String,
    },
}

#[derive(Clone, Debug)]
pub enum WindowMessage {
    Title {
        title: String,
    },
}
