use crate::render::gl::raw::*;
use crate::types::*;
use beryllium::{FullscreenStyle, GLWindow, SDLToken, WindowFlags, WINDOW_POSITION_CENTERED};
use cgmath::*;
use core::mem::{transmute, ManuallyDrop};

pub struct OpenGLWindow {
    inner: ManuallyDrop<GLWindow<'static>>,
}

impl Drop for OpenGLWindow {
    fn drop<'drop>(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
        }
    }
}

impl OpenGLWindow {
    pub fn new(desc: &WindowSettings, sdl: &SDLToken) -> OpenGLWindow {
        // Attributes
        sdl.gl_set_attribute(beryllium::GLattr::ContextFlags, beryllium::CONTEXT_FORWARD_COMPATIBLE_FLAG);
        sdl.gl_set_attribute(beryllium::GLattr::ContextProfileMask, beryllium::CONTEXT_PROFILE_CORE);
        sdl.gl_set_attribute(beryllium::GLattr::ContextMajorVersion, 3);
        sdl.gl_set_attribute(beryllium::GLattr::ContextMinorVersion, 3);

        // Make a window
        let window = sdl
            .create_window(
                &desc.title,
                WINDOW_POSITION_CENTERED,
                WINDOW_POSITION_CENTERED,
                1000,
                1000,
                WindowFlags::default().with_shown(false).with_hidden(true).with_opengl(true),
            )
            .expect("Unable to build the window.")
            .try_into_gl()
            .expect("Unable to upgrade into a GL window.");
        let window: GLWindow<'static> = unsafe { transmute(window) };

        // Load OpenGL
        load_with(|s| unsafe { sdl.gl_get_proc_address(s) });

        // Logging
        info!("SDL Loaded {:?}", beryllium::version());
        info!("OpenGL Loaded {}", get_string(StringTarget::Version));

        let window = OpenGLWindow {
            // This really isn't safe but sue me.
            inner: ManuallyDrop::new(window),
        };
        window.set_display_mode(desc.display_mode);
        window.set_vsync(desc.vsync);
        window.inner.show();
        window
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

    pub fn set_title(&self, title: &str) {
        (**self.inner).set_title(title);;
    }

    pub fn set_display_mode(&self, display_mode: DisplayMode) {
        match display_mode {
            DisplayMode::Windowed {
                width,
                height,
                resizable,
            } => {
                self.inner.set_size(width, height);
                self.inner.set_resizable(resizable);
                self.inner.set_fullscreen_style(FullscreenStyle::Windowed).unwrap();
            },
            DisplayMode::WindowedFullscreen => {
                let dm = (*self.inner).desktop_display_mode().unwrap();
                self.inner.set_size(dm.width, dm.height);
                self.inner.set_fullscreen_style(FullscreenStyle::FullscreenDesktop).unwrap();
            },
            DisplayMode::Fullscreen => {
                let dm = (*self.inner).desktop_display_mode().unwrap();
                self.inner.set_size(dm.width, dm.height);
                self.inner.set_fullscreen_style(FullscreenStyle::Fullscreen).unwrap();
            },
        }
    }

    pub fn set_vsync(&self, vsync: Vsync) {
        let setting = match vsync {
            Vsync::Disabled => 0,
            Vsync::Enabled => 1,
            Vsync::Adaptive => -1,
        };
        let result = unsafe { self.inner.set_swap_interval(setting) };
        if result.is_err() {
            warn!("Failed to set vsync to {:?}", vsync);
            match vsync {
                Vsync::Disabled => warn!("Unable to configure vsync."),
                Vsync::Enabled => self.set_vsync(Vsync::Disabled),
                Vsync::Adaptive => self.set_vsync(Vsync::Enabled),
            }
        } else {
            info!("Set vsync to {:?}", vsync);
        }
    }
}
