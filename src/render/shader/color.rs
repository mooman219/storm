use cgmath::*;
use render::raw::*;
use render::shader::shader_program::*;

static VERTEX: &str = r#"
#version 400 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_explicit_attrib_location : enable

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec4 a_color;
out vec4 v_color;

layout(location = 0) uniform mat4 ortho;

void main() {
    gl_Position = ortho * vec4(a_pos, 1.0);
    v_color = a_color;
}
"#;
static FRAGMENT: &str = r#"
#version 400 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_explicit_attrib_location : enable

in vec4 v_color;
out vec4 a_color;

void main() {
    a_color = v_color;
}
"#;

const UNIFORM_ORTHO: i32 = 0;

pub struct ColorShader {
    program: ShaderProgram,
    dirty: bool,
    ortho: Matrix4<f32>,
    ortho_translation: Matrix4<f32>,
    ortho_scale: Matrix4<f32>,
}

impl ColorShader {
    pub fn new() -> ColorShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        ColorShader {
            program: program,
            dirty: true,
            ortho: ortho(0f32, 1f32, 0f32, 1f32, -1f32, 1f32),
            ortho_translation: Matrix4::from_translation(Vector3::new(0f32, 0f32, 0f32)),
            ortho_scale: Matrix4::from_scale(1f32),
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    pub fn set_bounds(&mut self, width: f32, height: f32) {
        self.dirty = true;
        let aspect_ratio = width / height;
        self.ortho = ortho(0f32, aspect_ratio, 0f32, 1f32, -1f32, 1f32);
    }

    pub fn set_translation(&mut self, translation: Vector2<f32>) {
        self.dirty = true;
        self.ortho_translation = Matrix4::from_translation(Vector3::new(translation.x, translation.y, 0f32));
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.dirty = true;
        self.ortho_scale = Matrix4::from_scale(scale);
    }

    pub fn sync(&mut self) {
        if self.dirty {
            self.dirty = false;
            self.program.bind();
            let matrix = self.ortho * self.ortho_translation * self.ortho_scale;
            uniform_matrix_4fv(UNIFORM_ORTHO, 1, false, matrix.as_ptr());
        }
    }
}
