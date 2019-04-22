use gl;
use std::os::raw::c_void;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferBindingTarget {
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
pub enum BufferUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

/// Creates and initializes a buffer object's data store.
///
/// # Arguments
///
/// `target` - Specifies the target to which the buffer object is bound.
/// `size` - Specifies the size in bytes of the buffer object's new data store.
/// `data` - Specifies a pointer to data that will be copied into the data store for initialization,
/// or NULL if no data is to be copied. `usage` - Bitwise OR of masks that indicate the buffers to
/// be cleared. The three masks are GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, and
/// GL_STENCIL_BUFFER_BIT.
#[inline]
pub fn buffer_data(target: BufferBindingTarget, size: isize, data: *const c_void, usage: BufferUsage) {
    unsafe {
        gl::BufferData(target as u32, size, data, usage as u32);
    }
}

/// Updates a subset of a buffer object's data store.
///
/// # Arguments
///
/// `target` - Specifies the target to which the buffer object is bound.
/// `offset` - Specifies the offset into the buffer object's data store where data replacement will
/// begin, measured in bytes. `size` - Specifies the size in bytes of the buffer object's new data
/// store. `data` - Specifies a pointer to data that will be copied into the data store for
/// initialization, or NULL if no data is to be copied.
#[inline]
pub fn buffer_sub_data(target: BufferBindingTarget, offset: isize, size: isize, data: *const c_void) {
    unsafe {
        gl::BufferSubData(target as u32, offset, size, data);
    }
}

/// Creates and initializes a buffer object's data store.
///
/// # Arguments
///
/// `target` - Specifies the target to which the buffer object is bound.
/// `buffer` - Specifies the name of a buffer object.
#[inline]
pub fn bind_buffer(target: BufferBindingTarget, buffer: u32) {
    unsafe {
        gl::BindBuffer(target as u32, buffer);
    }
}

/// Generate a buffer object.
#[inline]
pub fn gen_buffer() -> u32 {
    unsafe {
        let mut name = 0u32;
        gl::GenBuffers(1, &mut name);
        name
    }
}

/// Delete a named buffer object.
///
/// # Arguments
/// `buffer` - Specifies a buffer to be deleted.
#[inline]
pub fn delete_buffer(buffer: u32) {
    unsafe {
        gl::DeleteBuffers(1, &buffer as *const _);
    }
}
