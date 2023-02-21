use crate::graphics::std140::*;

// float ========================================

impl IntoStd140 for [f32; 3] {
    type Output = vec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0],
            y: self[1],
            z: self[2],
        }
    }
}

impl IntoStd140 for [f32; 4] {
    type Output = vec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0],
            y: self[1],
            z: self[2],
            w: self[3],
        }
    }
}

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

impl IntoStd140 for [i32; 3] {
    type Output = ivec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0],
            y: self[1],
            z: self[2],
        }
    }
}

impl IntoStd140 for [i32; 4] {
    type Output = ivec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0],
            y: self[1],
            z: self[2],
            w: self[3],
        }
    }
}

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

impl IntoStd140 for [u32; 3] {
    type Output = uvec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0],
            y: self[1],
            z: self[2],
        }
    }
}

impl IntoStd140 for [u32; 4] {
    type Output = uvec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0],
            y: self[1],
            z: self[2],
            w: self[3],
        }
    }
}

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

impl IntoStd140 for [bool; 3] {
    type Output = bvec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0].into(),
            y: self[1].into(),
            z: self[2].into(),
        }
    }
}

impl IntoStd140 for [bool; 4] {
    type Output = bvec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self[0].into(),
            y: self[1].into(),
            z: self[2].into(),
            w: self[3].into(),
        }
    }
}

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
