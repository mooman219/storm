use cgmath::*;

// Re-exports.
pub use winit::event::MouseButton as CursorButton;
pub use winit::event::VirtualKeyCode as KeyboardButton;

/// An input event. These are represented as an enumeration to preserve
/// ordering when stored in a vector and read sequentially.
#[derive(Copy, Clone, Debug)]
pub enum Event {
    /// The window has requested it close.
    CloseRequested,
    /// Received a character. This includes control characters.
    ReceivedCharacter(char),
    /// Keyboard press event.
    KeyPressed(KeyboardButton),
    /// Keyboard release event.
    KeyReleased(KeyboardButton),
    /// Cursor press event. Contains the button pressed and the position it was pressed at.
    CursorPressed {
        /// Button pressed.
        button: CursorButton,
        /// Cursor position at time of press.
        pos: Vector2<f32>,
    },
    /// Cursor release event. Contains the button released and the position it was released at.
    CursorReleased {
        /// Button released.
        button: CursorButton,
        /// Cursor position at time of release.
        pos: Vector2<f32>,
    },
    /// Cursor wheel scroll event.
    CursorScroll(ScrollDirection),
    /// Cursor moved event. Contains the position of the cursor and the delta from its last
    /// position.
    CursorMoved {
        /// Current cursor position.
        pos: Vector2<f32>,
        /// Change from last position.
        delta: Vector2<f32>,
    },
    /// Cursor left the bounds of the window event.
    CursorLeft,
    /// Cursor entered the bounds of the window event.
    CursorEntered,
    /// Window resized event. Contains the new dimensions of the window.
    WindowResized(Vector2<f32>),
    /// This event is useful as a place to put your code that should be run after all state-changing
    /// events have been handled and you want to do stuff (updating state, performing calculations,
    /// etc) that happens as the "main body" of your event loop. The value is the time passed since
    /// the last update in seconds.
    Update(f32),
}

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
