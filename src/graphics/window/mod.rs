#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::OpenGLWindow;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use self::wasm::OpenGLWindow;

mod display_mode;
mod window_settings;

pub use display_mode::{DisplayMode, Vsync};
pub use window_settings::WindowSettings;

use cgmath::Vector2;
use winit::event_loop::EventLoop;

pub(crate) trait OpenGLWindowContract: Sized {
    fn new(desc: &WindowSettings, event_loop: &EventLoop<()>) -> (Self, glow::Context);

    /// Gets the scale factor of the window. This is related to DPI scaling.
    fn scale_factor(&self) -> f32;

    /// Gets the logical size of the window. This may differ from the viewport's logical size.
    fn logical_size(&self) -> Vector2<f32>;

    /// Gets the physical size of the window. This may differ from the viewport's physical size.
    fn physical_size(&self) -> Vector2<f32>;

    /// Grabs the cursor, preventing it from leaving the window.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** This locks the cursor in a fixed location, which looks visually awkward.
    fn set_cursor_grab(&self, grab: bool);

    /// Sets the visibility of the cursor.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** The cursor is only hidden within the confines of the window.
    /// - **X11:** The cursor is only hidden within the confines of the window.
    /// - **Wayland:** The cursor is only hidden within the confines of the window.
    /// - **macOS:** The cursor is hidden as long as the window has input focus, even if the cursor is
    ///   outside of the window.
    fn set_cursor_visible(&self, grab: bool);

    /// Sets the title of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** This sets the page title.
    fn set_title(&self, title: &str);

    /// Sets the display mode of the window.
    fn set_display_mode(&self, display_mode: DisplayMode);

    /// Swaps the buffers in case of double or triple buffering. You should call this function every
    /// time you have finished rendering, or the image may not be displayed on the screen.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** This is a no-op.
    fn swap_buffers(&self);
}