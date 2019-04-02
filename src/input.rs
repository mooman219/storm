use cgmath::*;

pub use glutin::MouseButton as CursorButton;
pub use glutin::VirtualKeyCode as Key;

/// An input event. These are represented as an enumeration to preserve
/// ordering when stored in a vector and read sequentially.
#[derive(Copy, Clone)]
pub enum InputMessage {
    /// Keyboard press event.
    KeyPressed(Key),
    /// Keyboard release event.
    KeyReleased(Key),
    /// Cursor press event.
    CursorPressed(CursorButton, Vector2<f64>),
    /// Cursor release event.
    CursorReleased(CursorButton, Vector2<f64>),
    /// Cursor left the bounds of the window event.
    CursorLeft,
    /// Cursor entered the bounds of the window event.
    CursorEntered,
}
