pub mod consumer;
pub mod producer;

pub struct InputFrame {}

impl InputFrame {
    pub fn new() -> InputFrame {
        InputFrame {}
    }
}

#[derive(Copy, Clone)]
pub struct ResizeMessage {
    pub width: u32,
    pub height: u32,
}
