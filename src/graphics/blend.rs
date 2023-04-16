use crate::graphics::{graphics, BlendEquation, BlendFactor};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendState {
    equation: BlendEquation,
    sfactor: BlendFactor,
    dfactor: BlendFactor,
}

/// Pixel arithmetic description for blending operations. Specifies how incoming RGBA values
/// (source) and the RGBA in framebuffer (destination) are combined. source_color is the new pixel
/// color and destination color is color from the destination buffer.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendMode {
    /// Adds source and destination. Source and destination are multiplied
    /// by blending parameters before addition.
    ///
    /// Example: `SourceColor * BlendFactor1 + DestinationColor * BlendFactor2`
    Add(BlendFactor, BlendFactor),
    /// Subtracts destination from source. Source and destination are
    /// multiplied by blending parameters before subtraction.
    ///
    /// Example: `SourceColor * BlendFactor1 - DestinationColor * BlendFactor2`
    Subtract(BlendFactor, BlendFactor),
    /// Subtracts source from destination. Source and destination are
    /// multiplied by blending parameters before subtraction.
    ///
    /// Example: `DestinationColor * BlendFactor1 - SourceColor * BlendFactor2`
    ReverseSubtract(BlendFactor, BlendFactor),
    /// Selects the minimum between the source and destination. Min does not use the source or
    /// destination factors, only the source and destination colors.
    Min,
    /// Selects the maximum between the source and destination. Max does not use the source or
    /// destination factors, only the source and destination colors.
    Max,
}

impl BlendMode {
    pub(crate) fn apply(&self) {
        let gl = graphics().gl();
        match self {
            BlendMode::Add(src, dst) => {
                gl.blend_equation(BlendEquation::Add);
                gl.blend_func(*src, *dst);
            }
            BlendMode::Subtract(src, dst) => {
                gl.blend_equation(BlendEquation::Subtract);
                gl.blend_func(*src, *dst);
            }
            BlendMode::ReverseSubtract(src, dst) => {
                gl.blend_equation(BlendEquation::ReverseSubtract);
                gl.blend_func(*src, *dst);
            }
            BlendMode::Min => {
                gl.blend_equation(BlendEquation::Min);
            }
            BlendMode::Max => {
                gl.blend_equation(BlendEquation::Max);
            }
        }
    }
}
