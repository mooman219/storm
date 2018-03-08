use gl;
use std::ffi::CStr;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DrawMode {
    Points = gl::POINTS,
    LineStrip = gl::LINE_STRIP,
    LineLoop = gl::LINE_LOOP,
    Lines = gl::LINES,
    LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
    LinesAdjacency = gl::LINES_ADJACENCY,
    TriangleStrip = gl::TRIANGLE_STRIP,
    TriangleFan = gl::TRIANGLE_FAN,
    Triangles = gl::TRIANGLES,
    TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
    TrianglesAdjacency = gl::TRIANGLES_ADJACENCY,
    Patches = gl::PATCHES,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferType {
    ArrayBuffer = gl::ARRAY_BUFFER,
    AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = gl::COPY_READ_BUFFER,
    CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
    QueryBuffer = gl::QUERY_BUFFER,
    ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
    TextureBuffer = gl::TEXTURE_BUFFER,
    TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum GlString {
    Vendor = gl::VENDOR,
    Renderer = gl::RENDERER,
    Version = gl::VERSION,
    ShadingLanguageVersion = gl::SHADING_LANGUAGE_VERSION,
    Extensions = gl::EXTENSIONS,
}

impl GlString {
    pub fn get_string(self) -> String {
        unsafe {
            let data = CStr::from_ptr(gl::GetString(self as u32) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        }
    }
}
