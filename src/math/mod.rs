mod aabb;
mod trigonometry;

use cgmath::*;

pub use self::aabb::*;
pub use self::trigonometry::*;

pub fn ortho_from_bounds(bounds: &Vector2<f32>) -> Matrix4<f32> {
    let w = bounds.x / 2.0;
    let h = bounds.y / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}
