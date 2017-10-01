use gl;
use std::ffi::CStr;
use glutin::GlContext;
use glutin;

#[derive(Copy, Clone)]
pub enum GlString {
    Vendor,
    Renderer,
    Version,
    Shading_language_version,
    Extensions,
}

impl GlString {
    pub fn to_gl_enum(&self) -> u32 {
        match *self {
            GlString::Vendor => gl::VENDOR,
            GlString::Renderer => gl::RENDERER,
            GlString::Version => gl::VERSION,
            GlString::Shading_language_version => gl::SHADING_LANGUAGE_VERSION,
            GlString::Extensions => gl::EXTENSIONS,
        }
    }

    pub fn get_string(&self) -> String {
        unsafe {
            let data = CStr::from_ptr(gl::GetString(self.to_gl_enum()) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        }
    }
}

pub struct Display {
    window: glutin::GlWindow,
    clear_mode: u32,
}

impl Display {
    pub fn new(
        window_builder: glutin::WindowBuilder,
        context_builder: glutin::ContextBuilder,
        events_loop: &glutin::EventsLoop,
    ) -> Display {
        let gl_window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
        unsafe {
            gl_window.make_current().unwrap();
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }
        Display {
            window: gl_window,
            clear_mode: 0,
        }
    }

    // Strings

    pub fn get_vender_string(&self) -> String {
        GlString::Vendor.get_string()
    }

    pub fn get_renderer_string(&self) -> String {
        GlString::Renderer.get_string()
    }

    pub fn get_version_string(&self) -> String {
        GlString::Version.get_string()
    }

    pub fn get_shading_language_version_string(&self) -> String {
        GlString::Shading_language_version.get_string()
    }

    pub fn get_extensions_string(&self) -> String {
        GlString::Extensions.get_string()
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

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window.resize(width, height);
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers().unwrap();
    }
}
