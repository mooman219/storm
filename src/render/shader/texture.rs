use cgmath::*;
use render::raw::*;
use render::shader::shader_program::*;

static VERTEX: &str = r#"
#version 330

layout(location = 0) in float a_pos_z;
layout(location = 1) in vec4 a_pos;
layout(location = 2) in vec4 a_uv;
layout(location = 3) in vec4 a_color;
out vec2 v_uv;
out vec4 v_color;

uniform mat4 ortho;

void main() {
    // (x:left, y:right, z:bottom, w:top)
    vec2 pos = vec2(0.0);

    switch (gl_VertexID) {
        case 0:
            pos = vec2(a_pos.x, a_pos.w); // left top
            v_uv = vec2(a_uv.x, a_uv.z); // left bottom
            break;
        case 1:
            pos = vec2(a_pos.x, a_pos.z); // left bottom
            v_uv = vec2(a_uv.x, a_uv.w); // left top
            break;
        case 2:
            pos = vec2(a_pos.y, a_pos.w); // right top
            v_uv = vec2(a_uv.y, a_uv.z); /// right bottom
            break;
        case 3:
            pos = vec2(a_pos.y, a_pos.z); // right bottom
            v_uv = vec2(a_uv.y, a_uv.w); // right top
            break;
    }

    v_color = a_color;
    gl_Position = ortho * vec4(pos, a_pos_z, 1.0);
}
"#;
static FRAGMENT: &str = r#"
#version 330

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

uniform sampler2D tex;

void main() {
    vec4 color = texture(tex, v_uv) * v_color;
    if (color.w <= 0.0) {
        discard;
    }
    a_color = color;
}
"#;

pub struct TextureShader {
    program: ShaderProgram,
    uniform_ortho: i32,
    uniform_texture: i32,
}

impl TextureShader {
    pub fn new() -> TextureShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        let uniform_ortho = program.get_uniform_location("ortho");
        let uniform_texture = program.get_uniform_location("tex");
        TextureShader {
            program: program,
            uniform_ortho: uniform_ortho,
            uniform_texture: uniform_texture,
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    /// Updates the ortho uniform in the shader.
    pub fn ortho(&self, matrix: Matrix4<f32>) {
        uniform_matrix_4fv(self.uniform_ortho, 1, false, matrix.as_ptr());
    }

    /// Updates the texture uniform in the shader.
    pub fn texture(&self, unit: TextureUnit) {
        let unit = (unit as u32 - TextureUnit::Atlas as u32) as i32;
        uniform_1i(self.uniform_texture, unit);
    }
}
