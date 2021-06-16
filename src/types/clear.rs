use crate::RGBA8;

pub struct ClearMode {
    pub color: Option<RGBA8>,
    pub depth: bool,
}

impl ClearMode {
    pub fn color_depth(color: RGBA8) -> ClearMode {
        ClearMode {
            color: Some(color),
            depth: true,
        }
    }

    pub fn color(color: RGBA8) -> ClearMode {
        ClearMode {
            color: Some(color),
            depth: false,
        }
    }

    pub fn depth() -> ClearMode {
        ClearMode {
            color: None,
            depth: true,
        }
    }
}
