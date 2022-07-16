use storm::cgmath::*;
use storm::graphics::{
    DrawMode, VertexAttribute, VertexDescriptor, VertexInputType, VertexInstancing, VertexOutputType,
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
    const DRAW_MODE: DrawMode = DrawMode::TriangleStrip;
}

impl Default for TrianglePoint {
    fn default() -> TrianglePoint {
        TrianglePoint {
            pos: Vector3::zero(),
            col: Vector3::zero(),
        }
    }
}
