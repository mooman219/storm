use cgmath::*;
use render::gl::raw::*;
use render::gl::shader::*;
use render::gl::texture_handle::*;
use render::texture::*;
use render::Window;

pub struct OpenGLState {
    shader: TextureShader,
    texture_atlas: TextureHandle,
    texture_font: TextureHandle,
}

impl OpenGLState {
    pub fn new(window: Window) -> OpenGLState {
        let state = OpenGLState {
            shader: TextureShader::new(),
            texture_atlas: TextureHandle::new(TextureUnit::Atlas),
            texture_font: TextureHandle::new(TextureUnit::Font),
        };
        state.setup();
        state
    }

    fn setup(&mut self) {
        // Bind shader once.
        self.shader.bind();
        // Setup cabilities.
        enable(Capability::CullFace);
        enable(Capability::Blend);
        enable(Capability::DepthTest);
        clear_color(1.0, 1.0, 1.0, 1.0);
        depth_func(DepthTest::Less);
        blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        cull_face(CullFace::Back);
    }

    pub fn shader_ortho(&mut self, matrix: &Matrix4<f32>) {
        self.shader.ortho(matrix);
    }

    pub fn shader_texture_atlas(&mut self) {
        self.shader.texture(TextureUnit::Atlas);
    }

    pub fn shader_texture_font(&mut self) {
        self.shader.texture(TextureUnit::Font);
    }

    pub fn texture_upload_atlas(&mut self, texture: &Texture) {
        self.texture_atlas.set_texture(texture);
    }

    pub fn texture_upload_font(&mut self, texture: &Texture) {
        self.texture_font.set_texture(texture);
    }

    pub fn resize(&mut self, size: Vector2<i32>) {
        viewport(0, 0, size.x as i32, size.y as i32);
    }
}
