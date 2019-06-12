use crate::render::gl::raw::bool_to_enum;
use gl;
use std::os::raw::c_void;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum AttributeType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Int = gl::INT,
    UnsignedInt = gl::UNSIGNED_INT,
    HalfFloat = gl::HALF_FLOAT,
    Float = gl::FLOAT,
    Double = gl::DOUBLE,
    Fixed = gl::FIXED,
    Int2_10_10_10_Rev = gl::INT_2_10_10_10_REV,
    UnsignedInt2_10_10_10_Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
    UnsignedInt10f_11f_11f_Rev = gl::UNSIGNED_INT_10F_11F_11F_REV,
}

/// Bind a vertex array object.
///
/// # Arguments
///
/// `name` - Specifies the name of the vertex array to bind.
#[inline]
pub fn bind_vertex_array(name: u32) {
    unsafe {
        gl::BindVertexArray(name);
    }
}

/// Generate a vertex array.
#[inline]
pub fn gen_vertex_array() -> u32 {
    unsafe {
        let mut name = 0u32;
        gl::GenVertexArrays(1, &mut name);
        name
    }
}

/// Delete a named vertex array.
///
/// # Arguments
///
/// `name` - Specifies the name of a vertex array to delete.
#[inline]
pub fn delete_vertex_array(name: u32) {
    unsafe {
        gl::DeleteVertexArrays(1, &name as *const _);
    }
}

/// Enable a generic vertex attribute array.
///
/// # Arguments
///
/// `index` - Specifies the index of the generic vertex attribute to be enabled or disabled.
#[inline]
pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

/// Disable a generic vertex attribute array.
///
/// # Arguments
///
/// `index` - Specifies the index of the generic vertex attribute to be enabled or disabled.
#[inline]
pub fn disable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::DisableVertexAttribArray(index);
    }
}

/// Define an array of generic vertex attribute data.
///
/// # Arguments
///
/// `index` - Specifies the index of the generic vertex attribute to be modified.
/// `size` - Specifies the number of components per generic vertex attribute.
/// `attribute` - Specifies the data type of each component in the array.
/// `normalized` - Specifies whether fixed-point data values should be normalized.
/// `stride` - Specifies the byte offset between consecutive generic vertex attributes. If stride is
/// 0, the generic vertex attributes are understood to be tightly packed in the array. The initial
/// value is 0. `pointer` - Specifies a offset of the first component of the first generic vertex
/// attribute in the array in the data store of the buffer currently bound to the array buffer
/// target. The initial value is 0.
#[inline]
pub fn vertex_attrib_pointer(
    index: u32,
    size: i32,
    attribute: AttributeType,
    normalized: bool,
    stride: i32,
    pointer: *const c_void,
) {
    unsafe {
        gl::VertexAttribPointer(index, size, attribute as u32, bool_to_enum(normalized), stride, pointer);
    }
}

/// Modify the rate at which generic vertex attributes advance during instanced rendering.
///
/// # Arguments
///
/// `index` - Specify the index of the generic vertex attribute.
/// `divisor` - Specify the number of instances that will pass between updates of the generic
/// attribute at slot index.
#[inline]
pub fn vertex_attrib_divisor(index: u32, divisor: u32) {
    unsafe {
        gl::VertexAttribDivisor(index, divisor);
    }
}
