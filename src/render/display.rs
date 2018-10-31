use cgmath::*;
use glutin;
use glutin::dpi::*;
use glutin::GlContext;
use render::raw::*;

pub struct Display {
    window: glutin::GlWindow,
}

// Mark the display as send. In some systems, glutin::GlWindow isn't send so we
// make it as such. This might be a problem later, but not today.
unsafe impl Send for Display {}

impl Display {
    pub fn new(
        window_builder: glutin::WindowBuilder,
        context_builder: glutin::ContextBuilder,
        events_loop: &glutin::EventsLoop,
    ) -> Display {
        let gl_window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
        Display { window: gl_window }
    }

    pub fn bind(&self) {
        unsafe {
            self.window.make_current().unwrap();
        }
        load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        info!("Render: Bound new display");
        info!("Render: OpenGL version {}", get_string(StringTarget::Version));
    }

    // Window

    /// Modifies the title of the window.
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    /// Sets the window to maximized or back.
    pub fn set_maximized(&self, maximized: bool) {
        self.window.set_maximized(maximized);
    }

    /// Turn window decorations on or off.
    pub fn set_decorations(&self, decorations: bool) {
        self.window.set_decorations(decorations);
    }

    /// Change whether or not the window will always be on top of other windows.
    pub fn set_always_on_top(&self, always_on_top: bool) {
        self.window.set_always_on_top(always_on_top);
    }

    // Buffer

    /// Swaps the buffers in case of double or triple buffering.
    /// You should call this function every time you have finished rendering, or the image may not be displayed on the screen.
    pub fn swap_buffers(&self) {
        self.window.swap_buffers().expect("Error while swapping buffers.");
    }

    /// Resize the GL context.
    pub fn resize(&self, dimensions: Vector2<f64>) {
        self.window.resize(PhysicalSize::from((dimensions.x, dimensions.y)));
        viewport(0, 0, dimensions.x as i32, dimensions.y as i32);
    }
}
