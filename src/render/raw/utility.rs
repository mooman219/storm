use gl;
use std::ffi::CStr;
use std::os::raw::c_void;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum StringTarget {
    Vendor = gl::VENDOR,
    Renderer = gl::RENDERER,
    Version = gl::VERSION,
    ShadingLanguageVersion = gl::SHADING_LANGUAGE_VERSION,
    Extensions = gl::EXTENSIONS,
}

/// Load each OpenGL symbol using a custom load function. This allows for the use of functions like glfwGetProcAddress or SDL_GL_GetProcAddress.
pub fn load_with<F>(loadfn: F)
where
    F: FnMut(&str) -> *const c_void,
{
    gl::load_with(loadfn);
}

/// Loads the associated string for the target.
///
/// # Arguments
///
/// `target` - Specify the target to load the string from.
pub fn get_string(target: StringTarget) -> String {
    unsafe {
        let data = CStr::from_ptr(gl::GetString(target as u32) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    }
}
