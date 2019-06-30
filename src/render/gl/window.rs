use crate::render::gl::raw::*;
use crate::types::*;
use beryllium::*;
use cgmath::*;
use std::mem::*;

pub struct StormWindow {
    inner: ManuallyDrop<GLWindow<'static>>,
}

impl Drop for StormWindow {
    fn drop<'drop>(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
        }
    }
}

impl StormWindow {
    pub fn new(desc: &WindowDescription, sdl: &SDLToken) -> StormWindow {
        // Attributes
        sdl.gl_set_attribute(
            beryllium::GLattr::ContextFlags,
            beryllium::CONTEXT_DEBUG_FLAG | beryllium::CONTEXT_FORWARD_COMPATIBLE_FLAG,
        );
        sdl.gl_set_attribute(beryllium::GLattr::ContextProfileMask, beryllium::CONTEXT_PROFILE_CORE);
        sdl.gl_set_attribute(beryllium::GLattr::ContextMajorVersion, 4);
        sdl.gl_set_attribute(beryllium::GLattr::ContextMinorVersion, 1);

        // Make a window
        let window = sdl
            .create_window(
                &desc.title,
                WINDOW_POSITION_CENTERED,
                WINDOW_POSITION_CENTERED,
                desc.size.x,
                desc.size.y,
                WindowFlags::default().with_shown(true).with_opengl(true).with_resizable(desc.resizable),
            )
            .expect("Unable to build the window.")
            .try_into_gl()
            .expect("Unable to upgrade into a GL window.");
        let window: GLWindow<'static> = unsafe { core::mem::transmute(window) };

        // Load OpenGL
        load_with(|s| unsafe { sdl.gl_get_proc_address(s) });

        // Logging
        info!("SDL Loaded {:?}", beryllium::version());
        info!("OpenGL Loaded {}", get_string(StringTarget::Version));

        StormWindow {
            // This really isn't safe but sue me.
            inner: ManuallyDrop::new(window),
        }
    }

    #[inline]
    pub fn logical_size(&self) -> Vector2<f32> {
        let (x, y) = self.inner.size();
        Vector2::new(x as f32, y as f32)
    }

    #[inline]
    pub fn physical_size(&self) -> Vector2<f32> {
        let (x, y) = self.inner.drawable_size();
        Vector2::new(x as f32, y as f32)
    }

    /// Swaps the buffers in case of double or triple buffering. You should
    /// call this function every time you have finished rendering, or the
    /// image may not be displayed on the screen.
    #[inline]
    pub fn swap_buffers(&self) {
        unsafe {
            self.inner.swap_window();
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        (**self.inner).set_title(title);;
    }
}
