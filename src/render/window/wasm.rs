use crate::types::*;
use cgmath::*;
use wasm_bindgen::JsCast;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowExtWebSys;
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct OpenGLWindow {
    inner: Window,
}

impl OpenGLWindow {
    pub fn new(desc: &WindowSettings, event_loop: &EventLoop<()>) -> (OpenGLWindow, glow::Context) {
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
            DisplayMode::WindowedFullscreen => {
                let fullscreen = Fullscreen::Borderless(event_loop.primary_monitor());
                window_builder = window_builder.with_fullscreen(Some(fullscreen));
            }
            DisplayMode::Fullscreen => {
                let fullscreen = Fullscreen::Borderless(event_loop.primary_monitor());
                window_builder = window_builder.with_fullscreen(Some(fullscreen));
            }
        }
        let winit_window = window_builder.build(event_loop).expect("Window build");
        let canvas = winit_window.canvas();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let webgl2_context = canvas
            .get_context("webgl2")
            .expect("Get webgl2 context A")
            .expect("Get webgl2 context B")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Get webgl2 context C");
        let gl = glow::Context::from_webgl2_context(webgl2_context);
        document.set_title(&desc.title);
        body.append_child(&canvas).expect("Append canvas to HTML body");

        (
            OpenGLWindow {
                inner: winit_window,
            },
            gl,
        )
    }
    pub fn logical_size(&self) -> Vector2<f32> {
        let size = self.inner.inner_size();
        let scale_factor = self.inner.scale_factor() as f32;
        let width = size.width as f32 / scale_factor;
        let height = size.height as f32 / scale_factor;
        Vector2::new(width, height)
    }
    pub fn physical_size(&self) -> Vector2<f32> {
        let size = self.inner.inner_size();
        Vector2::new(size.width as f32, size.height as f32)
    }
    pub fn swap_buffers(&self) {
        // This is implicit on web.
    }
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
        web_sys::window().unwrap().document().unwrap().set_title(title);
    }
    pub fn set_display_mode(&self, _display_mode: DisplayMode) {}
}
