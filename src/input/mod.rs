pub mod consumer;
pub mod producer;

use cgmath::*;
use glutin::VirtualKeyCode;

/// These are represented as an enumeration to preserve ordering when stored
/// in a vector and read sequentially.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum InputFrame {
    // Represents keyboard events.
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),

    // Represents cursor events.
    CursorMoved(Vector2<f32>),
    CursorPressed(CursorButton),
    CursorReleased(CursorButton),
    CursorLeft,
    CursorEntered,
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
