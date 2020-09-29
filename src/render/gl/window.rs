use crate::types::*;
use cgmath::*;

pub struct OpenGLWindow {
    inner: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
}

impl OpenGLWindow {
    pub fn new(
        desc: &WindowSettings,
        event_loop: &glutin::event_loop::EventLoop<()>,
    ) -> (OpenGLWindow, glow::Context) {
        let mut window_builder = glutin::window::WindowBuilder::new().with_title(&desc.title);
        match desc.display_mode {
            DisplayMode::Windowed {
                width,
                height,
                resizable,
            } => {
                window_builder = window_builder
                    .with_resizable(resizable)
                    .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
            }
            DisplayMode::WindowedFullscreen => {
                let fullscreen = glutin::window::Fullscreen::Borderless(event_loop.primary_monitor());
                window_builder = window_builder.with_fullscreen(Some(fullscreen));
            }
            DisplayMode::Fullscreen => {
                let fullscreen = glutin::window::Fullscreen::Borderless(event_loop.primary_monitor());
                window_builder = window_builder.with_fullscreen(Some(fullscreen));
            }
        }
        let mut context_builder = glutin::ContextBuilder::new();
        match desc.vsync {
            Vsync::Disabled => {
                context_builder = context_builder.with_vsync(false);
            }
            Vsync::Enabled => {
                context_builder = context_builder.with_vsync(true);
            }
        }
        let window_context = context_builder.build_windowed(window_builder, &event_loop).unwrap();
        let window_context = unsafe { window_context.make_current() }.unwrap();
        let gl = unsafe {
            glow::Context::from_loader_function(|s| window_context.get_proc_address(s) as *const _)
        };
        info!("Created window.");
        (
            OpenGLWindow {
                inner: window_context,
            },
            gl,
        )
    }

    #[inline]
    pub fn logical_size(&self) -> Vector2<f32> {
        let size = self.inner.window().inner_size();
        let scale_factor = self.inner.window().scale_factor() as f32;
        let width = size.width as f32 / scale_factor;
        let height = size.height as f32 / scale_factor;
        Vector2::new(width, height)
    }

    #[inline]
    pub fn physical_size(&self) -> Vector2<f32> {
        let size = self.inner.window().inner_size();
        Vector2::new(size.width as f32, size.height as f32)
    }

    /// Swaps the buffers in case of double or triple buffering. You should
    /// call this function every time you have finished rendering, or the
    /// image may not be displayed on the screen.
    #[inline]
    pub fn swap_buffers(&self) {
        self.inner.swap_buffers().unwrap();
    }

    pub fn set_title(&self, title: &str) {
        self.inner.window().set_title(title);
    }

    pub fn set_display_mode(&self, display_mode: DisplayMode) {
        match display_mode {
            DisplayMode::Windowed {
                width,
                height,
                resizable,
            } => {
                self.inner.window().set_inner_size(glutin::dpi::LogicalSize::new(width, height));
                self.inner.window().set_resizable(resizable);
                self.inner.window().set_fullscreen(None);
            }
            DisplayMode::WindowedFullscreen => {
                let fullscreen =
                    glutin::window::Fullscreen::Borderless(self.inner.window().primary_monitor());
                self.inner.window().set_fullscreen(Some(fullscreen));
            }
            DisplayMode::Fullscreen => {
                let fullscreen =
                    glutin::window::Fullscreen::Borderless(self.inner.window().primary_monitor());
                self.inner.window().set_fullscreen(Some(fullscreen));
            }
        }
    }

    pub fn set_vsync(&self, _vsync: Vsync) {
        // let setting = match vsync {
        //     Vsync::Disabled => 0,
        //     Vsync::Enabled => 1,
        //     Vsync::Adaptive => -1,
        // };
        // let result = unsafe { self.inner.set_swap_interval(setting) };
        // if result.is_err() {
        //     warn!("Failed to set vsync to {:?}", vsync);
        //     match vsync {
        //         Vsync::Disabled => warn!("Unable to configure vsync."),
        //         Vsync::Enabled => self.set_vsync(Vsync::Disabled),
        //         Vsync::Adaptive => self.set_vsync(Vsync::Enabled),
        //     }
        // } else {
        //     info!("Set vsync to {:?}", vsync);
        // }
    }
}
