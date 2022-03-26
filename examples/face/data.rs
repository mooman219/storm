use storm::cgmath::*;
use storm::graphics::{TextureSection, VertexAttribute, VertexDescriptor, VertexInputType, VertexOutputType};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FaceKind(Vector4<u16>);

#[allow(non_upper_case_globals)]
impl FaceKind {
    pub const BlockPosX: FaceKind = FaceKind(Vector4::new(
        0x888, // 1 1 1 LT
        0x808, // 1 0 1 LB
        0x880, // 1 1 0 RT
        0x800, // 1 0 0 RB
    ));
    pub const BlockNegX: FaceKind = FaceKind(Vector4::new(
        0x080, // 0 1 0 LT
        0x000, // 0 0 0 LB
        0x088, // 0 1 1 RT
        0x008, // 0 0 1 RB
    ));
    pub const BlockPosY: FaceKind = FaceKind(Vector4::new(
        0x880, // 1 1 0 LT
        0x080, // 0 1 0 LB
        0x888, // 1 1 1 RT
        0x088, // 0 1 1 RB
    ));
    pub const BlockNegY: FaceKind = FaceKind(Vector4::new(
        0x000, // 0 0 0 LT
        0x800, // 1 0 0 LB
        0x008, // 0 0 1 RT
        0x808, // 1 0 1 RB
    ));
    pub const BlockPosZ: FaceKind = FaceKind(Vector4::new(
        0x088, // 0 1 1 LT
        0x008, // 0 0 1 LB
        0x888, // 1 1 1 RT
        0x808, // 1 0 1 RB
    ));
    pub const BlockNegZ: FaceKind = FaceKind(Vector4::new(
        0x880, // 1 1 0 LT
        0x800, // 1 0 0 LB
        0x080, // 0 1 0 RT
        0x000, // 0 0 0 RB
    ));
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Face {
    pub pos: Vector3<f32>,
    pub kind: FaceKind,
    pub texture: TextureSection,
}

impl VertexDescriptor for Face {
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
        VertexAttribute::new(4, VertexInputType::U16, VertexOutputType::I32),
        VertexAttribute::new(4, VertexInputType::U16, VertexOutputType::NormalizedF32),
    ];
}

impl Default for Face {
    fn default() -> Face {
        Face {
            pos: Vector3::zero(),
            kind: FaceKind::BlockPosZ,
            texture: TextureSection::default(),
        }
    }
}
