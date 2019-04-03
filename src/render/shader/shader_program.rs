use gl;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_shader: &str, fragment_shader: &str) -> ShaderProgram {
        unsafe {
            // Compile the shaders
            let vertex_shader_id = compile_shader(vertex_shader, gl::VERTEX_SHADER);
            let fragment_shader_id = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);

            // Attach them to a program
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);

            // Delete the shaders post link
            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);

            // Error check
            check_link_status(program_id);

            // Return the program
            ShaderProgram { id: program_id }
        }
    }

    pub fn get_uniform_location(&self, uniform: &str) -> i32 {
        let c_uniform = CString::new(uniform).unwrap();
        unsafe { gl::GetUniformLocation(self.id, c_uniform.as_ptr()) }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn compile_shader(src: &str, shader_type: GLenum) -> u32 {
    unsafe {
        let c_src = CString::new(src).unwrap();
        let shader_id = gl::CreateShader(shader_type);

        // Attempt to compile the shader
        gl::ShaderSource(shader_id, 1, [c_src.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(shader_id);

        // Error check
        check_compile_status(shader_id);

        // Return the shader
        shader_id
    }
}

fn check_link_status(program_id: u32) {
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            // Subtract 1 to skip the trailing null character
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(program_id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!(
                "Link Status: {}",
                str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8",)
            );
        }
    }
}

fn check_compile_status(shader_id: u32) {
    unsafe {
        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader_id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!(
                "Compile Status: {}",
                str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8",)
            );
        }
    }
}
