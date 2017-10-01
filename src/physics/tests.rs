use test::Bencher;
use test_utility::black_box;

use cgmath::Vector2;
use physics::aabb::*;

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
