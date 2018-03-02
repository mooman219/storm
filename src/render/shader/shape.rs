use cgmath::*;
use gl;
use render::shader::*;

static VERTEX: &str = r#"
    #version 400
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec4 a_color;
    out vec4 v_color;

    uniform mat4 ortho;

    void main() {
        gl_Position = ortho * vec4(a_pos, 0.0, 1.0);
        v_color = a_color;
    }
"#;
static FRAGMENT: &str = r#"
    #version 330
    in vec4 v_color;
    out vec4 a_color;

    void main() {
        a_color = v_color;
    }
"#;

pub struct ShapeShader {
    program: ShaderProgram,
    uniform_ortho: i32,
    ortho_translation: Matrix4<f32>,
    ortho_scale: Matrix4<f32>,
}

impl ShapeShader {
    pub fn new() -> ShapeShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        let uniform_ortho = program.get_uniform_location("ortho");
        println!("Render: Shape ortho [{}]", uniform_ortho);
        ShapeShader {
            program: program,
            uniform_ortho: uniform_ortho,
            ortho_translation: Matrix4::from_translation(Vector3::new(0f32, 0f32, 0f32)),
            ortho_scale: Matrix4::from_scale(1f32),
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.ortho_translation = Matrix4::from_translation(translation);
        self.sync();
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.ortho_scale = Matrix4::from_scale(scale);
        self.sync();
    }

    fn sync(&self) {
        let matrix = self.ortho_translation * self.ortho_scale;
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
