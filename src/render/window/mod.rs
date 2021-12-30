#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::OpenGLWindow;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use self::wasm::OpenGLWindow;

use crate::{DisplayMode, WindowSettings};
use cgmath::Vector2;
use winit::event_loop::EventLoop;

pub(crate) trait OpenGLWindowContract: Sized {
    fn new(desc: &WindowSettings, event_loop: &EventLoop<()>) -> (Self, glow::Context);

    fn scale_factor(&self) -> f32;

    fn logical_size(&self) -> Vector2<f32>;

    fn physical_size(&self) -> Vector2<f32>;

    /// Swaps the buffers in case of double or triple buffering. You should call this function every
    /// time you have finished rendering, or the image may not be displayed on the screen. This
    /// function operates differently on different platforms. This is a no-op on web.
    fn swap_buffers(&self);

    fn set_title(&self, title: &str);

    fn set_display_mode(&self, display_mode: DisplayMode);
}
