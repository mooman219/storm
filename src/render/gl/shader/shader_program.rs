use crate::render::gl::raw::{resource, OpenGL, ShaderType};

pub struct ShaderProgram {
    gl: OpenGL,
    program: resource::Program,
}

impl ShaderProgram {
    pub fn new(gl: OpenGL, vertex_shader: &str, fragment_shader: &str) -> ShaderProgram {
        let vertex = gl.create_shader(ShaderType::Vertex);
        gl.shader_source(vertex, vertex_shader);
        gl.compile_shader(vertex);
        gl.check_shader(vertex).unwrap();

        let fragment = gl.create_shader(ShaderType::Fragment);
        gl.shader_source(fragment, fragment_shader);
        gl.compile_shader(fragment);
        gl.check_shader(fragment).unwrap();

        let program = gl.create_program();
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);
        gl.check_program(program).unwrap();

        gl.delete_shader(vertex);
        gl.delete_shader(fragment);

        ShaderProgram {
            gl,
            program,
        }
    }

    pub fn get_uniform_location(&self, uniform: &str) -> resource::UniformLocation {
        self.gl.get_uniform_location(self.program, uniform).unwrap()
    }

    pub fn bind(&self) {
        self.gl.use_program(Some(self.program));
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gl.delete_program(self.program);
    }
}
