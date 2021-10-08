mod r8;
mod rg8;
mod rgb8;
mod rgba8;

pub use r8::R8;
pub use rg8::RG8;
pub use rgb8::RGB8;
pub use rgba8::RGBA8;

use crate::render::raw::{PixelFormat, PixelInternalFormat, PixelType};

/// A trait to describe size and layout of color components.
pub trait ColorDescription: Sized + Copy {
    /// Gets the component type of the color.
    fn component_type() -> ColorComponentType;

    /// Gets the layout of the color.
    fn layout() -> ColorLayoutFormat;
}

/// Represents the type of each color component.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorComponentType {
    UnsignedByte,
}

impl ColorComponentType {
    pub(crate) fn pixel_type(&self) -> PixelType {
        match self {
            ColorComponentType::UnsignedByte => PixelType::UnsignedByte,
        }
    }
}

/// Represents the layout of the color components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorLayoutFormat {
    R,
    RG,
    RGB,
    RGBA,
}

impl ColorLayoutFormat {
    pub(crate) fn gpu_format(&self) -> PixelInternalFormat {
        match self {
            ColorLayoutFormat::R => PixelInternalFormat::R8,
            ColorLayoutFormat::RG => PixelInternalFormat::RG8,
            ColorLayoutFormat::RGB => PixelInternalFormat::RGB8,
            ColorLayoutFormat::RGBA => PixelInternalFormat::RGBA8,
        }
    }

    pub(crate) fn cpu_format(&self) -> PixelFormat {
        match self {
            ColorLayoutFormat::R => PixelFormat::RED,
            ColorLayoutFormat::RG => PixelFormat::RG,
            ColorLayoutFormat::RGB => PixelFormat::RGB,
            ColorLayoutFormat::RGBA => PixelFormat::RGBA,
        }
    }
}

impl ColorDescription for u8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::UnsignedByte
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::R
    }
}
