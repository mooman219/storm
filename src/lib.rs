#![no_std]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

pub extern crate log;

extern crate alloc;
// TODO: Resolve when glow is fixed.
// #[cfg(any(test, not(target_arch = "wasm32")))]
extern crate std;

/// Asset utilities.
pub mod asset;
/// Audio primitives. Creating and controlling sounds are included in here.
pub mod audio;
/// Color primitives. These are used in the graphics and image modules for managing images and
/// textures.
pub mod color;
/// Event utilities.
pub mod event;
/// Graphics primitives.
pub mod graphics;
/// Image utilities. Images are used for creating textures.
pub mod image;
/// Math utilities.
pub mod math;
/// Synchronization utilities.
pub mod sync;
/// Time utilities.
pub mod time;

mod context;

pub use cgmath;
pub use context::{request_stop, start, wait_for, wait_periodic, wait_until};
pub use fontdue;

pub(crate) use context::ctx;

/// Type that holds all of your application state and handles events.
pub trait App {
    /// This event is useful as a place to put your code that should be run after all state-changing
    /// events have been handled and you want to do stuff (updating state, performing calculations,
    /// etc) that happens as the "main body" of your event loop.
    /// # Arguments
    ///
    /// * `delta` - The time passed since the last update in seconds.
    fn on_update(&mut self, _delta: f32) {}

    /// The window has requested it close.
    fn on_close_requested(&mut self) {}

    /// Received a character. This includes control characters.
    /// # Arguments
    ///
    /// * `character` - The character typed.
    fn on_received_character(&mut self, _character: char) {}

    /// Keyboard press event. Includes a flag for if this is a repeat event.
    /// # Arguments
    ///
    /// * `key` - The button pressed.
    /// * `is_repeat` - Flag for if this key was already pressed. Some environments may fire repeat
    /// key pressed events when the key is held.
    fn on_key_pressed(&mut self, _key: event::KeyboardButton, _is_repeat: bool) {}

    /// Keyboard release event.
    /// # Arguments
    ///
    /// * `key` - The button released.
    fn on_key_released(&mut self, _key: event::KeyboardButton) {}

    /// Cursor press event. Contains the button pressed and the position it was pressed at.
    /// # Arguments
    ///
    /// * `button` - The button pressed.
    /// * `physical_pos` - Cursor position at time of press. This is based on the physical size of
    /// the window, with (0,0) being the bottom left.
    /// * `normalized_pos` - Cursor position at time of press. This is normalized where the x and y
    /// values are between -1 and 1, with the bottom left of the screen being (-1, -1), and the top
    /// right being (1, 1). This may be useful for converting screen space coordinates into world
    /// space.
    fn on_cursor_pressed(
        &mut self,
        _button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
    }

    /// Cursor press event. Contains the button pressed and the position it was pressed at.
    /// # Arguments
    ///
    /// * `button` - The button released.
    /// * `physical_pos` - Cursor position at time of release. This is base with (0,0) being the
    /// bottom left.
    /// * `normalized_pos` - Cursor position at time of release. This is normalized where the x and
    /// y values are between -1 and 1, with the bottom left of the screen being (-1, -1), and the
    /// top right being (1, 1). This may be useful for converting screen space coordinates into
    /// world space.
    fn on_cursor_released(
        &mut self,
        _button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
    }

    /// Cursor wheel scroll event..
    /// # Arguments
    ///
    /// * `direction` - The direction scrolled.
    fn on_cursor_scroll(&mut self, _direction: event::ScrollDirection) {}

    /// Cursor moved event. Use this for interacting with UI. Contains the new position of the
    /// cursor.
    /// # Arguments
    ///
    /// * `physical_pos` - Current cursor position. This is based on the physical size of the
    /// window, with (0,0) being the bottom left.
    /// * `normalized_pos` - Current cursor position. This is normalized where the x and y values
    /// are between -1 and 1, with the bottom left of the screen being (-1, -1), and the top right
    /// being (1, 1). This may be useful for converting screen space coordinates into world space.
    fn on_cursor_moved(
        &mut self,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
    }

    /// Cursor delta event. Use this to control a 3D camera. Contains the represents raw, unfiltered
    /// physical motion. Represents the change in physical position of the pointing device.
    /// # Arguments
    ///
    /// * `delta` - Change from last position. The units are arbitrary and up to the device.
    /// * `focused` - Flag for if the window is focused. This event may return deltas even when the
    /// window is not focused.
    fn on_cursor_delta(&mut self, _delta: cgmath::Vector2<f32>, _focused: bool) {}

    /// Cursor left the bounds of the window event.
    fn on_cursor_left(&mut self) {}

    /// Cursor entered the bounds of the window event.
    fn on_cursor_entered(&mut self) {}

    /// Window resized event. Contains the new dimensions of the window.
    /// # Arguments
    ///
    /// * `physical_size` - The size of the viewport.
    /// * `logical_size` - The logical size of the viewport. This is derived from the physical_size
    /// divided by the scale_factor.
    /// * `scale_factor` - The window's scale factor. This is a multiplier between the physical size
    /// and logical size of the window.
    fn on_window_resized(
        &mut self,
        _physical_size: cgmath::Vector2<f32>,
        _logical_size: cgmath::Vector2<f32>,
        _scale_factor: f32,
    ) {
    }

    /// The window gained or lost focus.
    /// # Arguments
    ///
    /// * `focused` - The parameter is true if the window has gained focus, and false if it has lost focus.
    fn on_window_focused(&mut self, _focused: bool) {}
}
