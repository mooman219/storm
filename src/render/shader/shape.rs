use render::shader::*;
use cgmath::*;
use gl;
use render::color::*;
use render::vertex::shape::*;

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
}

impl ShapeShader {
    pub fn new() -> ShapeShader {
        let program = ShaderProgram::new(VERTEX, FRAGMENT);
        let uniform_ortho = program.get_uniform_location("ortho");
        println!("Render: Shape ortho [{}]", uniform_ortho);
        ShapeShader {
            program: program,
            uniform_ortho: uniform_ortho,
        }
    }

    pub fn bind(&self) {
        self.program.bind();
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        let matrix = Matrix4::from_translation(translation);
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
