#![feature(test, asm)]
#![allow(dead_code)]

extern crate cgmath;
extern crate core;
extern crate parking_lot;
extern crate storm;
extern crate test;

mod utility;

use cgmath::Vector2;
use storm::math::*;
use test::Bencher;
use utility::benching::black_box;

#[bench]
fn bench_slide(b: &mut Bencher) {
    let v = black_box(vec![
        AABB2D::new(2f32, 0f32, 3f32, 1f32),
        AABB2D::new(0f32, 1f32, 1f32, 2f32),
        AABB2D::new(3f32, 1f32, 4f32, 2f32),
        AABB2D::new(1f32, 2f32, 2f32, 3f32),
        AABB2D::new(2f32, 0f32, 3f32, 1f32),
        AABB2D::new(0f32, 1f32, 1f32, 2f32),
        AABB2D::new(3f32, 1f32, 4f32, 2f32),
        AABB2D::new(1f32, 2f32, 2f32, 3f32),
    ]);
    let mota = black_box(Vector2::new(2f32, 0f32));
    let motb = black_box(Vector2::new(-4f32, 1f32));
    b.iter(|| {
        let mut aabb = black_box(AABB2D::new(0f32, 0f32, 1f32, 1f32));
        aabb.slide(&mota, &v);
        aabb.slide(&motb, &v);
    });
}
