use crate::color::RGBA8;
use crate::graphics::Texture;
use crate::graphics::{
    BlendFactor, BlendMode, Capability, ClearMode, CullFace, DepthTest, DisplayMode, OpenGL, OpenGLWindow,
    OpenGLWindowContract, PixelStoreAlignment, TextureFiltering, WindowSettings,
};
use crate::image::Image;
use crate::{App, Context};
use cgmath::*;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use log::trace;

#[no_mangle]
static mut _STORM_GRAPHICS_INITIALIZED: AtomicBool = AtomicBool::new(false);
#[no_mangle]
static mut _STORM_GRAPHICS: MaybeUninit<OpenGLState> = MaybeUninit::<OpenGLState>::uninit();

pub(crate) fn graphics() -> &'static mut OpenGLState {
    unsafe { _STORM_GRAPHICS.assume_init_mut() }
}

pub(crate) struct OpenGLState {
    gl: OpenGL,
    window: OpenGLWindow,
    logical_size: Vector2<f32>,
    physical_size: Vector2<f32>,
    default_texture: Option<Texture>,
    max_texture_size: i32,
    max_texture_anisotropy: Option<f32>,
}

impl OpenGLState {
    pub(crate) fn init(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) {
        if unsafe { _STORM_GRAPHICS_INITIALIZED.swap(true, Ordering::Relaxed) } {
            panic!("Graphics has already initialized.");
        }

        let (window, gl) = OpenGLWindow::new(desc, event_loop);
        let mut gl = OpenGL::new(gl);
        let extensions = gl.get_supported_extensions();
        let max_texture_size = gl.get_max_texture_size();
        let max_texture_anisotropy = if extensions.contains("GL_EXT_texture_filter_anisotropic") {
            Some(gl.get_max_texture_anisotropy())
        } else {
            None
        };
        gl.pixel_store(PixelStoreAlignment::UnpackAlignment, 1);
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        // gl.enable(Capability::DebugOutput); // DEBUG
        // gl.debug_message_callback(|source: u32, error_type: u32, id: u32, severity: u32, message: &str| {
        //     log::warn!(
        //         "source: {}, error_type: {}, id: {}, severity: {}, message: {}",
        //         source,
        //         error_type,
        //         id,
        //         severity,
        //         message
        //     );
        // }); // DEBUG
        gl.clear_color(RGBA8::BLACK);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha);
        gl.cull_face(CullFace::Back);
        trace!("MAX_TEXTURE_SIZE: {}", max_texture_size);

        unsafe {
            _STORM_GRAPHICS.write(OpenGLState {
                gl,
                logical_size: window.logical_size(),
                physical_size: window.physical_size(),
                window,
                default_texture: None,
                max_texture_size,
                max_texture_anisotropy,
            })
        };
    }

    #[inline(always)]
    pub(crate) fn gl(&mut self) -> &mut OpenGL {
        &mut self.gl
    }

    /// Gets the window.
    pub(crate) fn window(&mut self) -> &mut impl OpenGLWindowContract {
        &mut self.window
    }

    /// Gets the max anisotropic texture filtering supported by the GPU. Returns None if unsupported.
    pub(crate) fn max_texture_anisotropy(&self) -> Option<f32> {
        self.max_texture_anisotropy
    }

    /// Resizes the viewport.
    pub(crate) fn resize_viewport(&mut self, physical: Vector2<f32>, logical: Vector2<f32>) {
        if self.logical_size != logical || self.physical_size != physical {
            trace!("Window resized: Physical({:?}) Logical({:?})", physical, logical);
            self.logical_size = logical;
            self.physical_size = physical;
            self.gl.viewport(0, 0, physical.x as i32, physical.y as i32);
        }
    }
}

/// Graphics and window related functions.
impl<A: App> Context<A> {
    /// Returns a simple 1x1 white texture. This texture is reused globally.
    pub fn default_texture(&self) -> Texture {
        let graphics = graphics();
        match &graphics.default_texture {
            Some(texture) => texture.clone(),
            None => {
                let texture = Texture::from_image(
                    self,
                    &Image::from_color(RGBA8::WHITE, 1, 1),
                    TextureFiltering::none(),
                );
                graphics.default_texture = Some(texture.clone());
                texture
            }
        }
    }

    /// Sets if backface culling is enabled. This is enabled by default.
    pub fn set_backface_culling(&self, enabled: bool) {
        if enabled {
            graphics().gl().enable(Capability::CullFace)
        } else {
            graphics().gl().disable(Capability::CullFace)
        }
    }

    /// Gets the max texture size supported on the GPU.
    pub fn max_texture_size(&self) -> i32 {
        graphics().max_texture_size
    }

    /// Gets the max anisotropic texture filtering supported by the GPU. Returns None if unsupported.
    pub fn max_texture_anisotropy(&self) -> Option<f32> {
        graphics().max_texture_anisotropy
    }

    /// Sets the title of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** This sets the page title.
    pub fn set_window_title(&self, title: &str) {
        graphics().window.set_title(title);
    }

    /// Sets the display mode of the window.
    pub fn set_window_display_mode(&self, display_mode: DisplayMode) {
        graphics().window.set_display_mode(display_mode);
    }

    /// Gets the logical size of the window. This may differ from the viewport's logical size.
    pub fn window_logical_size(&self) -> Vector2<f32> {
        graphics().window.logical_size()
    }

    /// Gets the physical size of the window. This may differ from the viewport's physical size.
    pub fn window_physical_size(&self) -> Vector2<f32> {
        graphics().window.physical_size()
    }

    /// Grabs the cursor, preventing it from leaving the window.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** This locks the cursor in a fixed location, which looks visually awkward.
    pub fn window_cursor_grab(&self, grab: bool) {
        graphics().window.set_cursor_grab(grab)
    }

    /// Sets the visibility of the cursor.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** The cursor is only hidden within the confines of the window.
    /// - **X11:** The cursor is only hidden within the confines of the window.
    /// - **Wayland:** The cursor is only hidden within the confines of the window.
    /// - **macOS:** The cursor is hidden as long as the window has input focus, even if the cursor is
    ///   outside of the window.
    pub fn window_cursor_visibility(&self, grab: bool) {
        graphics().window.set_cursor_visible(grab)
    }

    /// Gets the logical size of the viewport. This may differ from the window's logical size.
    pub fn viewport_logical_size(&self) -> Vector2<f32> {
        graphics().logical_size
    }

    /// Gets the physical size of the viewport. This may differ from the window's physical size.
    pub fn viewport_physical_size(&self) -> Vector2<f32> {
        graphics().physical_size
    }

    /// Clears the screen buffers according to the given clear mode.
    pub fn clear(&self, clear_mode: ClearMode) {
        let gl = graphics().gl();
        if let Some(clear_color) = clear_mode.color {
            gl.clear_color(clear_color);
        }
        if let Some(depth) = clear_mode.depth {
            gl.clear_depth(depth);
        }
        if let Some(depth_test) = clear_mode.depth_test {
            gl.depth_func(depth_test);
        }
        gl.clear(clear_mode.mode);
    }

    /// Configures how pixels from different fragments are blended. A common setup for alpha
    /// blending is:
    /// ```
    /// BlendMode::Add(BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)
    /// ```
    /// Which represents `source_color * source_color.a + destination_color * (1 - source_color.a)`.
    /// This is the default blend mode on startup.
    pub fn blend_mode(&self, blend_mode: BlendMode) {
        blend_mode.apply();
    }
}
