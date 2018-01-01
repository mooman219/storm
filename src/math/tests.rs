use test::Bencher;
use utility::benching::black_box;

use math::*;

const MIN_ATAN2: f32 = -3f32;
const MAX_ATAN2: f32 = 3f32;
const INC_ATAN2: f32 = 0.01f32;

#[bench]
fn bench_fast_atan2(b: &mut Bencher) {
    b.iter(|| {
        let mut sum = black_box(0f32);
        let mut x = MIN_ATAN2;
        while x < MAX_ATAN2 {
            let mut y = MIN_ATAN2;
            while y < MAX_ATAN2 {
                sum += atan2(y, x);
                y += INC_ATAN2;
            }
            x += INC_ATAN2;
        }
        black_box(sum)
    });
}

#[bench]
fn bench_default_atan2(b: &mut Bencher) {
    b.iter(|| {
        let mut sum = black_box(0f32);
        let mut x = MIN_ATAN2;
        while x < MAX_ATAN2 {
            let mut y = MIN_ATAN2;
            while y < MAX_ATAN2 {
                sum += y.atan2(x);
                y += INC_ATAN2;
            }
            x += INC_ATAN2;
        }
        black_box(sum)
    });
}

const MIN_SIN: f32 = -9f32;
const MAX_SIN: f32 = 9f32;
const INC_SIN: f32 = 0.01f32;

#[bench]
fn bench_fast_sin(b: &mut Bencher) {
    SIN.init();
    b.iter(|| {
        let mut sum = black_box(0f32);
        let mut x = MIN_SIN;
        while x < MAX_SIN {
            sum += sin_rad(x);
            x += INC_SIN;
        }
        black_box(sum)
    });
}

#[bench]
fn bench_default_sin(b: &mut Bencher) {
    b.iter(|| {
        let mut sum = black_box(0f32);
        let mut x = MIN_SIN;
        while x < MAX_SIN {
            sum += x.sin();
            x += INC_SIN;
        }
        black_box(sum)
    });
}
