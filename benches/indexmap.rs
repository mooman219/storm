#![feature(test, asm)]
#![allow(dead_code)]

extern crate cgmath;
extern crate core;
extern crate parking_lot;
extern crate storm;
extern crate test;

mod utility;

use test::Bencher;
use utility::benching::black_box;
use utility::indexmap::*;

#[bench]
fn bench_indexmap_cycle(bench: &mut Bencher) {
    let mut map = IndexMap::new();

    bench.iter(|| {
        let a = map.add();
        black_box(map.get(&a));
        black_box(map.remove(&a));
    });
}

#[bench]
fn bench_indexmap_get(bench: &mut Bencher) {
    let mut map = IndexMap::new();
    let a = map.add();

    bench.iter(|| {
        black_box(map.get(&a));
    });
}
