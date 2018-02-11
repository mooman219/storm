pub mod draw_mode {
    use gl;

    pub const POINTS: u32 = gl::POINTS;
    pub const LINE_STRIP: u32 = gl::LINE_STRIP;
    pub const LINE_LOOP: u32 = gl::LINE_LOOP;
    pub const LINES: u32 = gl::LINES;
    pub const LINE_STRIP_ADJACENCY: u32 = gl::LINE_STRIP_ADJACENCY;
    pub const LINES_ADJACENCY: u32 = gl::LINES_ADJACENCY;
    pub const TRIANGLE_STRIP: u32 = gl::TRIANGLE_STRIP;
    pub const TRIANGLE_FAN: u32 = gl::TRIANGLE_FAN;
    pub const TRIANGLES: u32 = gl::TRIANGLES;
    pub const TRIANGLE_STRIP_ADJACENCY: u32 = gl::TRIANGLE_STRIP_ADJACENCY;
    pub const TRIANGLES_ADJACENCY: u32 = gl::TRIANGLES_ADJACENCY;
    pub const PATCHES: u32 = gl::PATCHES;
}

pub mod buffer_type {
    use gl;

    pub const ARRAY_BUFFER: u32 = gl::ARRAY_BUFFER;
    pub const ATOMIC_COUNTER_BUFFER: u32 = gl::ATOMIC_COUNTER_BUFFER;
    pub const COPY_READ_BUFFER: u32 = gl::COPY_READ_BUFFER;
    pub const COPY_WRITE_BUFFER: u32 = gl::COPY_WRITE_BUFFER;
    pub const DISPATCH_INDIRECT_BUFFER: u32 = gl::DISPATCH_INDIRECT_BUFFER;
    pub const DRAW_INDIRECT_BUFFER: u32 = gl::DRAW_INDIRECT_BUFFER;
    pub const ELEMENT_ARRAY_BUFFER: u32 = gl::ELEMENT_ARRAY_BUFFER;
    pub const PIXEL_PACK_BUFFER: u32 = gl::PIXEL_PACK_BUFFER;
    pub const PIXEL_UNPACK_BUFFER: u32 = gl::PIXEL_UNPACK_BUFFER;
    pub const QUERY_BUFFER: u32 = gl::QUERY_BUFFER;
    pub const SHADER_STORAGE_BUFFER: u32 = gl::SHADER_STORAGE_BUFFER;
    pub const TEXTURE_BUFFER: u32 = gl::TEXTURE_BUFFER;
    pub const TRANSFORM_FEEDBACK_BUFFER: u32 = gl::TRANSFORM_FEEDBACK_BUFFER;
    pub const UNIFORM_BUFFER: u32 = gl::UNIFORM_BUFFER;
}
