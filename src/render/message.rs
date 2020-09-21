use crate::texture::*;
use crate::types::*;
use crate::RGBA8;

pub struct RenderState {
    pub batches: Vec<BatchState>,
    pub batch_changes: Vec<BatchMessage>,
    pub atlas: Option<Image>,
    pub window: WindowState,
}

impl Default for RenderState {
    fn default() -> RenderState {
        RenderState {
            batches: Vec::new(),
            batch_changes: Vec::new(),
            atlas: None,
            window: WindowState::default(),
        }
    }
}

pub struct BatchState {
    pub sprites: Vec<Sprite>,
    pub strings: Vec<Sprite>,
    pub dirty_sprites: bool,
    pub dirty_strings: bool,
}

impl Default for BatchState {
    fn default() -> BatchState {
        BatchState {
            sprites: Vec::new(),
            strings: Vec::new(),
            dirty_sprites: false,
            dirty_strings: false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum BatchMessage {
    Create {
        desc: BatchSettings,
    },
    Update {
        index: usize,
        desc: BatchSettings,
    },
    Remove {
        index: usize,
    },
}

pub struct WindowState {
    pub title: Option<String>,
    pub clear_color: Option<RGBA8>,
    pub display_mode: Option<DisplayMode>,
    pub vsync: Option<Vsync>,
}

impl Default for WindowState {
    fn default() -> WindowState {
        WindowState {
            title: None,
            clear_color: None,
            display_mode: None,
            vsync: None,
        }
    }
}
