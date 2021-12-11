use crate::color::RGBA8;
use crate::render::raw::ClearMode as RawClearMode;

/// Parameters for how the screen should be cleared.
pub struct ClearMode {
    pub(crate) color: Option<RGBA8>,
    pub(crate) mode: u32,
}

impl ClearMode {
    pub fn color_depth(color: RGBA8) -> ClearMode {
        ClearMode {
            color: Some(color),
            mode: RawClearMode::COLOR | RawClearMode::DEPTH,
        }
    }

    pub fn color(color: RGBA8) -> ClearMode {
        ClearMode {
            color: Some(color),
            mode: RawClearMode::COLOR,
        }
    }

    pub fn depth() -> ClearMode {
        ClearMode {
            color: None,
            mode: RawClearMode::DEPTH,
        }
    }
}
