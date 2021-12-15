use crate::render::raw::{AttributeType, OpenGL};

/// A trait to describe vertices that will be consumed by a shader.
pub trait VertexDescriptor {
    const ATTRIBUTES: &'static [VertexAttribute];
}

/// Describes a vertex attribute.
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
    pub const NormalizedF32: VertexOutputType = VertexOutputType {
        integer: false,
        normalized: true,
    };
    pub const F32: VertexOutputType = VertexOutputType {
        integer: false,
        normalized: false,
    };
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
    pub const I8: VertexInputType = VertexInputType {
        size: 1,
        format: AttributeType::Byte,
    };
    pub const U8: VertexInputType = VertexInputType {
        size: 1,
        format: AttributeType::UnsignedByte,
    };
    pub const I16: VertexInputType = VertexInputType {
        size: 2,
        format: AttributeType::Short,
    };
    pub const U16: VertexInputType = VertexInputType {
        size: 2,
        format: AttributeType::UnsignedShort,
    };
    pub const I32: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::Int,
    };
    pub const U32: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::UnsignedInt,
    };
    pub const F16: VertexInputType = VertexInputType {
        size: 2,
        format: AttributeType::HalfFloat,
    };
    pub const F32: VertexInputType = VertexInputType {
        size: 4,
        format: AttributeType::Float,
    };
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
