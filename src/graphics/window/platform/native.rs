use crate::graphics::{DisplayMode, OpenGLWindowContract, Vsync, WindowSettings};
use cgmath::*;
use glutin::ContextBuilder;
use log::info;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct OpenGLWindow {
    inner: glutin::ContextWrapper<glutin::PossiblyCurrent, Window>,
}

impl OpenGLWindowContract for OpenGLWindow {
    fn new(desc: &WindowSettings, event_loop: &EventLoop<()>) -> (OpenGLWindow, glow::Context) {
        let mut window_builder = WindowBuilder::new().with_title(&desc.title);
        match desc.display_mode {
            DisplayMode::Windowed {
                width,
                height,
                resizable,
            } => {
                window_builder =
                    window_builder.with_resizable(resizable).with_inner_size(LogicalSize::new(width, height))
            }
            DisplayMode::WindowedFullscreen | DisplayMode::Fullscreen => {
                let fullscreen = Fullscreen::Borderless(event_loop.primary_monitor());
                window_builder = window_builder.with_fullscreen(Some(fullscreen));
            }
        }
        let mut context_builder = ContextBuilder::new();
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

    fn scale_factor(&self) -> f32 {
        self.inner.window().scale_factor() as f32
    }

    fn logical_size(&self) -> Vector2<f32> {
        let size = self.inner.window().inner_size();
        let scale_factor = self.inner.window().scale_factor() as f32;
        let size = Vector2::new(size.width as f32, size.height as f32);
        size / scale_factor
    }

    fn physical_size(&self) -> Vector2<f32> {
        let size = self.inner.window().inner_size();
        Vector2::new(size.width as f32, size.height as f32)
    }

    fn set_cursor_grab(&self, grab: bool) {
        let _ = self.inner.window().set_cursor_grab(grab);
    }

    fn set_cursor_visible(&self, visible: bool) {
        let _ = self.inner.window().set_cursor_visible(visible);
    }

    fn swap_buffers(&self) {
        self.inner.swap_buffers().unwrap();
    }

    fn set_title(&self, title: &str) {
        self.inner.window().set_title(title);
    }

    fn set_display_mode(&self, display_mode: DisplayMode) {
        match display_mode {
            DisplayMode::Windowed {
                width,
                height,
                resizable,
            } => {
                self.inner.window().set_inner_size(LogicalSize::new(width, height));
                self.inner.window().set_resizable(resizable);
                self.inner.window().set_fullscreen(None);
            }
            DisplayMode::WindowedFullscreen | DisplayMode::Fullscreen => {
                let fullscreen = Fullscreen::Borderless(self.inner.window().current_monitor());
                self.inner.window().set_fullscreen(Some(fullscreen));
            }
        }
    }
}
