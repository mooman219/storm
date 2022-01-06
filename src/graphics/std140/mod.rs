mod cgmath;
mod core;

/// Macro which configures a structure to act as a std140 uniform for use in shaders.
///
/// # Example
/// ```
/// // The macro relies on the `std140` module being available on the path.
/// use storm::graphics::std140;
///
/// #[std140::uniform]
/// #[derive(Copy, Clone)]
/// pub struct SpriteUniform {
///     pub ortho: std140::mat4,
/// }
/// ```
pub use storm_macro::uniform;

/// Marker trait for element types supported by the `#[uniform]` macro. These types have specific
/// safety, padding, and alignment requirements.
pub unsafe trait Std140Element: Copy {}

/// Marker trait for structs supported by the `#[uniform]` macro. These types have specific safety,
/// padding, and alignment requirements.
pub unsafe trait Std140Struct: Copy {}

// float ========================================

/// A 32-bit floating point value.
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct float {
    pub x: f32,
}

unsafe impl Std140Element for float {}
impl float {
    /// Creates a new [float] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [float] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: v,
        }
    }
}

/// A column vector of 2 float values.
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec2 {
    pub x: f32,
    pub y: f32,
}

unsafe impl Std140Element for vec2 {}
impl vec2 {
    /// Creates a new [vec2] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [vec2] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: v,
            y: v,
        }
    }
}

/// A column vector of 3 float values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl Std140Element for vec3 {}
impl vec3 {
    /// Creates a new [vec3] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [vec3] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
        }
    }
}

/// A column vector of 4 float values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

unsafe impl Std140Element for vec4 {}
impl vec4 {
    /// Creates a new [vec4] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [vec4] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
}

// int ==========================================

/// A 32-bit signed integer value.
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct int {
    pub x: i32,
}

unsafe impl Std140Element for int {}
impl int {
    /// Creates a new [int] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [int] with the given value in all positions.
    pub const fn fill(v: i32) -> Self {
        Self {
            x: v,
        }
    }
}

/// A column vector of 2 int values.
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec2 {
    pub x: i32,
    pub y: i32,
}

unsafe impl Std140Element for ivec2 {}
impl ivec2 {
    /// Creates a new [ivec2] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [ivec2] with the given value in all positions.
    pub const fn fill(v: i32) -> Self {
        Self {
            x: v,
            y: v,
        }
    }
}

/// A column vector of 3 int values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

unsafe impl Std140Element for ivec3 {}
impl ivec3 {
    /// Creates a new [ivec3] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [ivec3] with the given value in all positions.
    pub const fn fill(v: i32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
        }
    }
}

/// A column vector of 4 int values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

unsafe impl Std140Element for ivec4 {}
impl ivec4 {
    /// Creates a new [ivec4] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [ivec4] with the given value in all positions.
    pub const fn fill(v: i32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
}

// uint =========================================

/// A 32-bit unsigned integer value.
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uint {
    pub x: u32,
}

unsafe impl Std140Element for uint {}
impl uint {
    /// Creates a new [uint] with zeros in all positions.
    pub const fn zero() -> Self {
        Self {
            x: 0,
        }
    }

    /// Creates a new [uint] with the given value in all positions.
    pub const fn fill(v: u32) -> Self {
        Self {
            x: v,
        }
    }
}

/// A column vector of 2 uint values.
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec2 {
    pub x: u32,
    pub y: u32,
}

unsafe impl Std140Element for uvec2 {}
impl uvec2 {
    /// Creates a new [uvec2] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [uvec2] with the given value in all positions.
    pub const fn fill(v: u32) -> Self {
        Self {
            x: v,
            y: v,
        }
    }
}

/// A column vector of 3 uint values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

unsafe impl Std140Element for uvec3 {}
impl uvec3 {
    /// Creates a new [uvec3] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [uvec3] with the given value in all positions.
    pub const fn fill(v: u32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
        }
    }
}

/// A column vector of 4 uint values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

unsafe impl Std140Element for uvec4 {}
impl uvec4 {
    /// Creates a new [uvec4] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0)
    }

    /// Creates a new [uvec4] with the given value in all positions.
    pub const fn fill(v: u32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
}

// boolean ======================================

/// A 32-bit boolean value.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum boolean {
    True,
    False,
}

/// A column vector of 2 boolean values.
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec2 {
    pub x: boolean,
    pub y: boolean,
}

unsafe impl Std140Element for bvec2 {}
impl bvec2 {
    /// Creates a new [bvec2] with false in all positions.
    pub const fn falsey() -> Self {
        Self::fill(boolean::False)
    }

    /// Creates a new [bvec2] with true in all positions.
    pub const fn truthy() -> Self {
        Self::fill(boolean::True)
    }

    /// Creates a new [bvec2] with the given value in all positions.
    pub const fn fill(v: boolean) -> Self {
        Self {
            x: v,
            y: v,
        }
    }
}

/// A column vector of 3 boolean values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec3 {
    pub x: boolean,
    pub y: boolean,
    pub z: boolean,
}

unsafe impl Std140Element for bvec3 {}
impl bvec3 {
    /// Creates a new [bvec3] with false in all positions.
    pub const fn falsey() -> Self {
        Self::fill(boolean::False)
    }

    /// Creates a new [bvec3] with true in all positions.
    pub const fn truthy() -> Self {
        Self::fill(boolean::True)
    }

    /// Creates a new [bvec3] with the given value in all positions.
    pub const fn fill(v: boolean) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
        }
    }
}

/// A column vector of 4 boolean values.
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec4 {
    pub x: boolean,
    pub y: boolean,
    pub z: boolean,
    pub w: boolean,
}

unsafe impl Std140Element for bvec4 {}
impl bvec4 {
    /// Creates a new [bvec4] with false in all positions.
    pub const fn falsey() -> Self {
        Self::fill(boolean::False)
    }

    /// Creates a new [bvec4] with true in all positions.
    pub const fn truthy() -> Self {
        Self::fill(boolean::True)
    }

    /// Creates a new [bvec4] with the given value in all positions.
    pub const fn fill(v: boolean) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
}

// matn =========================================

/// A matrix with 2 columns and up to 2 rows, represented by 2 vec4 vectors.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct mat2 {
    pub x: vec4,
    pub y: vec4,
}

unsafe impl Std140Element for mat2 {}
impl mat2 {
    /// Creates a new [mat2] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [mat2] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: vec4::fill(v),
            y: vec4::fill(v),
        }
    }
}

/// A matrix with 3 columns and up to 3 rows, represented by 3 vec4 vectors.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct mat3 {
    pub x: vec4,
    pub y: vec4,
    pub z: vec4,
}

unsafe impl Std140Element for mat3 {}
impl mat3 {
    /// Creates a new [mat3] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [mat3] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: vec4::fill(v),
            y: vec4::fill(v),
            z: vec4::fill(v),
        }
    }
}

/// A matrix with 4 columns and up to 4 rows, represented by 4 vec4 vectors.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct mat4 {
    pub x: vec4,
    pub y: vec4,
    pub z: vec4,
    pub w: vec4,
}

unsafe impl Std140Element for mat4 {}
impl mat4 {
    /// Creates a new [mat4] with zeros in all positions.
    pub const fn zero() -> Self {
        Self::fill(0.0)
    }

    /// Creates a new [mat4] with the given value in all positions.
    pub const fn fill(v: f32) -> Self {
        Self {
            x: vec4::fill(v),
            y: vec4::fill(v),
            z: vec4::fill(v),
            w: vec4::fill(v),
        }
    }
}
