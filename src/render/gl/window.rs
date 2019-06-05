use cgmath::*;
use glutin;
use glutin::ContextTrait;
use render::gl::raw::*;

pub struct Window {
    inner: glutin::WindowedContext,
}

// Mark the display as send. In some systems, glutin::GlWindow isn't send so we
// make it as such. This might be a problem later, but not today.
unsafe impl Send for Window {}

impl Window {
    pub fn new(window: glutin::WindowedContext) -> Window {
        Window {
            inner: window,
        }
    }

    /// Initialize the display. The display is bound in the thread we're going
    /// to be making opengl calls in. Behavior is undefined is the display is
    /// bound outside of the thread and usually segfaults.
    pub fn bind(&self) {
        unsafe {
            self.inner.context().make_current().unwrap();
        }
        load_with(|symbol| self.inner.get_proc_address(symbol) as *const _);
        info!("Render: OpenGL version {}", get_string(StringTarget::Version));
    }

    #[inline]
    pub fn get_logical_size(&self) -> Vector2<f64> {
        let logical_size = self.inner.get_inner_size().expect("Window no longer exists.");
        Vector2::new(logical_size.width, logical_size.height)
    }

    #[inline]
    pub fn get_physical_size(&self) -> Vector2<f64> {
        let physical_size = self
            .inner
            .get_inner_size()
            .expect("Window no longer exists.")
            .to_physical(self.inner.get_hidpi_factor());
        Vector2::new(physical_size.width, physical_size.height)
    }

    /// Swaps the buffers in case of double or triple buffering. You should
    /// call this function every time you have finished rendering, or the
    /// image may not be displayed on the screen.
    #[inline]
    pub fn swap_buffers(&self) {
        self.inner.swap_buffers().expect("Error while swapping buffers.");
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }
}
