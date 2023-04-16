mod aabb;
mod interpolation;
mod num;
mod orthographic;
mod perspective;

use cgmath::Vector2;

pub use self::aabb::AABB2D;
pub use self::interpolation::Interpolation;
pub use self::num::{Float, UnsignedInteger};
pub use self::orthographic::{ortho_from_bounds, OrthographicCamera};
pub use self::perspective::PerspectiveCamera;

/// Represents 2 * pi.
pub const TAO: f32 = 6.283_185_307_179_586_476f32;
/// Represents pi.
pub const PI: f32 = 3.141_592_653_589_793_238f32;
/// Represents pi / 2.
pub const PI_2: f32 = 1.570_796_326_794_896_619f32;
/// Represents -pi / 2.
pub const PI_NEG_2: f32 = -1.570_796_326_794_896_619f32;
/// Simple const identity matrix.
pub const IDENTITY_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

/// Fast `Vector2<f32>` normalization.
pub fn fast_normalize2(vector: Vector2<f32>) -> Vector2<f32> {
    vector * (vector.x * vector.x + vector.y * vector.y).inv_sqrt()
}

/// Linearly interpolates between a and b by t.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}
