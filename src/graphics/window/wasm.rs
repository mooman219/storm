use crate::graphics::{DisplayMode, OpenGLWindowContract, WindowSettings};
use cgmath::*;
use log::info;
use wasm_bindgen::JsCast;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowExtWebSys;
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct OpenGLWindow {
    inner: Window,
}

impl OpenGLWindowContract for OpenGLWindow {
    fn new(desc: &WindowSettings, event_loop: &EventLoop<()>) -> (OpenGLWindow, glow::Context) {
        let mut builder = WindowBuilder::new().with_title(&desc.title);
        builder = match desc.display_mode {
            DisplayMode::Windowed {
                width,
                height,
                ..
            } => builder.with_inner_size(LogicalSize::new(width, height)),
            DisplayMode::WindowedFullscreen | DisplayMode::Fullscreen => {
                builder.with_fullscreen(Some(Fullscreen::Borderless(None)))
            }
        };
        let winit_window = builder.build(event_loop).expect("Window build");

        let canvas = winit_window.canvas();
        let webgl2_context = canvas
            .get_context("webgl2") // Result<Option<Object>, JsValue>
            .expect("Get webgl2 context A") // Option<Object>
            .expect("Get webgl2 context B") // Object
            .dyn_into::<web_sys::WebGl2RenderingContext>() // Result<WebGl2RenderingContext, Object>
            .expect("Get webgl2 context C"); // WebGl2RenderingContext
        let gl = glow::Context::from_webgl2_context(webgl2_context);

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.append_child(&canvas).expect("Append canvas to HTML body");

        let window = OpenGLWindow {
            inner: winit_window,
        };
        window.set_title(&desc.title);
        window.set_display_mode(desc.display_mode);
        info!("Created canvas.");

        (window, gl)
    }

    fn scale_factor(&self) -> f32 {
        self.inner.scale_factor() as f32
    }

    fn logical_size(&self) -> Vector2<f32> {
        let size = self.inner.inner_size();
        let scale_factor = self.inner.scale_factor() as f32;
        let size = Vector2::new(size.width as f32, size.height as f32);
        size / scale_factor
    }

    fn physical_size(&self) -> Vector2<f32> {
        let size = self.inner.inner_size();
        Vector2::new(size.width as f32, size.height as f32)
    }

    fn swap_buffers(&self) {
        // This is implicit on web.
    }

    fn set_title(&self, title: &str) {
        web_sys::window() // Option<Window>
            .unwrap() // Window
            .document() // Option<Document>
            .unwrap() // Document
            .set_title(title);
    }

    fn set_display_mode(&self, display_mode: DisplayMode) {
        match display_mode {
            DisplayMode::Windowed {
                width,
                height,
                ..
            } => {
                if let Some(_) = self.inner.fullscreen() {
                    self.inner.set_fullscreen(None);
                }
                self.inner.set_inner_size(LogicalSize::new(width, height));
            }
            DisplayMode::WindowedFullscreen | DisplayMode::Fullscreen => {
                self.inner.set_fullscreen(Some(Fullscreen::Borderless(None)));
            }
        }
    }
}
