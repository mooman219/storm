use crate::graphics::std140::*;
use ::cgmath::{Matrix2, Matrix3, Matrix4, Vector1, Vector2, Vector3, Vector4};
use ::core::mem::transmute;

// float ========================================

impl IntoStd140 for Vector3<f32> {
    type Output = vec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl IntoStd140 for Vector4<f32> {
    type Output = vec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl From<Vector1<f32>> for float {
    fn from(v: Vector1<f32>) -> Self {
        unsafe { transmute(v) }
    }
}
impl From<Vector2<f32>> for vec2 {
    fn from(v: Vector2<f32>) -> Self {
        unsafe { transmute(v) }
    }
}

impl From<Vector3<f32>> for vec3 {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector1<f32>> for vec4 {
    fn from(v: Vector1<f32>) -> Self {
        Self {
            x: v.x,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<Vector2<f32>> for vec4 {
    fn from(v: Vector2<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<Vector3<f32>> for vec4 {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
}

impl From<Vector4<f32>> for vec4 {
    fn from(v: Vector4<f32>) -> Self {
        unsafe { transmute(v) }
    }
}

// int ==========================================

impl IntoStd140 for Vector3<i32> {
    type Output = ivec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl IntoStd140 for Vector4<i32> {
    type Output = ivec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl From<Vector1<i32>> for int {
    fn from(v: Vector1<i32>) -> Self {
        unsafe { transmute(v) }
    }
}

impl From<Vector2<i32>> for ivec2 {
    fn from(v: Vector2<i32>) -> Self {
        unsafe { transmute(v) }
    }
}

impl From<Vector3<i32>> for ivec3 {
    fn from(v: Vector3<i32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector4<i32>> for ivec4 {
    fn from(v: Vector4<i32>) -> Self {
        unsafe { transmute(v) }
    }
}

// uint ==========================================

impl IntoStd140 for Vector3<u32> {
    type Output = uvec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl IntoStd140 for Vector4<u32> {
    type Output = uvec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl From<Vector1<u32>> for int {
    fn from(v: Vector1<u32>) -> Self {
        unsafe { transmute(v) }
    }
}

impl From<Vector2<u32>> for uvec2 {
    fn from(v: Vector2<u32>) -> Self {
        unsafe { transmute(v) }
    }
}

impl From<Vector3<u32>> for uvec3 {
    fn from(v: Vector3<u32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector4<u32>> for uvec4 {
    fn from(v: Vector4<u32>) -> Self {
        unsafe { transmute(v) }
    }
}

// boolean ======================================

impl IntoStd140 for Vector3<bool> {
    type Output = bvec3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
        }
    }
}

impl IntoStd140 for Vector4<bool> {
    type Output = bvec4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
            w: self.w.into(),
        }
    }
}

impl From<Vector1<bool>> for boolean {
    fn from(v: Vector1<bool>) -> Self {
        match v.x {
            true => boolean::True,
            false => boolean::False,
        }
    }
}

impl From<Vector2<bool>> for bvec2 {
    fn from(v: Vector2<bool>) -> Self {
        Self {
            x: v.x.into(),
            y: v.y.into(),
        }
    }
}

impl From<Vector3<bool>> for bvec3 {
    fn from(v: Vector3<bool>) -> Self {
        Self {
            x: v.x.into(),
            y: v.y.into(),
            z: v.z.into(),
        }
    }
}

impl From<Vector4<bool>> for bvec4 {
    fn from(v: Vector4<bool>) -> Self {
        Self {
            x: v.x.into(),
            y: v.y.into(),
            z: v.z.into(),
            w: v.w.into(),
        }
    }
}

// matn =========================================

impl IntoStd140 for Matrix2<f32> {
    type Output = mat2;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl IntoStd140 for Matrix3<f32> {
    type Output = mat3;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
        }
    }
}

impl IntoStd140 for Matrix4<f32> {
    type Output = mat4;
    fn std140(self) -> Self::Output {
        Self::Output {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
            w: self.w.into(),
        }
    }
}

impl From<Matrix2<f32>> for mat2 {
    fn from(v: Matrix2<f32>) -> Self {
        Self {
            x: v.x.into(),
            y: v.y.into(),
        }
    }
}

impl From<Matrix3<f32>> for mat3 {
    fn from(v: Matrix3<f32>) -> Self {
        Self {
            x: v.x.into(),
            y: v.y.into(),
            z: v.z.into(),
        }
    }
}

impl From<Matrix4<f32>> for mat4 {
    fn from(v: Matrix4<f32>) -> Self {
        unsafe { transmute(v) }
    }
}
