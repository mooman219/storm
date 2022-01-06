use crate::graphics::std140::*;
use ::cgmath::{Matrix2, Matrix3, Matrix4, Vector1, Vector2, Vector3, Vector4};
use ::core::mem::transmute;

// float ========================================

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

// boolean ======================================

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
