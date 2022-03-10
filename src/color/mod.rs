mod r8;
mod rg8;
mod rgb8;
mod rgba8;

pub use r8::R8;
pub use rg8::RG8;
pub use rgb8::RGB8;
pub use rgba8::RGBA8;

use crate::graphics::{PixelFormat, PixelInternalFormat, PixelType};

/// A trait to describe size and layout of color components.
///
/// # Example
/// ```
/// // This is an example for how to implement ColorDescriptor for a simple type.
/// use storm::color::*;
///
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub struct BGRA8 {
///     pub b: u8,
///     pub r: u8,
///     pub g: u8,
///     pub a: u8,
/// }
///
/// // This allows for bytes to represent single channel resources.
/// impl ColorDescriptor for BGRA8 {
///     fn component_type() -> ColorComponentType {
///         ColorComponentType::U8
///     }
///     fn layout() -> ColorLayoutFormat {
///         ColorLayoutFormat::BGRA
///     }
/// }
/// ```
pub trait ColorDescriptor: Sized + Copy + Default {
    /// Gets the component type of the color.
    fn component_type() -> ColorComponentType;

    /// Gets the layout of the color.
    fn layout() -> ColorLayoutFormat;
}

/// Represents the type of each color component.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ColorComponentType {
    U8 = PixelType::UnsignedByte as u32,
    F32 = PixelType::Float as u32,
}

impl ColorComponentType {
    pub(crate) fn pixel_type(&self) -> PixelType {
        unsafe { core::mem::transmute(*self) }
    }
}

/// Represents the layout of the color components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ColorLayoutFormat {
    R = PixelFormat::RED as u32,
    RG = PixelFormat::RG as u32,
    RGB = PixelFormat::RGB as u32,
    RGBA = PixelFormat::RGBA as u32,
    BGRA = PixelFormat::BGRA as u32,
}

impl ColorLayoutFormat {
    pub(crate) fn gpu_format(&self) -> PixelInternalFormat {
        match self {
            ColorLayoutFormat::R => PixelInternalFormat::R8,
            ColorLayoutFormat::RG => PixelInternalFormat::RG8,
            ColorLayoutFormat::RGB => PixelInternalFormat::RGB8,
            ColorLayoutFormat::RGBA => PixelInternalFormat::RGBA8,
            ColorLayoutFormat::BGRA => PixelInternalFormat::RGBA8,
        }
    }

    pub(crate) fn cpu_format(&self) -> PixelFormat {
        unsafe { core::mem::transmute(*self) }
    }
}

impl ColorDescriptor for u8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::U8
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::R
    }
}
