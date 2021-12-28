use crate::render::{AttributeType, OpenGL};
use log::trace;

/// A trait to describe vertices that will be consumed by a shader.
///
/// # Example
/// ```
/// // This is an example for how to implement VertexDescriptor for a simple type.
/// use storm::cgmath::*;
/// use storm::graphics::*;
///
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// struct Demo {
///     pos: Vector3<f32>,
///     size: Vector2<u16>,
/// }
///
/// impl VertexDescriptor for Demo {
///     const ATTRIBUTES: &'static [VertexAttribute] = &[
///         // This value represents the three f32s in pos's Vector3<f32>. When invoked in the
///         // shader, the values will be read as f32s.
///         VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
///         // This value represents the two u16s in size's Vector3<u16>. When invoked in the
///         // shader, the values will be read as f32s.
///         VertexAttribute::new(2, VertexInputType::U16, VertexOutputType::F32),
///     ];
/// }
/// ```
pub trait VertexDescriptor {
    const ATTRIBUTES: &'static [VertexAttribute];
}

/// Describes an individual vertex attribute. These usually correspond to fields in a struct.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VertexAttribute {
    /// Specifies the number of components per generic vertex attribute
    pub count: i32,
    /// Specifies the data type of each component in the array.
    pub input: VertexInputType,
    /// Specifies the output conversion in the shader.
    pub output: VertexOutputType,
}

impl VertexAttribute {
    /// Helper function to create a new vertex attribute.
    pub const fn new(count: i32, input: VertexInputType, output: VertexOutputType) -> VertexAttribute {
        VertexAttribute {
            count,
            input,
            output,
        }
    }
}

/// The output format a vertice will be converted into.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VertexOutputType {
    integer: bool,
    normalized: bool,
}

#[allow(non_upper_case_globals)]
impl VertexOutputType {
    /// F32 specifies the input be converted into a f32, normalizing in the process. Signed values
    /// are normalized into [-1, 1], and unsigned values are normalized into [0, 1]. For example, if
    /// the input is a u16 with the value of u16::MAX / 2, it will be converted into 0.5.
    pub const NormalizedF32: VertexOutputType = VertexOutputType {
        integer: false,
        normalized: true,
    };
    /// F32 specifies the input be converted into a f32. For example, if the input is a u32 with the
    /// value of 10, it will be converted into 10.0.
    pub const F32: VertexOutputType = VertexOutputType {
        integer: false,
        normalized: false,
    };
    /// I32 specifies the input be converted into a i32. For example, if the input is a f32 with the
    /// value of 10.1, it will be converted into 10.
    pub const I32: VertexOutputType = VertexOutputType {
        integer: true,
        normalized: false,
    };
}

/// The input format a vertice will be converted from.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VertexInputType {
    size: i32,
    format: AttributeType,
}

#[allow(non_upper_case_globals)]
impl VertexInputType {
    /// I8 specifies the input is an i8.
    pub const I8: VertexInputType = VertexInputType {
        size: 1,
        format: AttributeType::Byte,
    };
    /// U8 specifies the input is an u8.
    pub const U8: VertexInputType = VertexInputType {
        size: 1,
        format: AttributeType::UnsignedByte,
    };
    /// I16 specifies the input is an i16.
    pub const I16: VertexInputType = VertexInputType {
        size: 2,
        format: AttributeType::Short,
    };
    /// U16 specifies the input is an u16.
    pub const U16: VertexInputType = VertexInputType {
        size: 2,
        format: AttributeType::UnsignedShort,
    };
    /// I32 specifies the input is an i32.
    pub const I32: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::Int,
    };
    /// U32 specifies the input is an u32.
    pub const U32: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::UnsignedInt,
    };
    /// F16 specifies the input is a f16 (A f32 with half percision).
    pub const F16: VertexInputType = VertexInputType {
        size: 2,
        format: AttributeType::HalfFloat,
    };
    /// F32 specifies the input is an f32.
    pub const F32: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::Float,
    };
    /// F64 specifies the input is an f64.
    pub const F64: VertexInputType = VertexInputType {
        size: 8,
        format: AttributeType::Double,
    };
    pub const Fixed: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::Fixed,
    };
    pub const Int2_10_10_10_rev: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::Int2_10_10_10_Rev,
    };
    pub const UnsignedInt2_10_10_10_Rev: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::UnsignedInt2_10_10_10_Rev,
    };
    pub const UnsignedInt10F_11f_11f_rev: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::UnsignedInt10f_11f_11f_Rev,
    };
}

pub(crate) fn configure_vertex<T: VertexDescriptor + Copy>(attributes: &[VertexAttribute], gl: &OpenGL) {
    let stride = core::mem::size_of::<T>() as i32;
    let mut index = 0;
    let mut size = 0;
    for attribute in attributes {
        gl.enable_vertex_attrib_array(index);
        gl.vertex_attrib_divisor(index, 1);
        if attribute.output.integer {
            gl.vertex_attrib_pointer_i32(index, attribute.count, attribute.input.format, stride, size);
        } else {
            gl.vertex_attrib_pointer_f32(
                index,
                attribute.count,
                attribute.input.format,
                attribute.output.normalized,
                stride,
                size,
            );
        }
        size += attribute.count * attribute.input.size;
        index += 1;
    }
    trace!("Configured vertex {}: Size {}, Stride: {}", core::any::type_name::<T>(), size, stride);
}
