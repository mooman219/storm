// Re-exports.
pub use winit::event::MouseButton as CursorButton;
pub use winit::event::VirtualKeyCode as KeyboardButton;

/// A cursor wheel movement. Some mice have left and right scroll options.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ScrollDirection {
    /// Cursor wheel scrolled up.
    Up,
    /// Cursor wheel scrolled down.
    Down,
    /// Cursor wheel scrolled left.
    Left,
    /// Cursor wheel scrolled right.
    Right,
}
