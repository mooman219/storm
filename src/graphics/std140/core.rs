use crate::graphics::std140::*;

// float ========================================

impl From<f32> for float {
    fn from(v: f32) -> Self {
        Self::fill(v)
    }
}

impl From<f32> for vec2 {
    fn from(v: f32) -> Self {
        Self::fill(v)
    }
}

impl From<f32> for vec3 {
    fn from(v: f32) -> Self {
        Self::fill(v)
    }
}

impl From<f32> for vec4 {
    fn from(v: f32) -> Self {
        Self::fill(v)
    }
}

// int ==========================================

impl From<i32> for int {
    fn from(v: i32) -> Self {
        Self::fill(v)
    }
}

impl From<i32> for ivec2 {
    fn from(v: i32) -> Self {
        Self::fill(v)
    }
}

impl From<i32> for ivec3 {
    fn from(v: i32) -> Self {
        Self::fill(v)
    }
}

impl From<i32> for ivec4 {
    fn from(v: i32) -> Self {
        Self::fill(v)
    }
}

// uint =========================================

impl From<u32> for uint {
    fn from(v: u32) -> Self {
        Self::fill(v)
    }
}

impl From<u32> for uvec2 {
    fn from(v: u32) -> Self {
        Self::fill(v)
    }
}

impl From<u32> for uvec3 {
    fn from(v: u32) -> Self {
        Self::fill(v)
    }
}

impl From<u32> for uvec4 {
    fn from(v: u32) -> Self {
        Self::fill(v)
    }
}

// boolean ======================================

impl From<bool> for boolean {
    fn from(x: bool) -> Self {
        match x {
            true => boolean::True,
            false => boolean::False,
        }
    }
}

impl From<bool> for bvec2 {
    fn from(v: bool) -> Self {
        Self::fill(v.into())
    }
}

impl From<bool> for bvec3 {
    fn from(v: bool) -> Self {
        Self::fill(v.into())
    }
}

impl From<bool> for bvec4 {
    fn from(v: bool) -> Self {
        Self::fill(v.into())
    }
}
