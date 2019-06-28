use crate::render::gl::raw::*;
use beryllium::*;
use cgmath::*;

pub struct StormWindow {
    inner: Window<'static>,
}

impl StormWindow {
    pub fn new(window: Window, sdl: &SDLToken) -> StormWindow {
        // This really isn't safe but sue me.
        let window_static: Window<'static> = unsafe { core::mem::transmute(window) };
        StormWindow {
            inner: window_static,
        }
    }

    #[inline]
    pub fn get_logical_size(&self) -> Vector2<f32> {
        let (x, y) = self.inner.size();
        Vector2::new(x as f32, y as f32)
    }

    #[inline]
    pub fn get_physical_size(&self) -> Vector2<f32> {
        let (x, y) = self.inner.gl_get_drawable_size();
        Vector2::new(x as f32, y as f32)
    }

    /// Swaps the buffers in case of double or triple buffering. You should
    /// call this function every time you have finished rendering, or the
    /// image may not be displayed on the screen.
    #[inline]
    pub fn swap_buffers(&self) {
        unsafe {
            self.inner.gl_swap_window();
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        // TODO: This.
        // self.inner.set_title(title);
    }
}
