use crate::color::RGBA8;
use crate::graphics::Texture;
use crate::prelude::WindowSettings;
use crate::render::raw::{BlendFactor, Capability, CullFace, DepthTest, OpenGL, PixelStoreAlignment};
use crate::render::window::OpenGLWindow;
use crate::Image;
use cgmath::*;

pub struct OpenGLState {
    gl: OpenGL,
    logical_size: Vector2<f32>,
    physical_size: Vector2<f32>,
    default_texture: Option<Texture>,
    max_texture_size: i32,
}

impl OpenGLState {
    pub fn init(
        desc: &WindowSettings,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> (OpenGLState, OpenGLWindow) {
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
            default_texture: None,
            max_texture_size,
        };
        (state, window)
    }

    #[inline(always)]
    pub fn gl(&mut self) -> &mut OpenGL {
        &mut self.gl
    }

    pub fn logical_size(&self) -> Vector2<f32> {
        self.logical_size
    }

    pub fn physical_size(&self) -> Vector2<f32> {
        self.physical_size
    }

    pub fn max_texture_size(&self) -> i32 {
        self.max_texture_size
    }

    pub fn default_texture(&mut self) -> Texture {
        match &self.default_texture {
            Some(texture) => texture.clone(),
            None => {
                let texture = Texture::from_image(&Image::from_color(RGBA8::WHITE, 1, 1));
                self.default_texture = Some(texture.clone());
                texture
            }
        }
    }

    pub fn resize(&mut self, physical: Vector2<f32>, logical: Vector2<f32>) {
        if self.logical_size != logical || self.physical_size != physical {
            trace!("Window resized: Physical({:?}) Logical({:?})", physical, logical);
            self.logical_size = logical;
            self.physical_size = physical;
            self.gl.viewport(0, 0, physical.x as i32, physical.y as i32);
        }
    }
}
