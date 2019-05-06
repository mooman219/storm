use cgmath::*;
use render::raw::*;
use render::shader::shader_program::*;

static VERTEX: &str = r#"
#version 330

const float PI     = 3.141592653589793238462643383279;
const float TWO_PI = 6.283185307179586476925286766559;

layout(location = 0) in float a_pos_z;
layout(location = 1) in vec4 a_pos;
layout(location = 2) in vec4 a_uv;
layout(location = 3) in vec4 a_color;
layout(location = 4) in float a_rotation;
out vec2 v_uv;
out vec4 v_color;

uniform mat4 ortho;

mat4 rotateZ(float psi){
    return mat4(
        vec4(cos(psi),-sin(psi),0.,0),
        vec4(sin(psi),cos(psi),0.,0.),
        vec4(0.,0.,1.,0.),
        vec4(0.,0.,0.,1.));
}

void main() {
    // (x:left, y:right, z:bottom, w:top)
    vec2 pos[4];
    vec2 uv[4];
    pos[0] = vec2(a_pos.x, a_pos.w); // left top
    pos[1] = vec2(a_pos.x, a_pos.z); // left bottom
    pos[2] = vec2(a_pos.y, a_pos.w); // right top
    pos[3] = vec2(a_pos.y, a_pos.z); // right bottom
    uv[0] = vec2(a_uv.x, a_uv.z); // left bottom
    uv[1] = vec2(a_uv.x, a_uv.w); // left top
    uv[2] = vec2(a_uv.y, a_uv.z); // right bottom
    uv[3] = vec2(a_uv.y, a_uv.w); // right top
    v_uv = uv[gl_VertexID];
    v_color = a_color;
    gl_Position = ortho * rotateZ(TWO_PI * a_rotation) * vec4(pos[gl_VertexID], a_pos_z, 1.0);
}
"#;
static FRAGMENT: &str = r#"
#version 330

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

uniform sampler2D tex;

void main() {
    a_color = texture(tex, v_uv) * v_color;
    if (a_color.a <= 0.0) {
        discard;
    }
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
