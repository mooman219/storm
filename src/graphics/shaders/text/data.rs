use crate::color::RGBA8;
use crate::graphics::{
    TextureSection, VertexAttribute, VertexDescriptor, VertexInputFormat, VertexOutputFormat,
};
use cgmath::{Vector2, Vector3};
use fontdue::layout::TextStyle;

/// Configuration settings for text.
pub struct Text<'a> {
    /// The text to layout.
    pub text: &'a str,
    /// The scale of the text in pixel units. The units of the scale are pixels per Em unit.
    pub px: f32,
    /// The font to layout the text in.
    pub font_index: usize,
    /// The text color,
    pub color: RGBA8,
    /// The depth value used for rendering the text.
    pub depth: f32,
}

impl<'a> Into<TextStyle<'a, TextUserData>> for &Text<'a> {
    fn into(self) -> TextStyle<'a, TextUserData> {
        TextStyle {
            text: self.text,
            px: self.px,
            font_index: self.font_index,
            user_data: TextUserData {
                depth: self.depth,
                color: self.color,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct TextUserData {
    pub color: RGBA8,
    pub depth: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextSprite {
    pub pos: Vector3<f32>,
    pub size: Vector2<u16>,
    pub texture: TextureSection,
    pub color: RGBA8,
}

impl TextSprite {
    pub fn new(pos: Vector3<f32>, size: Vector2<f32>, texture: TextureSection, color: RGBA8) -> TextSprite {
        TextSprite {
            pos,
            size: {
                let x = (size.x as u32) & 0xFFFF;
                let y = (size.y as u32) & 0xFFFF;
                Vector2::new(x as u16, y as u16)
            },
            texture,
            color,
        }
    }
}

impl VertexDescriptor for TextSprite {
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        // Position
        VertexAttribute {
            count: 3,
            normalized: false,
            input: VertexInputFormat::Float,
            output: VertexOutputFormat::Float,
        },
        // Size
        VertexAttribute {
            count: 2,
            normalized: false,
            input: VertexInputFormat::UnsignedShort,
            output: VertexOutputFormat::Float,
        },
        // UV
        VertexAttribute {
            count: 4,
            normalized: true,
            input: VertexInputFormat::UnsignedShort,
            output: VertexOutputFormat::Float,
        },
        // Color: RGBA8
        VertexAttribute {
            count: 4,
            normalized: true,
            input: VertexInputFormat::UnsignedByte,
            output: VertexOutputFormat::Float,
        },
    ];
}
