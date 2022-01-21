use crate::asset::Asset;
use cgmath::*;

// Re-exports.
pub use winit::event::MouseButton as CursorButton;
pub use winit::event::VirtualKeyCode as KeyboardButton;

/// An input event. These are represented as an enumeration to preserve
/// ordering when stored in a vector and read sequentially.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    /// The window has requested it close.
    CloseRequested,
    /// Received a character. This includes control characters.
    ReceivedCharacter(char),
    /// Keyboard press event. Includes a flag for if this is a repeat event.
    KeyPressed {
        /// The button pressed.
        keycode: KeyboardButton,
        /// Flag for if this key was already pressed. Some environments may fire repeat key pressed
        /// events when the key is held.
        is_repeat: bool,
    },
    /// Keyboard release event.
    KeyReleased(KeyboardButton),
    /// Cursor press event. Contains the button pressed and the position it was pressed at.
    CursorPressed {
        /// Button pressed.
        button: CursorButton,
        /// Cursor position at time of press. This is based on the physical size of the window, with
        /// (0,0) being the bottom left.
        physical_pos: Vector2<f32>,
        /// Cursor position at time of press. This is normalized where the x and y values are
        /// between -1 and 1, with the bottom left of the screen being (-1, -1), and the top right
        /// being (1, 1). This may be useful for converting screen space coordinates into world\
        /// space.
        normalized_pos: Vector2<f32>,
    },
    /// Cursor release event. Contains the button released and the position it was released at.
    CursorReleased {
        /// Button released.
        button: CursorButton,
        /// Cursor position at time of release. This is based on the physical size of the window,
        /// with (0,0) being the bottom left.
        physical_pos: Vector2<f32>,
        /// Cursor position at time of release. This is normalized where the x and y values are
        /// between -1 and 1, with the bottom left of the screen being (-1, -1), and the top right
        /// being (1, 1). This may be useful for converting screen space coordinates into world\
        /// space.
        normalized_pos: Vector2<f32>,
    },
    /// Cursor wheel scroll event.
    CursorScroll(ScrollDirection),
    /// Cursor moved event. Contains the new position of the cursor.
    CursorMoved {
        /// Current cursor position. This is based on the physical size of the window, with (0,0)
        /// being the bottom left.
        physical_pos: Vector2<f32>,
        /// Current cursor position. This is normalized where the x and y values are between -1 and
        /// 1, with the bottom left of the screen being (-1, -1), and the top right being (1, 1).
        /// This may be useful for converting screen space coordinates into world space.
        normalized_pos: Vector2<f32>,
    },
    /// Cursor delta event. Contains the represents raw, unfiltered physical motion. Represents the
    /// change in physical position of the pointing device.
    CursorDelta {
        /// Change from last position.
        delta: Vector2<f32>,
    },
    /// Cursor left the bounds of the window event.
    CursorLeft,
    /// Cursor entered the bounds of the window event.
    CursorEntered,
    /// Window resized event. Contains the new dimensions of the window.
    WindowResized {
        physical_size: Vector2<f32>,
        logical_size: Vector2<f32>,
        scale_factor: f32,
    },
    /// This event is useful as a place to put your code that should be run after all state-changing
    /// events have been handled and you want to do stuff (updating state, performing calculations,
    /// etc) that happens as the "main body" of your event loop. The value is the time passed since
    /// the last update in seconds.
    Update(f32),
    /// Event for when as asset has finished reading and is now available for consumption.
    AssetRead(Asset),
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
