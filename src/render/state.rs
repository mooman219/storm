use super::raw::{BlendFactor, Capability, CullFace, DepthTest, OpenGL};
use super::shader::SpriteShader;
use super::window::OpenGLWindow;
use crate::math::ortho_from_bounds;
use crate::prelude::WindowSettings;
use crate::{colors, Image, Texture};
use cgmath::*;

#[no_mangle]
static mut STATE: Option<OpenGLState> = None;

pub struct OpenGLState {
    pub gl: OpenGL,
    matrix_ortho: Matrix4<f32>,
    logical_size: Vector2<f32>,
    default_texture: Option<Texture>,
    pub sprite: SpriteShader,
}

impl OpenGLState {
    pub fn init(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> OpenGLWindow {
        if unsafe { STATE.is_some() } {
            panic!("State already initialized");
        }
        let (window, gl) = OpenGLWindow::new(desc, event_loop);
        let mut gl = OpenGL::new(gl);
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        gl.clear_color(colors::BLACK);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        gl.cull_face(CullFace::Back);
        trace!("MAX_TEXTURE_SIZE: {}", gl.get_max_texture_size());

        let logical_size = window.logical_size();
        let sprite = SpriteShader::new(&mut gl);
        let state = OpenGLState {
            gl,
            matrix_ortho: ortho_from_bounds(&logical_size),
            logical_size,
            default_texture: None,
            sprite,
        };
        unsafe { STATE = Some(state) };
        window
    }

    pub fn ctx() -> &'static mut OpenGLState {
        unsafe {
            match STATE.as_mut() {
                Some(ctx) => ctx,
                None => panic!("State not initialized"),
            }
        }
    }

    pub fn logical_size(&self) -> Vector2<f32> {
        self.logical_size
    }

    pub fn ortho(&self) -> Matrix4<f32> {
        self.matrix_ortho
    }

    pub fn default_texture(&mut self) -> Texture {
        match &self.default_texture {
            Some(texture) => texture.clone(),
            None => {
                let texture = Texture::new(&Image::from_color(colors::WHITE, 2, 2));
                self.default_texture = Some(texture.clone());
                texture
            }
        }
    }

    pub fn resize(&mut self, physical: Vector2<f32>, logical: Vector2<f32>) {
        if self.logical_size != logical {
            trace!("Window resized: Physical({:?}) Logical({:?})", physical, logical);
            self.logical_size = logical;
            self.gl.viewport(0, 0, physical.x as i32, physical.y as i32);
            self.matrix_ortho = ortho_from_bounds(&logical);
        }
    }
}
