use cgmath::*;
use gl;
use render::shader::*;

static VERTEX: &str = r#"
#version 400
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec4 a_uv;
out vec2 v_uv;

uniform mat4 ortho;

void main() {
    gl_Position = ortho * vec4(a_pos, 1.0);
    v_uv = a_uv;
}
"#;
static FRAGMENT: &str = r#"
#version 330
in vec2 v_uv;
out vec4 a_color;

uniform sampler2D sampler;

void main() {
    a_color = texture(sampler, v_uv).rgba;
}
"#;

pub struct TextureShader {
    program: ShaderProgram,
    uniform_ortho: i32,
    ortho: Matrix4<f32>,
    ortho_translation: Matrix4<f32>,
    ortho_scale: Matrix4<f32>,
}

impl TextureShader {
    pub fn new() -> TextureShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        let uniform_ortho = program.get_uniform_location("ortho");
        TextureShader {
            program: program,
            uniform_ortho: uniform_ortho,
            ortho: ortho(0f32, 1f32, 0f32, 1f32, -1f32, 1f32),
            ortho_translation: Matrix4::from_translation(Vector3::new(0f32, 0f32, 0f32)),
            ortho_scale: Matrix4::from_scale(1f32),
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    pub fn set_bounds(&mut self, width: f32, height: f32) {
        let aspect_ratio = width / height;
        self.ortho = ortho(0f32, aspect_ratio, 0f32, 1f32, -1f32, 1f32);
        self.sync();
    }

    pub fn set_translation(&mut self, translation: Vector2<f32>) {
        self.ortho_translation = Matrix4::from_translation(Vector3::new(translation.x, translation.y, 0f32));
        self.sync();
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.ortho_scale = Matrix4::from_scale(scale);
        self.sync();
    }

    fn sync(&self) {
        let matrix = self.ortho * self.ortho_translation * self.ortho_scale;
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ortho,          // Program location
                1,                           // Count
                gl::FALSE,                   // Should transpose
                matrix.as_ptr() as *const _, // Value pointer
            );
        }
    }
}
