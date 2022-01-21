use crate::color::RGBA8;
use crate::ctx;
use crate::graphics::Texture;
use crate::graphics::{
    BlendFactor, Capability, ClearMode, CullFace, DepthTest, DisplayMode, OpenGL, OpenGLWindow,
    OpenGLWindowContract, PixelStoreAlignment, WindowSettings,
};
use crate::image::Image;
use cgmath::*;
use log::trace;

pub(crate) struct OpenGLState {
    gl: OpenGL,
    window: OpenGLWindow,
    logical_size: Vector2<f32>,
    physical_size: Vector2<f32>,
    default_texture: Option<Texture>,
    max_texture_size: i32,
}

impl OpenGLState {
    pub(crate) fn init(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> OpenGLState {
        let (window, gl) = OpenGLWindow::new(desc, event_loop);
        let mut gl = OpenGL::new(gl);
        let max_texture_size = gl.get_max_texture_size();
        gl.pixel_store(PixelStoreAlignment::UnpackAlignment, 1);
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        // gl.enable(Capability::DebugOutput); // DEBUG
        // gl.debug_message_callback(|source: u32, error_type: u32, id: u32, severity: u32, message: &str| {
        //     warn!(
        //         "source: {}, error_type: {}, id: {}, severity: {}, message: {}",
        //         source, error_type, id, severity, message
        //     );
        // }); // DEBUG
        gl.clear_color(RGBA8::BLACK);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        gl.cull_face(CullFace::Back);
        trace!("MAX_TEXTURE_SIZE: {}", max_texture_size);

        let state = OpenGLState {
            gl,
            logical_size: window.logical_size(),
            physical_size: window.physical_size(),
            window,
            default_texture: None,
            max_texture_size,
        };
        state
    }

    #[inline(always)]
    pub(crate) fn gl(&mut self) -> &mut OpenGL {
        &mut self.gl
    }

    /// Gets the window.
    pub(crate) fn window(&mut self) -> &mut impl OpenGLWindowContract {
        &mut self.window
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

/// Returns a simple 1x1 white texture. This texture is reused globally.
pub fn default_texture() -> Texture {
    let graphics = ctx().graphics();
    match &graphics.default_texture {
        Some(texture) => texture.clone(),
        None => {
            let texture = Texture::from_image(&Image::from_color(RGBA8::WHITE, 1, 1));
            graphics.default_texture = Some(texture.clone());
            texture
        }
    }
}

/// Gets the max texture size supported on the GPU.
pub fn max_texture_size() -> i32 {
    ctx().graphics().max_texture_size
}

/// Sets the title of the window.
///
/// ## Platform-specific
///
/// - **Web:** This sets the page title.
pub fn set_window_title(title: &str) {
    ctx().graphics().window.set_title(title);
}

/// Sets the display mode of the window.
pub fn set_window_display_mode(display_mode: DisplayMode) {
    ctx().graphics().window.set_display_mode(display_mode);
}

/// Gets the logical size of the window. This may differ from the viewport's logical size.
pub fn window_logical_size() -> Vector2<f32> {
    ctx().graphics().window.logical_size()
}

/// Gets the physical size of the window. This may differ from the viewport's physical size.
pub fn window_physical_size() -> Vector2<f32> {
    ctx().graphics().window.physical_size()
}

/// Grabs the cursor, preventing it from leaving the window.
///
/// ## Platform-specific
///
/// - **macOS:** This locks the cursor in a fixed location, which looks visually awkward.
pub fn window_cursor_grab(grab: bool) {
    ctx().graphics().window.set_cursor_grab(grab)
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
pub fn window_cursor_visibility(grab: bool) {
    ctx().graphics().window.set_cursor_visible(grab)
}

/// Gets the logical size of the viewport. This may differ from the window's logical size.
pub fn viewport_logical_size() -> Vector2<f32> {
    ctx().graphics().logical_size
}

/// Gets the physical size of the viewport. This may differ from the window's physical size.
pub fn viewport_physical_size() -> Vector2<f32> {
    ctx().graphics().physical_size
}

/// Clears the screen buffers according to the clear mode.
pub fn clear(clear_mode: ClearMode) {
    let gl = ctx().graphics().gl();
    if let Some(clear_color) = clear_mode.color {
        gl.clear_color(clear_color);
    }
    gl.clear(clear_mode.mode);
}
