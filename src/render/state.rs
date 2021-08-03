use super::raw::{resource, BlendFactor, Capability, CullFace, DepthTest, OpenGL, TextureUnit};
use super::shader;
use super::window::OpenGLWindow;
use crate::colors;
use crate::math::ortho_from_bounds;
use crate::types::WindowSettings;
use cgmath::*;

#[no_mangle]
static mut STATE: Option<OpenGLState> = None;

pub struct OpenGLState {
    pub gl: OpenGL,
    matrix_ortho: Matrix4<f32>,
    logical_size: Vector2<f32>,
    program: resource::Program,
    uniform_ortho: resource::UniformLocation,
    uniform_texture: resource::UniformLocation,
}

impl OpenGLState {
    pub fn init(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> OpenGLWindow {
        if unsafe { STATE.is_some() } {
            panic!("State already initialized");
        }
        let (window, gl) = OpenGLWindow::new(desc, event_loop);
        let mut gl = OpenGL::new(gl);

        // Setup cabilities.
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        gl.clear_color(colors::BLACK);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        gl.cull_face(CullFace::Back);
        info!("MAX_TEXTURE_SIZE: {}", gl.get_max_texture_size());

        // Setup the shader.
        let program = gl.shader_program(shader::sprite::VERTEX, shader::sprite::FRAGMENT);
        let uniform_ortho = gl.get_uniform_location(program, "ortho").unwrap();
        let uniform_texture = gl.get_uniform_location(program, "tex[0]").unwrap();

        let logical_size = window.logical_size();

        let mut state = OpenGLState {
            gl,
            matrix_ortho: ortho_from_bounds(&logical_size),
            logical_size,
            program,
            uniform_ortho,
            uniform_texture,
        };

        // Bind and configure the shader.
        state.shader_bind();
        state.shader_texture(TextureUnit::Atlas);

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

    pub fn resize(&mut self, physical: Vector2<f32>, logical: Vector2<f32>) {
        if self.logical_size != logical {
            trace!("Window resized: Physical({:?}) Logical({:?})", physical, logical);
            self.logical_size = logical;
            self.gl.viewport(0, 0, physical.x as i32, physical.y as i32);
            self.matrix_ortho = ortho_from_bounds(&logical);
        }
    }

    /// Binds the shader.
    pub fn shader_bind(&mut self) {
        self.gl.use_program(Some(self.program));
    }

    /// Updates the ortho uniform in the shader.
    pub fn shader_ortho(&mut self, ortho: &Matrix4<f32>) {
        self.gl.uniform_matrix_4fv(Some(&self.uniform_ortho), false, ortho.as_ref());
    }

    /// Updates the texture uniform in the shader.
    pub fn shader_texture(&mut self, unit: TextureUnit) {
        let unit = (unit as u32 - TextureUnit::Atlas as u32) as i32;
        self.gl.uniform_1i(Some(&self.uniform_texture), unit);
    }
}

impl Drop for OpenGLState {
    fn drop(&mut self) {
        self.gl.delete_program(self.program);
    }
}
