use gl;
use glutin;
use glutin::GlContext;
use render::enums::*;

pub struct Display {
    window: glutin::GlWindow,
    clear_mode: u32,
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
        Display {
            window: gl_window,
            clear_mode: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.window.make_current().unwrap();
            gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }
        info!("Render: Bound new display");
        info!("Render: OpenGL version {}", GlString::Version.get_string());
    }

    // Clear

    pub fn clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear_stencil(&mut self, stencil: i32) {
        unsafe {
            gl::ClearStencil(stencil);
        }
    }

    pub fn clear_depth(&mut self, clamp: f32) {
        unsafe {
            gl::ClearDepthf(clamp);
        }
    }

    pub fn enable_clear_color(&mut self) {
        self.clear_mode |= gl::COLOR_BUFFER_BIT;
    }

    pub fn enable_clear_depth(&mut self) {
        self.clear_mode |= gl::DEPTH_BUFFER_BIT;
    }

    pub fn enable_clear_stencil(&mut self) {
        self.clear_mode |= gl::STENCIL_BUFFER_BIT;
    }

    pub fn disable_clear_color(&mut self) {
        self.clear_mode &= !gl::COLOR_BUFFER_BIT;
    }

    pub fn disable_clear_depth(&mut self) {
        self.clear_mode &= !gl::DEPTH_BUFFER_BIT;
    }

    pub fn disable_clear_stencil(&mut self) {
        self.clear_mode &= !gl::STENCIL_BUFFER_BIT;
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(self.clear_mode);
        }
    }

    // Buffer

    pub fn swap_buffers(&self) {
        self.window.swap_buffers().expect("Error while swapping buffers.");
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.window.resize(width, height);
    }
}
