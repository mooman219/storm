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

    // Buffer

    pub fn swap_buffers(&self) {
        self.window.swap_buffers().expect("Error while swapping buffers.");
    }

    pub fn resize(&self, dimensions: Vector2<f64>) {
        self.window.resize(PhysicalSize::from((dimensions.x, dimensions.y)));
        viewport(0, 0, dimensions.x as i32, dimensions.y as i32);
    }
}
