#![feature(test)]

extern crate cgmath;
extern crate storm;
extern crate test;

use cgmath::Vector2;
use storm::physics::aabb::*;

#[test]
fn test_slide() {
    let v = vec![
        AABB2D::new(2f32, 0f32, 3f32, 1f32),
        AABB2D::new(0f32, 1f32, 1f32, 2f32),
    ];
    let mut aabb = AABB2D::new(0f32, 0f32, 1f32, 1f32);

    {
        let mot = Vector2::new(2f32, 0f32);
        aabb.slide(&mot, &v);
        assert_eq!(aabb, AABB2D::new(1f32, 0f32, 2f32, 1f32));
    }

    {
        let mot = Vector2::new(-4f32, 1f32);
        aabb.slide(&mot, &v);
        assert_eq!(aabb, AABB2D::new(1f32, 1f32, 2f32, 2f32));
    }
}
