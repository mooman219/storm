use storm::cgmath::*;
use storm::graphics::{
    DrawMode, ShaderDescription, VertexAttribute, VertexDescriptor, VertexInputType, VertexInstancing,
    VertexOutputType,
};

pub const TRIANGLE_SHADER: ShaderDescription = ShaderDescription {
    vertex_shader: include_str!("vertex.glsl"),
    fragment_shader: include_str!("fragment.glsl"),
    texture_names: &[],
    uniform_names: &["vertex"],
};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TrianglePoint {
    pub pos: Vector3<f32>,
    pub col: Vector3<f32>,
}

impl VertexDescriptor for TrianglePoint {
    const INSTANCING: VertexInstancing = VertexInstancing::none();
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
    ];
    const DRAW_MODE: DrawMode = DrawMode::Triangles;
}

impl Default for TrianglePoint {
    fn default() -> TrianglePoint {
        TrianglePoint {
            pos: Vector3::zero(),
            col: Vector3::zero(),
        }
    }
}
