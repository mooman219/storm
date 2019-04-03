use cgmath::*;
use render::raw::*;
use render::shader::shader_program::*;

static VERTEX: &str = r#"
#version 400 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_explicit_attrib_location : enable

layout(location = 0) in float a_pos_z;
layout(location = 1) in vec4 a_pos;
layout(location = 2) in vec4 a_uv;
layout(location = 3) in vec4 a_color;
out vec2 v_uv;
out vec4 v_color;

layout(location = 1) uniform mat4 ortho;

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
#version 400 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_explicit_attrib_location : enable

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

layout(location = 0) uniform sampler2D atlas;

void main() {
    a_color = texture(atlas, v_uv) * v_color;
}
"#;

const UNIFORM_ORTHO: i32 = 1;
const UNIFORM_ATLAS: i32 = 0;

pub struct TextureShader {
    program: ShaderProgram,
    // todo: Precompute the ortho multiplication.
    ortho: Matrix4<f32>,
    ortho_translation: Matrix4<f32>,
    ortho_scale: Matrix4<f32>,
    atlas: i32,
}

impl TextureShader {
    pub fn new() -> TextureShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        TextureShader {
            program: program,
            ortho: ortho(-2.5f32, 2.5f32, -2.5f32, 2.5f32, -1f32, 1f32),
            ortho_translation: Matrix4::from_translation(Vector3::new(0f32, 0f32, 0f32)),
            ortho_scale: Matrix4::from_scale(1f32),
            atlas: TextureUnit::Atlas as i32,
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    pub fn set_bounds(&mut self, width: f32, height: f32) {
        let nw = width / 200f32;
        let nh = height / 200f32;
        self.ortho = ortho(-nw, nw, -nh, nh, -1f32, 1f32);
    }

    pub fn set_translation(&mut self, translation: Vector2<f32>) {
        self.ortho_translation = Matrix4::from_translation(Vector3::new(translation.x, translation.y, 0f32));
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.ortho_scale = Matrix4::from_scale(scale);
    }

    pub fn set_texture_unit(&mut self, unit: TextureUnit) {
        self.atlas = unit as i32;
    }

    pub fn sync_ortho(&mut self) {
        let matrix = self.ortho * self.ortho_translation * self.ortho_scale;
        uniform_matrix_4fv(UNIFORM_ORTHO, 1, false, matrix.as_ptr());
    }

    pub fn sync_atlas(&mut self) {
        uniform_1i(UNIFORM_ATLAS, self.atlas);
    }
}
