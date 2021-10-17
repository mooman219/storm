use crate::math::ortho_from_bounds;
use crate::prelude::WindowSettings;
use crate::render::raw::{BlendFactor, Capability, CullFace, DepthTest, OpenGL, PixelStoreAlignment};
use crate::render::shader::{SpriteShader, TextShader};
use crate::render::window::OpenGLWindow;
use crate::{Image, Texture, RGBA8};
use cgmath::*;

#[no_mangle]
static mut STATE: Option<OpenGLState> = None;

pub struct OpenGLState {
    pub gl: OpenGL,
    matrix_ortho: Matrix4<f32>,
    logical_size: Vector2<f32>,
    default_texture: Option<Texture<RGBA8>>,
    max_texture_size: i32,
    pub sprite: SpriteShader,
    pub text: TextShader,
}

impl OpenGLState {
    pub fn init(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> OpenGLWindow {
        if unsafe { STATE.is_some() } {
            panic!("State already initialized");
        }
        let (window, gl) = OpenGLWindow::new(desc, event_loop);
        let mut gl = OpenGL::new(gl);
        let max_texture_size = gl.get_max_texture_size();
        gl.pixel_store(PixelStoreAlignment::UnpackAlignment, 1);
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        gl.clear_color(RGBA8::BLACK);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        gl.cull_face(CullFace::Back);
        trace!("MAX_TEXTURE_SIZE: {}", max_texture_size);

        let logical_size = window.logical_size();
        let sprite = SpriteShader::new(&mut gl);
        let text = TextShader::new(&mut gl);
        let state = OpenGLState {
            gl,
            matrix_ortho: ortho_from_bounds(&logical_size),
            logical_size,
            default_texture: None,
            max_texture_size,
            sprite,
            text,
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

    pub fn max_texture_size(&self) -> i32 {
        self.max_texture_size
    }

    pub fn default_texture(&mut self) -> Texture<RGBA8> {
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
        if self.logical_size != logical {
            trace!("Window resized: Physical({:?}) Logical({:?})", physical, logical);
            self.logical_size = logical;
            self.gl.viewport(0, 0, physical.x as i32, physical.y as i32);
            self.matrix_ortho = ortho_from_bounds(&logical);
        }
    }
}
