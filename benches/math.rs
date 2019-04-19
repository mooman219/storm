#![feature(test, asm)]
#![allow(dead_code)]

extern crate cgmath;
extern crate core;
extern crate parking_lot;
extern crate storm;
extern crate test;

mod utility;

use storm::math::*;
use test::Bencher;
use utility::benching::black_box;

const MIN_ATAN2: f32 = -3f32;
const MAX_ATAN2: f32 = 3f32;
const INC_ATAN2: f32 = 0.01f32;

#[bench]
fn bench_atan2(b: &mut Bencher) {
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
fn bench_atan2_default(b: &mut Bencher) {
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

const SIN_START: f32 = -5f32;
const SIN_INCREMENT: f32 = 0.01f32;
const SIN_ITERATIONS: usize = 1000;

#[bench]
fn bench_sin(b: &mut Bencher) {
    b.iter(|| {
        let mut x = SIN_START;
        for _ in 0..SIN_ITERATIONS {
            black_box(sin_rad(x));
            x += SIN_INCREMENT;
        }
    });
}

#[bench]
fn bench_sin_default(b: &mut Bencher) {
    b.iter(|| {
        let mut x = SIN_START;
        for _ in 0..SIN_ITERATIONS {
            black_box(x.sin());
            x += SIN_INCREMENT;
        }
    });
}
