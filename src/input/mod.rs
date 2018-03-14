pub mod consumer;
pub mod producer;

use cgmath::*;
use glutin::VirtualKeyCode;

pub struct InputFrame {
    pub cursor: Vec<CursorMessage>,
    pub key: Vec<KeyMessage>,
}

impl InputFrame {
    pub fn new() -> InputFrame {
        InputFrame {
            cursor: Vec::new(),
            key: Vec::new(),
        }
    }
}

/// Represents a keyboard event.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum KeyMessage {
    Pressed(VirtualKeyCode),
    Released(VirtualKeyCode),
}

/// Represents a cursor event. These are represented as an enumeration to
/// preserve ordering when stored in a vector and read sequentially.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CursorMessage {
    Moved(Vector2<f32>),
    Pressed(CursorButton),
    Released(CursorButton),
    Left,
    Entered,
}

/// Describes the cursor button being manipulated.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CursorButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Describes the new bounds the window has been resized to.
#[derive(Copy, Clone)]
pub struct ResizeMessage {
    pub width: u32,
    pub height: u32,
}
