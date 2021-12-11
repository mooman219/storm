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
    /// Specifies whether fixed-point data values should be normalized (true) or converted directly
    /// as fixed-point values (false) when they are accessed.
    pub normalized: bool,
    /// Specifies the data type of each component in the array.
    pub input: VertexInputFormat,
    /// Specifies the output conversion in the shader.
    pub output: VertexOutputFormat,
}

/// The output format a vertice will be converted into.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VertexOutputFormat {
    Float,
    Integer,
}

/// The input format a vertice will be converted from.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VertexInputFormat {
    size: i32,
    format: AttributeType,
}

#[allow(non_upper_case_globals)]
impl VertexInputFormat {
    pub const Byte: VertexInputFormat = VertexInputFormat {
        size: 1,
        format: AttributeType::Byte,
    };
    pub const UnsignedByte: VertexInputFormat = VertexInputFormat {
        size: 1,
        format: AttributeType::UnsignedByte,
    };
    pub const Short: VertexInputFormat = VertexInputFormat {
        size: 2,
        format: AttributeType::Short,
    };
    pub const UnsignedShort: VertexInputFormat = VertexInputFormat {
        size: 2,
        format: AttributeType::UnsignedShort,
    };
    pub const Integer: VertexInputFormat = VertexInputFormat {
        size: 4,
        format: AttributeType::Int,
    };
    pub const UnsignedInteger: VertexInputFormat = VertexInputFormat {
        size: 4,
        format: AttributeType::UnsignedInt,
    };
    pub const HalfFloat: VertexInputFormat = VertexInputFormat {
        size: 2,
        format: AttributeType::HalfFloat,
    };
    pub const Float: VertexInputFormat = VertexInputFormat {
        size: 4,
        format: AttributeType::Float,
    };
    pub const Double: VertexInputFormat = VertexInputFormat {
        size: 8,
        format: AttributeType::Double,
    };
    pub const Fixed: VertexInputFormat = VertexInputFormat {
        size: 4,
        format: AttributeType::Fixed,
    };
    pub const Int2_10_10_10_rev: VertexInputFormat = VertexInputFormat {
        size: 4,
        format: AttributeType::Int2_10_10_10_Rev,
    };
    pub const UnsignedInt2_10_10_10_Rev: VertexInputFormat = VertexInputFormat {
        size: 4,
        format: AttributeType::UnsignedInt2_10_10_10_Rev,
    };
    pub const UnsignedInt10F_11f_11f_rev: VertexInputFormat = VertexInputFormat {
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
        match attribute.output {
            VertexOutputFormat::Float => gl.vertex_attrib_pointer_f32(
                index,
                attribute.count,
                attribute.input.format,
                attribute.normalized,
                stride,
                size,
            ),
            VertexOutputFormat::Integer => {
                gl.vertex_attrib_pointer_i32(index, attribute.count, attribute.input.format, stride, size)
            }
        }
        size += attribute.count * attribute.input.size;
        index += 1;
    }
    trace!("Configured vertex {}: Size {}, Stride: {}", core::any::type_name::<T>(), size, stride);
}
