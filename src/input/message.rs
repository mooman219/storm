use cgmath::*;

// Re-exports.
pub use beryllium::Keycode as KeyboardButton;
pub use beryllium::MouseButton as CursorButton;

/// An input event. These are represented as an enumeration to preserve
/// ordering when stored in a vector and read sequentially.
#[derive(Copy, Clone)]
pub enum InputMessage {
    /// The window has requested it close.
    CloseRequested,
    /// Keyboard press event.
    KeyPressed(KeyboardButton),
    /// Keyboard release event.
    KeyReleased(KeyboardButton),
    /// Cursor press event.
    CursorPressed(CursorButton, Vector2<f32>),
    /// Cursor release event.
    CursorReleased(CursorButton, Vector2<f32>),
    /// Cursor moved event.
    CursorMoved(Vector2<f32>),
    /// Cursor left the bounds of the window event.
    CursorLeft,
    /// Cursor entered the bounds of the window event.
    CursorEntered,
}
