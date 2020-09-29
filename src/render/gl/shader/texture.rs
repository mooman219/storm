use crate::render::gl::raw::{resource, OpenGL, TextureUnit};
use crate::render::gl::shader::shader_program::*;
use cgmath::*;

static VERTEX: &str = r#"
#version 330

const float PI     = 3.141592653589793238462643383279;
const float TWO_PI = 6.283185307179586476925286766559;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_size;
layout(location = 2) in vec4 a_uv;
layout(location = 3) in vec4 a_color;
layout(location = 4) in float a_rotation;

out vec2 v_uv;
out vec4 v_color;

uniform mat4 ortho;

// UV Layout: xmin xmax ymin ymax
// ymin and ymax are swapped below because OpenGL reads images from bottom row to top row, but
// they're stored top to bottom on upload, so this corrects that.
uniform vec4 uv_lut[4] = vec4[4](
    vec4(1.0, 0.0, 1.0, 0.0), // left bottom
    vec4(1.0, 0.0, 0.0, 1.0), // left top
    vec4(0.0, 1.0, 1.0, 0.0), // right bottom
    vec4(0.0, 1.0, 0.0, 1.0)); // right top

uniform vec2 pos_lut[4] = vec2[4](
    vec2(0.0, 65536.0),     // left top
    vec2(0.0, 0.0),         // left bottom
    vec2(65536.0, 65536.0), // right top
    vec2(65536.0, 0.0));    // right bottom

vec4 rotateZ(vec3 pos) {
    float psi = TWO_PI * a_rotation;
    vec2 origin = vec2(
        a_pos.x + (a_size.x * 32768.0),
        a_pos.y + (a_size.y * 32768.0));
    return vec4(
        (cos(psi) * (pos.x - origin.x)) - (sin(psi) * (pos.y - origin.y)) + origin.x,
        (sin(psi) * (pos.x - origin.x)) + (cos(psi) * (pos.y - origin.y)) + origin.y,
        pos.z,
        1.0);
}

void main() {
    vec4 temp = a_uv * uv_lut[gl_VertexID];
    v_uv = vec2(temp.x + temp.y, temp.z + temp.w);
    v_color = a_color;

    vec3 size = vec3(a_size * pos_lut[gl_VertexID], 0.0);
    vec3 pos = a_pos + size;
    gl_Position = ortho * rotateZ(pos);
}
"#;
static FRAGMENT: &str = r#"
#version 330

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

uniform sampler2D tex[1];

void main() {
    a_color = texture(tex[0], v_uv) * v_color;
    if (a_color.a <= 0.0) {
        discard;
    }
}
"#;

pub struct TextureShader {
    gl: OpenGL,
    program: ShaderProgram,
    uniform_ortho: resource::UniformLocation,
    uniform_texture: resource::UniformLocation,
}

impl TextureShader {
    pub fn new(gl: OpenGL) -> TextureShader {
        let program = ShaderProgram::new(gl.clone(), VERTEX, FRAGMENT);
        let uniform_ortho = program.get_uniform_location("ortho");
        let uniform_texture = program.get_uniform_location("tex[0]");
        TextureShader {
            gl,
            program,
            uniform_ortho,
            uniform_texture,
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    /// Updates the ortho uniform in the shader.
    pub fn ortho(&self, matrix: &Matrix4<f32>) {
        self.gl.uniform_matrix_4fv(Some(&self.uniform_ortho), false, matrix.as_ref());
    }

    /// Updates the texture uniform in the shader.
    pub fn texture(&self, unit: TextureUnit) {
        let unit = (unit as u32 - TextureUnit::Atlas as u32) as i32;
        self.gl.uniform_1i(Some(&self.uniform_texture), unit);
    }
}
