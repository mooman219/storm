use super::layer::SharedLayer;
use super::raw::{resource, BlendFactor, Capability, CullFace, DepthTest, OpenGL, TextureUnit};
use super::shader;
use crate::utility::bad::UnsafeShared;
use cgmath::*;

pub struct OpenGLState {
    pub gl: OpenGL,
    layers: Vec<UnsafeShared<SharedLayer>>,
    program: resource::Program,
    uniform_ortho: resource::UniformLocation,
    uniform_texture: resource::UniformLocation,
}

impl OpenGLState {
    pub fn new(gl: OpenGL) -> OpenGLState {
        // Setup cabilities.
        gl.enable(Capability::CullFace);
        gl.enable(Capability::Blend);
        gl.enable(Capability::DepthTest);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.depth_func(DepthTest::Less);
        gl.blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        gl.cull_face(CullFace::Back);

        // Setup the shader.
        let program = gl.shader_program(shader::texture::VERTEX, shader::texture::FRAGMENT);
        let uniform_ortho = gl.get_uniform_location(program, "ortho").unwrap();
        let uniform_texture = gl.get_uniform_location(program, "tex[0]").unwrap();

        let mut state = OpenGLState {
            gl,
            layers: Vec::new(),
            program,
            uniform_ortho,
            uniform_texture,
        };

        // Bind and configure the shader.
        state.shader_bind();
        state.shader_texture(TextureUnit::Atlas);

        state
    }

    /// Subscribes the layer into getting state updates like resizes.
    pub fn layer_add(&mut self, mut layer: UnsafeShared<SharedLayer>) {
        layer.set_index(self.layers.len());
        self.layers.push(layer);
    }

    /// Unsubscribes the layer at the given index from getting state updates like resizes.
    pub fn layer_drop(&mut self, index: usize) {
        self.layers.swap_remove(index);
        if let Some(layer) = self.layers.get_mut(index) {
            layer.set_index(index);
        }
    }

    pub fn resize(&mut self, physical: &Vector2<f32>, ortho: &Matrix4<f32>) {
        self.gl.viewport(0, 0, physical.x as i32, physical.y as i32);
        for layer in &mut self.layers {
            layer.set_ortho(ortho);
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
